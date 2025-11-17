use std::fmt;

#[derive(Debug)]
pub struct DriverError {
    message: String,
}

impl DriverError {
    pub fn new(message: impl Into<String>) -> Self {
        DriverError {
            message: message.into(),
        }
    }
}

impl fmt::Display for DriverError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.message)
    }
}
