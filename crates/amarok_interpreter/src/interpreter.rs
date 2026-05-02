use std::collections::HashMap;
use std::path::PathBuf;

use amarok_syntax::{Diagnostic, Expression, Program, SourceMap, Span, Spanned, Statement};

use crate::control_flow::ControlFlow;
use crate::function::{BuiltinFunction, Function};
use crate::module_loader::{ModuleExports, ModuleLoader};
use crate::scope::ScopeStack;
use crate::std_lib;
use crate::value::{Value, evaluate_binary, is_truthy};

pub struct Interpreter {
    scopes: ScopeStack,
    functions: HashMap<String, Function>,
    namespaces: HashMap<String, HashMap<String, BuiltinFunction>>,
    output: Vec<String>,
    loader: ModuleLoader,
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
        match self.execute_statement_list(&program.statements)? {
            ControlFlow::Continue => Ok(()),
            ControlFlow::Return(_) => Err(Diagnostic::new("Return outside of function.")),
        }
    }

    // --- internal accessors used by namespace registration modules ---

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

    // --- execution ---

    fn execute_statement_list(
        &mut self,
        statements: &[Spanned<Statement>],
    ) -> Result<ControlFlow, Diagnostic> {
        for statement in statements {
            match self.execute_statement(statement)? {
                ControlFlow::Continue => {}
                ControlFlow::Return(value) => return Ok(ControlFlow::Return(value)),
            }
        }
        Ok(ControlFlow::Continue)
    }

    fn execute_statement(
        &mut self,
        statement: &Spanned<Statement>,
    ) -> Result<ControlFlow, Diagnostic> {
        match &statement.value {
            Statement::Assignment { name, value } => {
                let evaluated = self.evaluate_expression(value)?;
                self.scopes.set_innermost(name, evaluated);
                Ok(ControlFlow::Continue)
            }

            Statement::Expression { expression } => {
                let _ = self.evaluate_expression(expression)?;
                Ok(ControlFlow::Continue)
            }

            Statement::Block { statements } => {
                self.scopes.push();
                let result = self.execute_statement_list(statements);
                self.scopes.pop();
                result
            }

            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let condition_value = self.evaluate_expression(condition)?;
                if is_truthy(&condition_value) {
                    self.execute_statement_list(then_branch)
                } else {
                    self.execute_statement_list(else_branch)
                }
            }

            Statement::While { condition, body } => {
                loop {
                    let condition_value = self.evaluate_expression(condition)?;
                    if !is_truthy(&condition_value) {
                        break;
                    }

                    match self.execute_statement_list(body)? {
                        ControlFlow::Continue => {}
                        ControlFlow::Return(value) => return Ok(ControlFlow::Return(value)),
                    }
                }

                Ok(ControlFlow::Continue)
            }

            Statement::FunctionDefinition {
                name,
                parameters,
                body,
            } => {
                self.functions.insert(
                    name.clone(),
                    Function::UserDefined {
                        parameters: parameters.clone(),
                        body: body.clone(),
                    },
                );
                Ok(ControlFlow::Continue)
            }

            Statement::Return { value } => {
                let return_value = match value {
                    Some(expression) => self.evaluate_expression(expression)?,
                    None => Value::Null,
                };
                Ok(ControlFlow::Return(return_value))
            }

            Statement::Use { path } => {
                self.load_and_merge_module(path, statement.span)?;
                Ok(ControlFlow::Continue)
            }
        }
    }

    fn evaluate_expression(
        &mut self,
        expression: &Spanned<Expression>,
    ) -> Result<Value, Diagnostic> {
        match &expression.value {
            Expression::Integer(value) => Ok(Value::Integer(*value)),

            Expression::String(value) => Ok(Value::String(value.clone())),

            Expression::Variable(name) => self.scopes.get(name, expression.span),

            Expression::Binary {
                left,
                operator,
                right,
            } => {
                let left_value = self.evaluate_expression(left)?;
                let right_value = self.evaluate_expression(right)?;
                evaluate_binary(*operator, left_value, right_value, expression.span)
            }

            Expression::FunctionCall { path, arguments } => {
                let mut evaluated_arguments = Vec::with_capacity(arguments.len());
                for argument in arguments {
                    evaluated_arguments.push(self.evaluate_expression(argument)?);
                }
                self.call_function(path, evaluated_arguments, expression.span)
            }
        }
    }

    fn call_function(
        &mut self,
        path: &[String],
        arguments: Vec<Value>,
        call_span: Span,
    ) -> Result<Value, Diagnostic> {
        match path.len() {
            0 => Err(Diagnostic::new("Empty function path.").with_span(call_span)),
            1 => self.call_unqualified(&path[0], arguments, call_span),
            2 => self.call_qualified(&path[0], &path[1], arguments, call_span),
            _ => Err(
                Diagnostic::new(format!("Unknown path: {}", path.join("::"))).with_span(call_span),
            ),
        }
    }

    fn call_qualified(
        &mut self,
        namespace: &str,
        name: &str,
        arguments: Vec<Value>,
        call_span: Span,
    ) -> Result<Value, Diagnostic> {
        let Some(builtin) = self
            .namespaces
            .get(namespace)
            .and_then(|items| items.get(name).copied())
        else {
            return Err(
                Diagnostic::new(format!("Unknown path: {namespace}::{name}")).with_span(call_span),
            );
        };

        Ok(builtin(self, arguments, call_span))
    }

    fn call_unqualified(
        &mut self,
        name: &str,
        arguments: Vec<Value>,
        call_span: Span,
    ) -> Result<Value, Diagnostic> {
        let Some(function) = self.functions.get(name).cloned() else {
            return Err(Diagnostic::new(format!("Undefined function: {name}")).with_span(call_span));
        };

        match function {
            Function::UserDefined { parameters, body } => {
                if arguments.len() != parameters.len() {
                    return Err(Diagnostic::new(format!(
                        "Function {name} expected {} arguments, got {}",
                        parameters.len(),
                        arguments.len()
                    ))
                    .with_span(call_span));
                }

                self.scopes.push();
                for (parameter, argument_value) in parameters.iter().zip(arguments) {
                    self.scopes.set_innermost(parameter, argument_value);
                }

                let result = self.execute_statement_list(&body);
                self.scopes.pop();

                match result? {
                    ControlFlow::Continue => Ok(Value::Null),
                    ControlFlow::Return(value) => Ok(value),
                }
            }
        }
    }

    // --- module loading ---

    fn load_and_merge_module(
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

        let evaluation = self.execute_statement_list(&program.statements);

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
