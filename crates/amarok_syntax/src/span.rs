use crate::source_map::FileId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub file_id: FileId,
}

impl Span {
    #[must_use]
    pub fn new(start: usize, end: usize) -> Self {
        Self {
            start,
            end,
            file_id: FileId::DUMMY,
        }
    }

    #[must_use]
    pub fn with_file(start: usize, end: usize, file_id: FileId) -> Self {
        Self {
            start,
            end,
            file_id,
        }
    }

    #[must_use]
    pub fn zero() -> Self {
        Self {
            start: 0,
            end: 0,
            file_id: FileId::DUMMY,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Spanned<T> {
    pub span: Span,
    pub value: T,
}

impl<T> Spanned<T> {
    pub fn new(span: Span, value: T) -> Self {
        Self { span, value }
    }

    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Spanned<U> {
        Spanned {
            span: self.span,
            value: f(self.value),
        }
    }

    pub fn with_zero_span(value: T) -> Self {
        Self {
            span: Span::zero(),
            value,
        }
    }
}

impl<T> From<T> for Spanned<T> {
    fn from(value: T) -> Self {
        Spanned::with_zero_span(value)
    }
}
