use amarok_syntax::{Diagnostic, Span};
use pest::iterators::Pair;

use crate::grammar::Rule;

pub(crate) fn span_of(pair: &Pair<Rule>) -> Span {
    let span = pair.as_span();
    Span::new(span.start(), span.end())
}

pub(crate) fn expect_single_inner<'input>(
    pair: Pair<'input, Rule>,
    context: &str,
) -> Result<Pair<'input, Rule>, Diagnostic> {
    let span = span_of(&pair);
    let mut inner = pair.into_inner();
    let first = inner.next().ok_or_else(|| {
        Diagnostic::new(format!("{context} had no inner content.")).with_span(span)
    })?;
    if inner.next().is_some() {
        return Err(
            Diagnostic::new(format!("{context} had more than one inner element.")).with_span(span),
        );
    }
    Ok(first)
}

/// Find the first inner child of `pair` matching `rule`.
///
/// `context` is included in the error message to identify which builder
/// asked for it (e.g. "If statement missing condition expression.").
pub(crate) fn find_child<'input>(
    pair: Pair<'input, Rule>,
    rule: Rule,
    context: &str,
) -> Result<Pair<'input, Rule>, Diagnostic> {
    let span = span_of(&pair);
    pair.into_inner()
        .find(|p| p.as_rule() == rule)
        .ok_or_else(|| Diagnostic::new(format!("{context} missing {rule:?}.")).with_span(span))
}

/// Convert a `path` parse node into its identifier segments, validating
/// each child is an `identifier` and rejecting empty paths.
pub(crate) fn collect_path_segments(
    path_pair: Pair<Rule>,
    context: &str,
) -> Result<Vec<String>, Diagnostic> {
    let path_span = span_of(&path_pair);
    let mut path: Vec<String> = Vec::new();
    for segment in path_pair.into_inner() {
        if segment.as_rule() != Rule::identifier {
            return Err(Diagnostic::new(format!(
                "{context} expected identifier, got {:?}",
                segment.as_rule()
            ))
            .with_span(span_of(&segment)));
        }
        path.push(segment.as_str().to_string());
    }

    if path.is_empty() {
        return Err(Diagnostic::new(format!("{context} path was empty.")).with_span(path_span));
    }

    Ok(path)
}

pub(crate) fn unquote_string(text: &str, span: Span) -> Result<String, Diagnostic> {
    if !text.starts_with('"') || !text.ends_with('"') || text.len() < 2 {
        return Err(Diagnostic::new(format!("Invalid string literal: {text}")).with_span(span));
    }

    let content = &text[1..text.len() - 1];

    // Minimal unescaping: support \" and \\ only.
    let mut result = String::with_capacity(content.len());
    let mut chars = content.chars();
    while let Some(character) = chars.next() {
        if character == '\\' {
            let next = chars
                .next()
                .ok_or_else(|| Diagnostic::new("String ends with a backslash.").with_span(span))?;
            match next {
                '"' => result.push('"'),
                '\\' => result.push('\\'),
                other => {
                    return Err(Diagnostic::new(format!(
                        "Unsupported escape sequence: \\{other} (only \\\" and \\\\ supported)"
                    ))
                    .with_span(span));
                }
            }
        } else {
            result.push(character);
        }
    }

    Ok(result)
}
