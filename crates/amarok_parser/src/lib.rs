//! Amarok parser: source text → AST.
//!
//! Errors are reported via [`amarok_syntax::Diagnostic`], shared with the
//! interpreter so a single rendering routine handles both phases.
//!
//! Internals:
//! - [`grammar`] — pest-derived `AmarokGrammar`, `Rule`, and pest-error adapter
//! - [`builders`] — Pair → AST conversion (statements, expressions, helpers)

use amarok_syntax::{Diagnostic, Expression, Program, Spanned, Statement};
use pest::Parser;

mod builders;
mod grammar;

use builders::{build_expression, build_program, build_statement};
use grammar::{AmarokGrammar, Rule, pest_error_to_diagnostic};

/// Parse a full Amarok program (multiple statements).
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
