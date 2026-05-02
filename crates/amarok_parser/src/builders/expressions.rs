use amarok_syntax::{BinaryOperator, Diagnostic, Expression, Spanned};
use pest::iterators::Pair;

use crate::grammar::Rule;

use super::helpers::{
    collect_path_segments, expect_single_inner, find_child, span_of, unquote_string,
};

pub(crate) fn build_expression(pair: Pair<Rule>) -> Result<Spanned<Expression>, Diagnostic> {
    let expression_span = span_of(&pair);

    match pair.as_rule() {
        Rule::expression => build_expression(expect_single_inner(pair, "expression")?),

        Rule::addition => {
            build_left_associative_binary(pair, Rule::add_operator, operator_from_add_text)
        }

        Rule::multiplication => build_left_associative_binary(
            pair,
            Rule::multiply_operator,
            operator_from_multiply_text,
        ),

        Rule::primary => build_expression(expect_single_inner(pair, "primary")?),

        Rule::parenthesized => {
            // parenthesized = { "(" ~ expression ~ ")" }
            build_expression(find_child(pair, Rule::expression, "Parenthesized expression")?)
        }

        Rule::function_call => build_function_call(pair),

        Rule::variable => {
            let inner = expect_single_inner(pair, "variable")?;
            if inner.as_rule() != Rule::identifier {
                return Err(Diagnostic::new(format!(
                    "Expected identifier inside variable, got {:?}",
                    inner.as_rule()
                ))
                .with_span(span_of(&inner)));
            }
            Ok(Spanned::new(
                expression_span,
                Expression::Variable(inner.as_str().to_string()),
            ))
        }

        Rule::integer => {
            let text = pair.as_str();
            let value: i64 = text.parse().map_err(|_| {
                Diagnostic::new(format!("Invalid integer literal: {text}"))
                    .with_span(expression_span)
            })?;
            Ok(Spanned::new(expression_span, Expression::Integer(value)))
        }

        Rule::string => Ok(Spanned::new(
            expression_span,
            Expression::String(unquote_string(pair.as_str(), expression_span)?),
        )),

        Rule::identifier => Ok(Spanned::new(
            expression_span,
            Expression::Variable(pair.as_str().to_string()),
        )),

        other => Err(
            Diagnostic::new(format!("Unhandled rule in build_expression: {other:?}"))
                .with_span(expression_span),
        ),
    }
}

fn build_function_call(pair: Pair<Rule>) -> Result<Spanned<Expression>, Diagnostic> {
    // function_call = { path ~ "(" ~ argument_list? ~ ")" }
    let call_span = span_of(&pair);

    let mut inner = pair.into_inner();

    let path_pair = inner
        .next()
        .ok_or_else(|| Diagnostic::new("Function call missing path.").with_span(call_span))?;

    if path_pair.as_rule() != Rule::path {
        return Err(Diagnostic::new(format!(
            "Function call expected path, got {:?}",
            path_pair.as_rule()
        ))
        .with_span(span_of(&path_pair)));
    }

    let path = collect_path_segments(path_pair, "Function call")?;

    let mut arguments: Vec<Spanned<Expression>> = Vec::new();
    for item in inner {
        if item.as_rule() == Rule::argument_list {
            arguments = build_argument_list(item)?;
        }
    }

    Ok(Spanned::new(
        call_span,
        Expression::FunctionCall { path, arguments },
    ))
}

fn build_argument_list(pair: Pair<Rule>) -> Result<Vec<Spanned<Expression>>, Diagnostic> {
    // argument_list = { expression ~ ("," ~ expression)* }
    let mut arguments: Vec<Spanned<Expression>> = Vec::new();

    for item in pair.into_inner() {
        if item.as_rule() == Rule::expression {
            arguments.push(build_expression(item)?);
        }
    }

    Ok(arguments)
}

fn build_left_associative_binary(
    pair: Pair<Rule>,
    expected_operator_rule: Rule,
    operator_from_text: fn(&str) -> Result<BinaryOperator, String>,
) -> Result<Spanned<Expression>, Diagnostic> {
    // addition = { multiplication ~ (add_operator ~ multiplication)* }
    // multiplication = { primary ~ (multiply_operator ~ primary)* }
    //
    // Children look like: operand, operator, operand, operator, operand...
    let full_span = span_of(&pair);
    let mut inner = pair.into_inner();

    let first_operand_pair = inner.next().ok_or_else(|| {
        Diagnostic::new("Expected left operand, found nothing.").with_span(full_span)
    })?;

    let mut expression = build_expression(first_operand_pair)?;

    while let Some(operator_pair) = inner.next() {
        if operator_pair.as_rule() != expected_operator_rule {
            return Err(Diagnostic::new(format!(
                "Expected operator rule {:?}, got {:?}",
                expected_operator_rule,
                operator_pair.as_rule()
            ))
            .with_span(span_of(&operator_pair)));
        }

        let operator_span = span_of(&operator_pair);
        let operator = operator_from_text(operator_pair.as_str())
            .map_err(|message| Diagnostic::new(message).with_span(operator_span))?;

        let right_operand_pair = inner.next().ok_or_else(|| {
            Diagnostic::new("Expected right operand after operator.").with_span(operator_span)
        })?;

        let right_expression = build_expression(right_operand_pair)?;

        expression = Spanned::new(
            full_span,
            Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right_expression),
            },
        );
    }

    Ok(expression)
}

fn operator_from_add_text(text: &str) -> Result<BinaryOperator, String> {
    match text {
        "+" => Ok(BinaryOperator::Add),
        "-" => Ok(BinaryOperator::Subtract),
        _ => Err(format!("Unknown add operator: {text}")),
    }
}

fn operator_from_multiply_text(text: &str) -> Result<BinaryOperator, String> {
    match text {
        "*" => Ok(BinaryOperator::Multiply),
        "/" => Ok(BinaryOperator::Divide),
        _ => Err(format!("Unknown multiply operator: {text}")),
    }
}
