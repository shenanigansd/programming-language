use std::collections::HashMap;
use std::path::PathBuf;

use amarok_syntax::{Diagnostic, Program, SourceMap};

use crate::control_flow::ControlFlow;
use crate::eval::statements::execute_statement_list;
use crate::function::{BuiltinFunction, Function};
use crate::module_loader::ModuleLoader;
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
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}
