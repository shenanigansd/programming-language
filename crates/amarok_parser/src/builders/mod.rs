use amarok_syntax::{Diagnostic, Program, Spanned, Statement};
use pest::iterators::Pair;

use crate::grammar::Rule;

pub(crate) mod expressions;
pub(crate) mod helpers;
pub(crate) mod statements;

pub(crate) use expressions::build_expression;

use helpers::span_of;
use statements::{
    build_assignment_statement, build_block_statement, build_expression_statement,
    build_function_definition, build_if_statement, build_return_statement, build_use_statement,
    build_while_statement,
};

pub(crate) fn build_program(pair: Pair<Rule>) -> Result<Program, Diagnostic> {
    if pair.as_rule() != Rule::program {
        return Err(
            Diagnostic::new(format!("Expected program rule, got {:?}", pair.as_rule()))
                .with_span(span_of(&pair)),
        );
    }

    let mut statements: Vec<Spanned<Statement>> = Vec::new();

    for item in pair.into_inner() {
        // Pest may include markers like EOI under program, depending on how the grammar is structured.
        // We only accept actual statement rules here.
        match item.as_rule() {
            Rule::assignment_statement
            | Rule::return_statement
            | Rule::if_statement
            | Rule::while_statement
            | Rule::function_definition
            | Rule::block_statement
            | Rule::use_statement
            | Rule::expression_statement => statements.push(build_statement(item)?),

            // Ignore anything else (EOI, or future wrapper rules).
            _ => {}
        }
    }

    Ok(Program { statements })
}

pub(crate) fn build_statement(pair: Pair<Rule>) -> Result<Spanned<Statement>, Diagnostic> {
    let statement_span = span_of(&pair);

    let statement_value = match pair.as_rule() {
        Rule::assignment_statement => build_assignment_statement(pair)?,
        Rule::expression_statement => build_expression_statement(pair)?,
        Rule::block_statement => build_block_statement(pair)?,
        Rule::if_statement => build_if_statement(pair)?,
        Rule::while_statement => build_while_statement(pair)?,
        Rule::function_definition => build_function_definition(pair)?,
        Rule::return_statement => build_return_statement(pair)?,
        Rule::use_statement => build_use_statement(pair)?,
        other => {
            return Err(
                Diagnostic::new(format!("Unhandled statement rule: {other:?}"))
                    .with_span(statement_span),
            );
        }
    };

    Ok(Spanned::new(statement_span, statement_value))
}
