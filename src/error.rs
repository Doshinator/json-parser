use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct ParseError {
    pub message: String,
    pub position: usize,
}

impl ParseError {
    pub fn new(message: impl Into<String>, position: usize) -> Self {
        ParseError { 
            message: message.into(),
            position 
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parse error at position {} : {}", self.position, self.message)
    }
}

// allows parase error to work with std error trait
impl std::error::Error for ParseError {}


// Alias for Result<T, ParseError> 
pub type ParseType<T> = Result<T, ParseError>;