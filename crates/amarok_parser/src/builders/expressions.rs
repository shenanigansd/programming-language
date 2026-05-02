use amarok_syntax::{BinaryOperator, Expression, Spanned};
use pest::iterators::Pair;

use crate::grammar::Rule;

use super::helpers::{expect_single_inner, span_of, unquote_string};

pub(crate) fn build_expression(pair: Pair<Rule>) -> Result<Spanned<Expression>, String> {
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
            let inner_expression_pair = pair
                .into_inner()
                .find(|p| p.as_rule() == Rule::expression)
                .ok_or_else(|| "Parenthesized expression missing inner expression.".to_string())?;
            build_expression(inner_expression_pair)
        }

        Rule::function_call => build_function_call(pair),

        Rule::variable => {
            let inner = expect_single_inner(pair, "variable")?;
            if inner.as_rule() != Rule::identifier {
                return Err(format!(
                    "Expected identifier inside variable, got {:?}",
                    inner.as_rule()
                ));
            }
            Ok(Spanned::new(
                expression_span,
                Expression::Variable(inner.as_str().to_string()),
            ))
        }

        Rule::integer => {
            let text = pair.as_str();
            let value: i64 = text
                .parse()
                .map_err(|_| format!("Invalid integer literal: {text}"))?;
            Ok(Spanned::new(expression_span, Expression::Integer(value)))
        }

        Rule::string => Ok(Spanned::new(
            expression_span,
            Expression::String(unquote_string(pair.as_str())?),
        )),

        Rule::identifier => Ok(Spanned::new(
            expression_span,
            Expression::Variable(pair.as_str().to_string()),
        )),

        other => Err(format!("Unhandled rule in build_expression: {other:?}")),
    }
}

fn build_function_call(pair: Pair<Rule>) -> Result<Spanned<Expression>, String> {
    // function_call = { identifier ~ "(" ~ argument_list? ~ ")" }
    let call_span = span_of(&pair);

    let mut inner = pair.into_inner();

    let name_pair = inner
        .next()
        .ok_or_else(|| "Function call missing name.".to_string())?;

    if name_pair.as_rule() != Rule::identifier {
        return Err(format!(
            "Function call expected identifier, got {:?}",
            name_pair.as_rule()
        ));
    }

    let name = name_pair.as_str().to_string();

    let mut arguments: Vec<Spanned<Expression>> = Vec::new();
    for item in inner {
        if item.as_rule() == Rule::argument_list {
            arguments = build_argument_list(item)?;
        }
    }

    Ok(Spanned::new(
        call_span,
        Expression::FunctionCall { name, arguments },
    ))
}

fn build_argument_list(pair: Pair<Rule>) -> Result<Vec<Spanned<Expression>>, String> {
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
) -> Result<Spanned<Expression>, String> {
    // addition = { multiplication ~ (add_operator ~ multiplication)* }
    // multiplication = { primary ~ (multiply_operator ~ primary)* }
    //
    // Children look like: operand, operator, operand, operator, operand...
    let full_span = span_of(&pair);
    let mut inner = pair.into_inner();

    let first_operand_pair = inner
        .next()
        .ok_or_else(|| "Expected left operand, found nothing.".to_string())?;

    let mut expression = build_expression(first_operand_pair)?;

    while let Some(operator_pair) = inner.next() {
        if operator_pair.as_rule() != expected_operator_rule {
            return Err(format!(
                "Expected operator rule {:?}, got {:?}",
                expected_operator_rule,
                operator_pair.as_rule()
            ));
        }

        let operator = operator_from_text(operator_pair.as_str())?;

        let right_operand_pair = inner
            .next()
            .ok_or_else(|| "Expected right operand after operator.".to_string())?;

        let right_expression = build_expression(right_operand_pair)?;

        // For spans, we use the full chain span (simple and stable).
        // If you want “tight” spans later, we can merge left.start to right.end.
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
