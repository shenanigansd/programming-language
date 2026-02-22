use amarok_syntax::{BinaryOperator, Expression, Program, Span, Spanned, Statement};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i64),
    String(String),
    Null,
}

#[derive(Debug, Clone)]
pub struct RuntimeError {
    pub message: String,
    pub span: Option<Span>,
}

impl RuntimeError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            span: None,
        }
    }

    pub fn with_span(mut self, span: Span) -> Self {
        self.span = Some(span);
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
enum ControlFlow {
    Continue,
    Return(Value),
}

type BuiltinFunction = fn(&mut Interpreter, Vec<Value>, Span) -> Result<Value, RuntimeError>;

#[derive(Clone)]
enum Function {
    UserDefined {
        parameters: Vec<String>,
        body: Vec<Spanned<Statement>>,
    },
}

pub struct Interpreter {
    scopes: Vec<HashMap<String, Value>>,
    functions: HashMap<String, Function>,
    builtins: HashMap<String, BuiltinFunction>,
    output: Vec<String>,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut interpreter = Self {
            scopes: vec![HashMap::new()],
            functions: HashMap::new(),
            builtins: HashMap::new(),
            output: Vec::new(),
        };

        interpreter.install_builtins();
        interpreter
    }

    pub fn output_lines(&self) -> &[String] {
        &self.output
    }

    pub fn run_program(&mut self, program: &Program) -> Result<(), RuntimeError> {
        match self.execute_statement_list(&program.statements)? {
            ControlFlow::Continue => Ok(()),
            ControlFlow::Return(_) => Err(RuntimeError::new("Return outside of function.")),
        }
    }

    fn install_builtins(&mut self) {
        self.builtins.insert("print".to_string(), builtin_print);
    }

    fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn exit_scope(&mut self) {
        self.scopes.pop();
        if self.scopes.is_empty() {
            self.scopes.push(HashMap::new());
        }
    }

    fn assign_variable(&mut self, name: &str, value: Value) {
        // For now: assign into the current (innermost) scope.
        // Later you can change this to update an existing variable in an outer scope if found.
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.to_string(), value);
        }
    }

    fn read_variable(&self, name: &str, span: Span) -> Result<Value, RuntimeError> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Ok(value.clone());
            }
        }
        Err(RuntimeError::new(format!("Undefined variable: {name}")).with_span(span))
    }

    fn execute_statement_list(
        &mut self,
        statements: &[Spanned<Statement>],
    ) -> Result<ControlFlow, RuntimeError> {
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
    ) -> Result<ControlFlow, RuntimeError> {
        match &statement.value {
            Statement::Assignment { name, value } => {
                let evaluated = self.evaluate_expression(value)?;
                self.assign_variable(name, evaluated);
                Ok(ControlFlow::Continue)
            }

            Statement::Expression { expression } => {
                let _ = self.evaluate_expression(expression)?;
                Ok(ControlFlow::Continue)
            }

            Statement::Block { statements } => {
                self.enter_scope();
                let result = self.execute_statement_list(statements);
                self.exit_scope();
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

    fn evaluate_expression(&mut self, expression: &Spanned<Expression>) -> Result<Value, RuntimeError> {
        match &expression.value {
            Expression::Integer(value) => Ok(Value::Integer(*value)),

            Expression::String(value) => Ok(Value::String(value.clone())),

            Expression::Variable(name) => self.read_variable(name, expression.span),

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
    ) -> Result<Value, RuntimeError> {
        if let Some(builtin) = self.builtins.get(name).copied() {
            return builtin(self, arguments, call_span);
        }

        let Some(function) = self.functions.get(name).cloned() else {
            return Err(RuntimeError::new(format!("Undefined function: {name}")).with_span(call_span));
        };

        match function {
            Function::UserDefined { parameters, body } => {
                if arguments.len() != parameters.len() {
                    return Err(RuntimeError::new(format!(
                        "Function {name} expected {} arguments, got {}",
                        parameters.len(),
                        arguments.len()
                    ))
                    .with_span(call_span));
                }

                self.enter_scope();
                for (parameter, argument_value) in parameters.iter().zip(arguments.into_iter()) {
                    self.assign_variable(parameter, argument_value);
                }

                let result = self.execute_statement_list(&body);
                self.exit_scope();

                match result? {
                    ControlFlow::Continue => Ok(Value::Null),
                    ControlFlow::Return(value) => Ok(value),
                }
            }
        }
    }
}

fn evaluate_binary(
    operator: BinaryOperator,
    left: Value,
    right: Value,
    span: Span,
) -> Result<Value, RuntimeError> {
    match (operator, left, right) {
        (BinaryOperator::Add, Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a + b)),
        (BinaryOperator::Subtract, Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a - b)),
        (BinaryOperator::Multiply, Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a * b)),
        (BinaryOperator::Divide, Value::Integer(a), Value::Integer(b)) => {
            if b == 0 {
                Err(RuntimeError::new("Division by zero.").with_span(span))
            } else {
                Ok(Value::Integer(a / b))
            }
        }

        // Convenience: string concatenation for "+"
        (BinaryOperator::Add, Value::String(a), Value::String(b)) => Ok(Value::String(format!("{a}{b}"))),

        (op, a, b) => Err(RuntimeError::new(format!("Unsupported operation: {a:?} {op} {b:?}")).with_span(span)),
    }
}

fn is_truthy(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::Integer(v) => *v != 0,
        Value::String(s) => !s.is_empty(),
    }
}

fn builtin_print(
    interpreter: &mut Interpreter,
    arguments: Vec<Value>,
    _call_span: Span,
) -> Result<Value, RuntimeError> {
    let mut pieces = Vec::new();
    for value in arguments {
        pieces.push(format_value(&value));
    }
    interpreter.output.push(pieces.join(" "));
    Ok(Value::Null)
}

fn format_value(value: &Value) -> String {
    match value {
        Value::Integer(v) => v.to_string(),
        Value::String(s) => s.clone(),
        Value::Null => "null".to_string(),
    }
}