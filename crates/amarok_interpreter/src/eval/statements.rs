use amarok_syntax::{Diagnostic, Spanned, Statement};

use crate::control_flow::ControlFlow;
use crate::eval::expressions::evaluate_expression;
use crate::function::Function;
use crate::interpreter::Interpreter;
use crate::value::{Value, is_truthy};

pub(crate) fn execute_statement_list(
    interp: &mut Interpreter,
    statements: &[Spanned<Statement>],
) -> Result<ControlFlow, Diagnostic> {
    for statement in statements {
        match execute_statement(interp, statement)? {
            ControlFlow::Continue => {}
            ControlFlow::Return(value) => return Ok(ControlFlow::Return(value)),
        }
    }
    Ok(ControlFlow::Continue)
}

pub(crate) fn execute_statement(
    interp: &mut Interpreter,
    statement: &Spanned<Statement>,
) -> Result<ControlFlow, Diagnostic> {
    match &statement.value {
        Statement::Assignment { name, value } => {
            let evaluated = evaluate_expression(interp, value)?;
            interp.scopes.set_innermost(name, evaluated);
            Ok(ControlFlow::Continue)
        }

        Statement::Expression { expression } => {
            let _ = evaluate_expression(interp, expression)?;
            Ok(ControlFlow::Continue)
        }

        Statement::Block { statements } => {
            interp.scopes.push();
            let result = execute_statement_list(interp, statements);
            interp.scopes.pop();
            result
        }

        Statement::If {
            condition,
            then_branch,
            else_branch,
        } => {
            let condition_value = evaluate_expression(interp, condition)?;
            if is_truthy(&condition_value) {
                execute_statement_list(interp, then_branch)
            } else {
                execute_statement_list(interp, else_branch)
            }
        }

        Statement::While { condition, body } => {
            loop {
                let condition_value = evaluate_expression(interp, condition)?;
                if !is_truthy(&condition_value) {
                    break;
                }

                match execute_statement_list(interp, body)? {
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
            interp.functions.insert(
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
                Some(expression) => evaluate_expression(interp, expression)?,
                None => Value::Null,
            };
            Ok(ControlFlow::Return(return_value))
        }

        Statement::Use { path } => {
            interp.load_and_merge_module(path, statement.span)?;
            Ok(ControlFlow::Continue)
        }
    }
}
