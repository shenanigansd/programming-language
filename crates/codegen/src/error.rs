use std::fmt;

#[derive(Debug)]
pub struct CodegenError {
    message: String,
}

impl CodegenError {
    pub fn new(message: impl Into<String>) -> Self {
        CodegenError {
            message: message.into(),
        }
    }
}

impl fmt::Display for CodegenError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.message)
    }
}
