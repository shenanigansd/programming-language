use amarok_syntax::{BinaryOperator, Expression, Program, Span, Spanned, Statement};
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
mod error;
pub use error::ParseError;

#[derive(Parser)]
#[grammar = "amarok.pest"]
struct AmarokGrammar;

/// Parse a full Amarok program (multiple statements).
pub fn parse_program(source: &str) -> Result<Program, String> {
    let mut pairs = AmarokGrammar::parse(Rule::program, source)
        .map_err(|error| error.to_string())?;

    let program_pair = pairs
        .next()
        .ok_or_else(|| "Expected a program, found nothing.".to_string())?;

    build_program(program_pair)
}

/// Parse a single statement (useful for REPL later).
pub fn parse_statement(source: &str) -> Result<Spanned<Statement>, String> {
    let mut pairs = AmarokGrammar::parse(Rule::statement, source)
        .map_err(|error| error.to_string())?;

    let statement_pair = pairs
        .next()
        .ok_or_else(|| "Expected a statement, found nothing.".to_string())?;

    build_statement(statement_pair)
}

/// Parse a single expression (useful for unit tests and REPL experiments).
pub fn parse_expression(source: &str) -> Result<Spanned<Expression>, String> {
    let mut pairs = AmarokGrammar::parse(Rule::expression, source)
        .map_err(|error| error.to_string())?;

    let expression_pair = pairs
        .next()
        .ok_or_else(|| "Expected an expression, found nothing.".to_string())?;

    build_expression(expression_pair)
}

fn build_program(pair: Pair<Rule>) -> Result<Program, String> {
    if pair.as_rule() != Rule::program {
        return Err(format!("Expected program rule, got {:?}", pair.as_rule()));
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
            | Rule::expression_statement => statements.push(build_statement(item)?),

            // Ignore anything else (EOI, or future wrapper rules).
            _ => {}
        }
    }

    Ok(Program { statements })
}

fn build_statement(pair: Pair<Rule>) -> Result<Spanned<Statement>, String> {
    let statement_span = span_of(&pair);

    let statement_value = match pair.as_rule() {
        Rule::assignment_statement => build_assignment_statement(pair)?,
        Rule::expression_statement => build_expression_statement(pair)?,
        Rule::block_statement => build_block_statement(pair)?,
        Rule::if_statement => build_if_statement(pair)?,
        Rule::while_statement => build_while_statement(pair)?,
        Rule::function_definition => build_function_definition(pair)?,
        Rule::return_statement => build_return_statement(pair)?,
        other => return Err(format!("Unhandled statement rule: {other:?}")),
    };

    Ok(Spanned::new(statement_span, statement_value))
}

fn build_assignment_statement(pair: Pair<Rule>) -> Result<Statement, String> {
    // assignment_statement = { identifier ~ "=" ~ expression ~ ";" }
    let mut inner = pair.into_inner();

    let name_pair = inner
        .next()
        .ok_or_else(|| "Assignment missing identifier.".to_string())?;
    if name_pair.as_rule() != Rule::identifier {
        return Err(format!(
            "Assignment expected identifier, got {:?}",
            name_pair.as_rule()
        ));
    }
    let name = name_pair.as_str().to_string();

    let expression_pair = inner
        .find(|p| p.as_rule() == Rule::expression)
        .ok_or_else(|| "Assignment missing expression.".to_string())?;

    let value = build_expression(expression_pair)?;

    Ok(Statement::Assignment { name, value })
}

fn build_expression_statement(pair: Pair<Rule>) -> Result<Statement, String> {
    // expression_statement = { expression ~ ";" }
    let expression_pair = pair
        .into_inner()
        .find(|p| p.as_rule() == Rule::expression)
        .ok_or_else(|| "Expression statement missing expression.".to_string())?;

    Ok(Statement::Expression {
        expression: build_expression(expression_pair)?,
    })
}

fn build_block_statement(pair: Pair<Rule>) -> Result<Statement, String> {
    // block_statement = { "{" ~ statement* ~ "}" }
    let mut statements: Vec<Spanned<Statement>> = Vec::new();

    for item in pair.into_inner() {
        // statement is silent in the grammar, so these should be concrete statement rules.
        statements.push(build_statement(item)?);
    }

    Ok(Statement::Block { statements })
}

