use amarok_syntax::{Diagnostic, Spanned, Statement};
use pest::iterators::Pair;

use crate::grammar::Rule;

use super::expressions::build_expression;
use super::helpers::{collect_path_segments, find_child, span_of};

pub(crate) fn build_assignment_statement(pair: Pair<Rule>) -> Result<Statement, Diagnostic> {
    // assignment_statement = { identifier ~ "=" ~ expression ~ ";" }
    let outer_span = span_of(&pair);
    let mut inner = pair.into_inner();

    let name_pair = inner
        .next()
        .ok_or_else(|| Diagnostic::new("Assignment missing identifier.").with_span(outer_span))?;
    if name_pair.as_rule() != Rule::identifier {
        return Err(Diagnostic::new(format!(
            "Assignment expected identifier, got {:?}",
            name_pair.as_rule()
        ))
        .with_span(span_of(&name_pair)));
    }
    let name = name_pair.as_str().to_string();

    let expression_pair = inner.find(|p| p.as_rule() == Rule::expression).ok_or_else(|| {
        Diagnostic::new("Assignment missing expression.").with_span(outer_span)
    })?;

    let value = build_expression(expression_pair)?;

    Ok(Statement::Assignment { name, value })
}

pub(crate) fn build_expression_statement(pair: Pair<Rule>) -> Result<Statement, Diagnostic> {
    // expression_statement = { expression ~ ";" }
    let expression_pair = find_child(pair, Rule::expression, "Expression statement")?;

    Ok(Statement::Expression {
        expression: build_expression(expression_pair)?,
    })
}

pub(crate) fn build_block_statement(pair: Pair<Rule>) -> Result<Statement, Diagnostic> {
    // block_statement = { "{" ~ statement* ~ "}" }
    let mut statements: Vec<Spanned<Statement>> = Vec::new();

    for item in pair.into_inner() {
        // statement is silent in the grammar, so these should be concrete statement rules.
        statements.push(super::build_statement(item)?);
    }

    Ok(Statement::Block { statements })
}

pub(crate) fn build_if_statement(pair: Pair<Rule>) -> Result<Statement, Diagnostic> {
    // if_statement = { "if" ~ "(" ~ expression ~ ")" ~ block_statement ~ else_clause? }
    let outer_span = span_of(&pair);
    let mut inner = pair.into_inner();

    let condition_pair = inner.find(|p| p.as_rule() == Rule::expression).ok_or_else(|| {
        Diagnostic::new("If statement missing condition expression.").with_span(outer_span)
    })?;
    let condition = build_expression(condition_pair)?;

    let then_block_pair = inner
        .find(|p| p.as_rule() == Rule::block_statement)
        .ok_or_else(|| {
            Diagnostic::new("If statement missing then block.").with_span(outer_span)
        })?;
    let then_branch = extract_block_statements(then_block_pair)?;

    let mut else_branch: Vec<Spanned<Statement>> = Vec::new();
    for item in inner {
        if item.as_rule() == Rule::else_clause {
            else_branch = extract_else_clause(item)?;
        }
    }

    Ok(Statement::If {
        condition,
        then_branch,
        else_branch,
    })
}

fn extract_else_clause(pair: Pair<Rule>) -> Result<Vec<Spanned<Statement>>, Diagnostic> {
    // else_clause = { "else" ~ block_statement }
    extract_block_statements(find_child(pair, Rule::block_statement, "Else clause")?)
}

fn extract_block_statements(
    block_pair: Pair<Rule>,
) -> Result<Vec<Spanned<Statement>>, Diagnostic> {
    // block_statement = { "{" ~ statement* ~ "}" }
    let mut statements = Vec::new();
    for item in block_pair.into_inner() {
        statements.push(super::build_statement(item)?);
    }
    Ok(statements)
}

pub(crate) fn build_while_statement(pair: Pair<Rule>) -> Result<Statement, Diagnostic> {
    // while_statement = { "while" ~ "(" ~ expression ~ ")" ~ block_statement }
    let outer_span = span_of(&pair);
    let mut inner = pair.into_inner();

    let condition_pair = inner.find(|p| p.as_rule() == Rule::expression).ok_or_else(|| {
        Diagnostic::new("While statement missing condition expression.").with_span(outer_span)
    })?;
    let condition = build_expression(condition_pair)?;

    let body_block_pair = inner
        .find(|p| p.as_rule() == Rule::block_statement)
        .ok_or_else(|| {
            Diagnostic::new("While statement missing body block.").with_span(outer_span)
        })?;
    let body = extract_block_statements(body_block_pair)?;

    Ok(Statement::While { condition, body })
}

pub(crate) fn build_function_definition(pair: Pair<Rule>) -> Result<Statement, Diagnostic> {
    // function_definition = { "def" ~ identifier ~ "(" ~ parameter_list? ~ ")" ~ block_statement }
    let outer_span = span_of(&pair);
    let mut inner = pair.into_inner();

    let name_pair = inner.find(|p| p.as_rule() == Rule::identifier).ok_or_else(|| {
        Diagnostic::new("Function definition missing name.").with_span(outer_span)
    })?;
    let name = name_pair.as_str().to_string();

    let mut parameters: Vec<String> = Vec::new();
    let mut body: Vec<Spanned<Statement>> = Vec::new();

    for item in inner {
        match item.as_rule() {
            Rule::parameter_list => {
                parameters = item
                    .into_inner()
                    .filter(|p| p.as_rule() == Rule::identifier)
                    .map(|p| p.as_str().to_string())
                    .collect();
            }
            Rule::block_statement => {
                body = extract_block_statements(item)?;
            }
            _ => {}
        }
    }

    Ok(Statement::FunctionDefinition {
        name,
        parameters,
        body,
    })
}

pub(crate) fn build_return_statement(pair: Pair<Rule>) -> Result<Statement, Diagnostic> {
    // return_statement = { "return" ~ expression? ~ ";" }
    let expression_pair = pair.into_inner().find(|p| p.as_rule() == Rule::expression);
    let value = match expression_pair {
        Some(p) => Some(build_expression(p)?),
        None => None,
    };

    Ok(Statement::Return { value })
}

pub(crate) fn build_use_statement(pair: Pair<Rule>) -> Result<Statement, Diagnostic> {
    // use_statement = { "use" ~ path ~ ";" }
    let path_pair = find_child(pair, Rule::path, "Use statement")?;
    let path = collect_path_segments(path_pair, "Use statement")?;

    Ok(Statement::Use { path })
}
