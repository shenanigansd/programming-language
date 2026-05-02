//! Amarok parser: source text → AST.
//!
//! Errors are reported via [`amarok_syntax::Diagnostic`], shared with the
//! interpreter so a single rendering routine handles both phases.
//!
//! Internals:
//! - [`grammar`] — pest-derived `AmarokGrammar`, `Rule`, and pest-error adapter
//! - [`builders`] — Pair → AST conversion (statements, expressions, helpers)

use amarok_syntax::{Diagnostic, Expression, FileId, Program, Spanned, Statement};
use pest::Parser;

mod builders;
mod grammar;

use builders::{build_expression, build_program, build_statement};
use grammar::{AmarokGrammar, Rule, pest_error_to_diagnostic};

/// Parse a full Amarok program (multiple statements).
///
/// All spans on the resulting AST will use [`FileId::DUMMY`]; for multi-file
/// scenarios use [`parse_program_with_file_id`].
///
/// # Errors
///
/// Returns an error if the source is not valid Amarok syntax for a full program.
pub fn parse_program(source: &str) -> Result<Program, Diagnostic> {
    let mut pairs = AmarokGrammar::parse(Rule::program, source)
        .map_err(|error: pest::error::Error<Rule>| pest_error_to_diagnostic(&error))?;

    let program_pair = pairs
        .next()
        .ok_or_else(|| Diagnostic::new("Expected a program, found nothing."))?;

    build_program(program_pair).map_err(Diagnostic::new)
}

/// Parse a full Amarok program and stamp every span with the given [`FileId`].
///
/// # Errors
///
/// Returns an error if the source is not valid Amarok syntax for a full program.
/// Errors are reported with the given `file_id` attached to their span.
pub fn parse_program_with_file_id(source: &str, file_id: FileId) -> Result<Program, Diagnostic> {
    let mut program = parse_program(source).map_err(|mut diagnostic| {
        if let Some(span) = diagnostic.span.as_mut() {
            span.file_id = file_id;
        }
        diagnostic
    })?;
    set_program_file_id(&mut program, file_id);
    Ok(program)
}

/// Parse a single statement (useful for REPL later).
///
/// # Errors
///
/// Returns an error if the source is not valid Amarok syntax for a single statement.
pub fn parse_statement(source: &str) -> Result<Spanned<Statement>, Diagnostic> {
    let mut pairs = AmarokGrammar::parse(Rule::statement, source)
        .map_err(|error: pest::error::Error<Rule>| pest_error_to_diagnostic(&error))?;

    let statement_pair = pairs
        .next()
        .ok_or_else(|| Diagnostic::new("Expected a statement, found nothing."))?;

    build_statement(statement_pair).map_err(Diagnostic::new)
}

/// Parse a single expression (useful for unit tests and REPL experiments).
///
/// # Errors
///
/// Returns an error if the source is not valid Amarok syntax for a single expression.
pub fn parse_expression(source: &str) -> Result<Spanned<Expression>, Diagnostic> {
    let mut pairs = AmarokGrammar::parse(Rule::expression, source)
        .map_err(|error: pest::error::Error<Rule>| pest_error_to_diagnostic(&error))?;

    let expression_pair = pairs
        .next()
        .ok_or_else(|| Diagnostic::new("Expected an expression, found nothing."))?;

    build_expression(expression_pair).map_err(Diagnostic::new)
}

// --- span rewriting -------------------------------------------------------

fn set_program_file_id(program: &mut Program, file_id: FileId) {
    for statement in &mut program.statements {
        set_statement_file_id(statement, file_id);
    }
}

fn set_statement_file_id(statement: &mut Spanned<Statement>, file_id: FileId) {
    statement.span.file_id = file_id;
    match &mut statement.value {
        Statement::Assignment { value, .. } => set_expression_file_id(value, file_id),
        Statement::Expression { expression } => set_expression_file_id(expression, file_id),
        Statement::Block { statements } => {
            for child in statements {
                set_statement_file_id(child, file_id);
            }
        }
        Statement::If {
            condition,
            then_branch,
            else_branch,
        } => {
            set_expression_file_id(condition, file_id);
            for child in then_branch {
                set_statement_file_id(child, file_id);
            }
            for child in else_branch {
                set_statement_file_id(child, file_id);
            }
        }
        Statement::While { condition, body } => {
            set_expression_file_id(condition, file_id);
            for child in body {
                set_statement_file_id(child, file_id);
            }
        }
        Statement::FunctionDefinition { body, .. } => {
            for child in body {
                set_statement_file_id(child, file_id);
            }
        }
        Statement::Return { value } => {
            if let Some(expression) = value {
                set_expression_file_id(expression, file_id);
            }
        }
        Statement::Use { .. } => {}
    }
}

fn set_expression_file_id(expression: &mut Spanned<Expression>, file_id: FileId) {
    expression.span.file_id = file_id;
    match &mut expression.value {
        Expression::Integer(_) | Expression::String(_) | Expression::Variable(_) => {}
        Expression::Binary { left, right, .. } => {
            set_expression_file_id(left, file_id);
            set_expression_file_id(right, file_id);
        }
        Expression::FunctionCall { arguments, .. } => {
            for argument in arguments {
                set_expression_file_id(argument, file_id);
            }
        }
    }
}
