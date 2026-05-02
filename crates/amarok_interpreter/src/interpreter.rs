use std::collections::HashMap;

use amarok_syntax::{BinaryOperator, Diagnostic, Expression, Program, Span, Spanned, Statement};

use crate::builtins::register_builtins;
use crate::control_flow::ControlFlow;
use crate::function::{BuiltinFunction, Function};
use crate::scope::ScopeStack;
use crate::value::{Value, is_truthy};

pub struct Interpreter {
    scopes: ScopeStack,
    functions: HashMap<String, Function>,
    builtins: HashMap<String, BuiltinFunction>,
    output: Vec<String>,
}

impl Interpreter {
    #[must_use]
    pub fn new() -> Self {
        let mut interpreter = Self {
            scopes: ScopeStack::new(),
            functions: HashMap::new(),
            builtins: HashMap::new(),
            output: Vec::new(),
        };

        register_builtins(&mut interpreter);
        interpreter
    }

    #[must_use]
    pub fn output_lines(&self) -> &[String] {
        &self.output
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

    // --- internal accessors used by builtins ---

    pub(crate) fn builtins_mut(&mut self) -> &mut HashMap<String, BuiltinFunction> {
        &mut self.builtins
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

            Expression::FunctionCall { name, arguments } => {
                let mut evaluated_arguments = Vec::with_capacity(arguments.len());
                for argument in arguments {
                    evaluated_arguments.push(self.evaluate_expression(argument)?);
                }
                self.call_function(name, evaluated_arguments, expression.span)
            }
        }
    }

    fn call_function(
        &mut self,
        name: &str,
        arguments: Vec<Value>,
        call_span: Span,
    ) -> Result<Value, Diagnostic> {
        if let Some(builtin) = self.builtins.get(name).copied() {
            return Ok(builtin(self, arguments, call_span));
        }

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
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

fn evaluate_binary(
    operator: BinaryOperator,
    left: Value,
    right: Value,
    span: Span,
) -> Result<Value, Diagnostic> {
    match (operator, left, right) {
        (BinaryOperator::Add, Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a + b)),
        (BinaryOperator::Subtract, Value::Integer(a), Value::Integer(b)) => {
            Ok(Value::Integer(a - b))
        }
        (BinaryOperator::Multiply, Value::Integer(a), Value::Integer(b)) => {
            Ok(Value::Integer(a * b))
        }
        (BinaryOperator::Divide, Value::Integer(a), Value::Integer(b)) => {
            if b == 0 {
                Err(Diagnostic::new("Division by zero.").with_span(span))
            } else {
                Ok(Value::Integer(a / b))
            }
        }

        // Convenience: string concatenation for "+"
        (BinaryOperator::Add, Value::String(a), Value::String(b)) => {
            Ok(Value::String(format!("{a}{b}")))
        }

        (op, a, b) => {
            Err(Diagnostic::new(format!("Unsupported operation: {a:?} {op} {b:?}")).with_span(span))
        }
    }
}
