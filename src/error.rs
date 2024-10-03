use std::{fmt, io};

#[derive(Debug)]
pub enum CommandError {
    MissingArgument(String),
    InvalidNumberFormat(String),
    MalformedLine(String),
    IoError(io::Error),
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandError::InvalidNumberFormat(msg) => write!(f, "Invalid number format: {}", msg),
            CommandError::MalformedLine(msg) => write!(f, "Malformed line: {}", msg),
            CommandError::MissingArgument(msg) => write!(f, "Missing argument: {}", msg),
            CommandError::IoError(err) => write!(f, "I/O error: {}", err),
        }
    }
}

impl std::error::Error for CommandError {}
