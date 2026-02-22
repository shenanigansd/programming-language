use std::fmt;

#[derive(Debug)]
pub enum ExitCode {
    Usage,   // like 64/65-ish
    Compile, // 65
    Runtime, // 70
}

#[derive(Debug)]
pub struct CommandError {
    pub code: ExitCode,
    pub message: String,
}

impl CommandError {
    pub fn new(code: ExitCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl fmt::Display for CommandError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.message)
    }
}
