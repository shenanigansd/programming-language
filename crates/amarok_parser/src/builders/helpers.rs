use amarok_syntax::Span;
use pest::iterators::Pair;

use crate::grammar::Rule;

pub(crate) fn span_of(pair: &Pair<Rule>) -> Span {
    let span = pair.as_span();
    Span::new(span.start(), span.end())
}

pub(crate) fn expect_single_inner<'input>(
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

pub(crate) fn unquote_string(text: &str) -> Result<String, String> {
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
