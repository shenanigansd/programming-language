use std::collections::HashMap;
use std::path::PathBuf;

use amarok_syntax::{Diagnostic, Program, SourceMap, Span};

use crate::control_flow::ControlFlow;
use crate::eval::statements::execute_statement_list;
use crate::function::{BuiltinFunction, Function};
use crate::module_loader::{ModuleExports, ModuleLoader};
use crate::scope::ScopeStack;
use crate::std_lib;

pub struct Interpreter {
    pub(crate) scopes: ScopeStack,
    pub(crate) functions: HashMap<String, Function>,
    pub(crate) namespaces: HashMap<String, HashMap<String, BuiltinFunction>>,
    pub(crate) output: Vec<String>,
    pub(crate) loader: ModuleLoader,
}

impl Interpreter {
    /// Constructs an interpreter with `std::` registered.
    #[must_use]
    pub fn new() -> Self {
        let mut interpreter = Self::empty();
        std_lib::register(&mut interpreter);
        interpreter
    }

    /// Constructs an interpreter with no namespaces registered. Calls into
    /// `std::` will fail with an `Unknown path` diagnostic at runtime.
    #[must_use]
    pub fn new_no_std() -> Self {
        Self::empty()
    }

    fn empty() -> Self {
        Self {
            scopes: ScopeStack::new(),
            functions: HashMap::new(),
            namespaces: HashMap::new(),
            output: Vec::new(),
            loader: ModuleLoader::new(),
        }
    }

    #[must_use]
    pub fn output_lines(&self) -> &[String] {
        &self.output
    }

    /// Add a directory that will be searched (in order) when resolving
    /// `use a::b::c;` statements.
    pub fn add_module_root(&mut self, root: impl Into<PathBuf>) {
        self.loader.add_root(root);
    }

    /// Replace the loader's [`SourceMap`] with one already populated by the
    /// caller (e.g. the CLI registering the entry file before parsing).
    pub fn set_source_map(&mut self, source_map: SourceMap) {
        *self.loader.source_map_mut() = source_map;
    }

    /// Borrow the [`SourceMap`] currently held by the loader. The CLI uses
    /// this to render diagnostics with their originating file path.
    #[must_use]
    pub fn source_map(&self) -> &SourceMap {
        self.loader.source_map()
    }

    /// Executes a parsed program from start to finish.
    ///
    /// # Errors
    ///
    /// Returns an error if runtime evaluation fails, such as undefined names,
    /// invalid operations, or a `return` used outside of a function.
    pub fn run_program(&mut self, program: &Program) -> Result<(), Diagnostic> {
        match execute_statement_list(self, &program.statements)? {
            ControlFlow::Continue => Ok(()),
            ControlFlow::Return(_) => Err(Diagnostic::new("Return outside of function.")),
        }
    }

    pub(crate) fn register_builtin(
        &mut self,
        namespace: &str,
        name: &str,
        function: BuiltinFunction,
    ) {
        self.namespaces
            .entry(namespace.to_string())
            .or_default()
            .insert(name.to_string(), function);
    }

    pub(crate) fn push_output(&mut self, line: String) {
        self.output.push(line);
    }

    pub(crate) fn load_and_merge_module(
        &mut self,
        segments: &[String],
        use_span: Span,
    ) -> Result<(), Diagnostic> {
        // 1. Resolve.
        let canonical = self.loader.resolve(segments).map_err(|tried| {
            let path_text = segments.join("::");
            let tried_text = ModuleLoader::format_tried_paths(&tried);
            Diagnostic::new(format!(
                "Module not found: {path_text}\nTried:\n{tried_text}"
            ))
            .with_span(use_span)
        })?;

        // 2. Cache hit → just merge.
        if let Some(exports) = self.loader.cache.get(&canonical).cloned() {
            self.merge_exports(exports);
            return Ok(());
        }

        // 3. Cycle?
        if self.loader.loading.contains(&canonical) {
            return Err(Diagnostic::new(format!(
                "Circular import detected while loading {}",
                canonical.display()
            ))
            .with_span(use_span));
        }

        // 4. Read & parse.
        let source = std::fs::read_to_string(&canonical).map_err(|error| {
            Diagnostic::new(format!(
                "Failed to read module {}: {}",
                canonical.display(),
                error
            ))
            .with_span(use_span)
        })?;

        let file_id = self
            .loader
            .source_map_mut()
            .add_file(canonical.clone(), source.clone());

        let program =
            amarok_parser::parse_program_with_file_id(&source, file_id).map_err(|diagnostic| {
                let span = diagnostic.span.unwrap_or(use_span);
                Diagnostic::new(format!(
                    "Failed to parse module {}: {}",
                    canonical.display(),
                    diagnostic.message
                ))
                .with_span(span)
            })?;

        // 5. Evaluate in isolated state.
        self.loader.loading.insert(canonical.clone());

        let saved_scopes = std::mem::replace(&mut self.scopes, ScopeStack::new());
        let saved_functions = std::mem::take(&mut self.functions);

        let evaluation = execute_statement_list(self, &program.statements);

        let exports = ModuleExports {
            functions: std::mem::take(&mut self.functions),
            variables: self.scopes.outermost_clone(),
        };

        self.scopes = saved_scopes;
        self.functions = saved_functions;
        self.loader.loading.remove(&canonical);

        match evaluation {
            Ok(ControlFlow::Continue) => {}
            Ok(ControlFlow::Return(_)) => {
                return Err(Diagnostic::new(format!(
                    "Return outside of function in module {}",
                    canonical.display()
                ))
                .with_span(use_span));
            }
            Err(diagnostic) => return Err(diagnostic),
        }

        // 6. Cache and merge.
        self.loader.cache.insert(canonical, exports.clone());
        self.merge_exports(exports);
        Ok(())
    }

    fn merge_exports(&mut self, exports: ModuleExports) {
        for (name, function) in exports.functions {
            self.functions.insert(name, function);
        }
        for (name, value) in exports.variables {
            self.scopes.set_outermost(&name, value);
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}
