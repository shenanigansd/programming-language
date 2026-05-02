use amarok_syntax::{Diagnostic, Expression, Span, Spanned};

use crate::control_flow::ControlFlow;
use crate::eval::statements::execute_statement_list;
use crate::function::Function;
use crate::interpreter::Interpreter;
use crate::value::{Value, evaluate_binary};

pub(crate) fn evaluate_expression(
    interp: &mut Interpreter,
    expression: &Spanned<Expression>,
) -> Result<Value, Diagnostic> {
    match &expression.value {
        Expression::Integer(value) => Ok(Value::Integer(*value)),

        Expression::String(value) => Ok(Value::String(value.clone())),

        Expression::Variable(name) => interp.scopes.get(name, expression.span),

        Expression::Binary {
            left,
            operator,
            right,
        } => {
            let left_value = evaluate_expression(interp, left)?;
            let right_value = evaluate_expression(interp, right)?;
            evaluate_binary(*operator, left_value, right_value, expression.span)
        }

        Expression::FunctionCall { path, arguments } => {
            let mut evaluated_arguments = Vec::with_capacity(arguments.len());
            for argument in arguments {
                evaluated_arguments.push(evaluate_expression(interp, argument)?);
            }
            call_function(interp, path, evaluated_arguments, expression.span)
        }
    }
}

pub(crate) fn call_function(
    interp: &mut Interpreter,
    path: &[String],
    arguments: Vec<Value>,
    call_span: Span,
) -> Result<Value, Diagnostic> {
    match path.len() {
        0 => Err(Diagnostic::new("Empty function path.").with_span(call_span)),
        1 => call_unqualified(interp, &path[0], arguments, call_span),
        2 => call_qualified(interp, &path[0], &path[1], arguments, call_span),
        _ => Err(
            Diagnostic::new(format!("Unknown path: {}", path.join("::"))).with_span(call_span),
        ),
    }
}

fn call_qualified(
    interp: &mut Interpreter,
    namespace: &str,
    name: &str,
    arguments: Vec<Value>,
    call_span: Span,
) -> Result<Value, Diagnostic> {
    let Some(builtin) = interp
        .namespaces
        .get(namespace)
        .and_then(|items| items.get(name).copied())
    else {
        return Err(
            Diagnostic::new(format!("Unknown path: {namespace}::{name}")).with_span(call_span),
        );
    };

    Ok(builtin(interp, arguments, call_span))
}

fn call_unqualified(
    interp: &mut Interpreter,
    name: &str,
    arguments: Vec<Value>,
    call_span: Span,
) -> Result<Value, Diagnostic> {
    let Some(function) = interp.functions.get(name).cloned() else {
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

            interp.scopes.push();
            for (parameter, argument_value) in parameters.iter().zip(arguments) {
                interp.scopes.set_innermost(parameter, argument_value);
            }

            let result = execute_statement_list(interp, &body);
            interp.scopes.pop();

            match result? {
                ControlFlow::Continue => Ok(Value::Null),
                ControlFlow::Return(value) => Ok(value),
            }
        }
    }
}