fn build_if_statement(pair: Pair<Rule>) -> Result<Statement, String> {
    // if_statement = { "if" ~ "(" ~ expression ~ ")" ~ block_statement ~ else_clause? }
    let mut inner = pair.into_inner();

    let condition_pair = inner
        .find(|p| p.as_rule() == Rule::expression)
        .ok_or_else(|| "If statement missing condition expression.".to_string())?;
    let condition = build_expression(condition_pair)?;

    let then_block_pair = inner
        .find(|p| p.as_rule() == Rule::block_statement)
        .ok_or_else(|| "If statement missing then block.".to_string())?;
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

fn extract_else_clause(pair: Pair<Rule>) -> Result<Vec<Spanned<Statement>>, String> {
    // else_clause = { "else" ~ block_statement }
    let block_pair = pair
        .into_inner()
        .find(|p| p.as_rule() == Rule::block_statement)
        .ok_or_else(|| "Else clause missing block.".to_string())?;

    extract_block_statements(block_pair)
}

fn extract_block_statements(block_pair: Pair<Rule>) -> Result<Vec<Spanned<Statement>>, String> {
    // block_statement = { "{" ~ statement* ~ "}" }
    let mut statements = Vec::new();
    for item in block_pair.into_inner() {
        statements.push(build_statement(item)?);
    }
    Ok(statements)
}

fn build_while_statement(pair: Pair<Rule>) -> Result<Statement, String> {
    // while_statement = { "while" ~ "(" ~ expression ~ ")" ~ block_statement }
    let mut inner = pair.into_inner();

    let condition_pair = inner
        .find(|p| p.as_rule() == Rule::expression)
        .ok_or_else(|| "While statement missing condition expression.".to_string())?;
    let condition = build_expression(condition_pair)?;

    let body_block_pair = inner
        .find(|p| p.as_rule() == Rule::block_statement)
        .ok_or_else(|| "While statement missing body block.".to_string())?;
    let body = extract_block_statements(body_block_pair)?;

    Ok(Statement::While { condition, body })
}

fn build_function_definition(pair: Pair<Rule>) -> Result<Statement, String> {
    // function_definition = { "def" ~ identifier ~ "(" ~ parameter_list? ~ ")" ~ block_statement }
    let mut inner = pair.into_inner();

    let name_pair = inner
        .find(|p| p.as_rule() == Rule::identifier)
        .ok_or_else(|| "Function definition missing name.".to_string())?;
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

fn build_return_statement(pair: Pair<Rule>) -> Result<Statement, String> {
    // return_statement = { "return" ~ expression? ~ ";" }
    let expression_pair = pair.into_inner().find(|p| p.as_rule() == Rule::expression);
    let value = match expression_pair {
        Some(p) => Some(build_expression(p)?),
        None => None,
    };

    Ok(Statement::Return { value })
}

fn build_expression(pair: Pair<Rule>) -> Result<Spanned<Expression>, String> {
    let expression_span = span_of(&pair);

    match pair.as_rule() {
        Rule::expression => build_expression(expect_single_inner(pair, "expression")?),

        Rule::addition => build_left_associative_binary(
            pair,
            Rule::add_operator,
            operator_from_add_text,
        ),

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

    loop {
        let operator_pair = match inner.next() {
            Some(p) => p,
            None => break,
        };

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

fn span_of(pair: &Pair<Rule>) -> Span {
    let span = pair.as_span();
    Span::new(span.start(), span.end())
}

fn expect_single_inner<'input>(
    pair: Pair<'input, Rule>,
    context: &str,
) -> Result<Pair<'input, Rule>, String> {
    let mut inner = pair.into_inner();
    let first = inner
        .next()
        .ok_or_else(|| format!("{context} had no inner content."))?;
    if inner.next().is_some() {
        return Err(format!("{context} had more than one inner element."));
    }
    Ok(first)
}

fn unquote_string(text: &str) -> Result<String, String> {
    if !text.starts_with('"') || !text.ends_with('"') || text.len() < 2 {
        return Err(format!("Invalid string literal: {text}"));
    }

    let content = &text[1..text.len() - 1];

    // Minimal unescaping: support \" and \\ only.
    let mut result = String::with_capacity(content.len());
    let mut chars = content.chars();
    while let Some(character) = chars.next() {
        if character == '\\' {
            let next = chars
                .next()
                .ok_or_else(|| "String ends with a backslash.".to_string())?;
            match next {
                '"' => result.push('"'),
                '\\' => result.push('\\'),
                other => {
                    return Err(format!(
                        "Unsupported escape sequence: \\{other} (only \\\" and \\\\ supported)"
                    ));
                }
            }
        } else {
            result.push(character);
        }
    }

    Ok(result)
}

