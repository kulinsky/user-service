use std::fmt;

use validator::{ValidationError, ValidationErrors};

pub type Result<T> = std::result::Result<T, Error>;

pub enum ErrorKind {
    NotFound,
    Validation(String),
    Repository(String),
    Configuration,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::NotFound => write!(f, "NotFound"),
            ErrorKind::Validation(err) => write!(f, "Validation({})", err),
            ErrorKind::Repository(repo) => write!(f, "Repository({})", repo),
            ErrorKind::Configuration => write!(f, "Configuration"),
        }
    }
}

pub struct Error {
    pub kind: ErrorKind,
    pub message: String,
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ kind: {}, message: {} }}", self.kind, self.message)
    }
}

impl From<ValidationError> for Error {
    fn from(err: ValidationError) -> Self {
        Self {
            kind: ErrorKind::Validation("email".to_string()),
            message: err.to_string(),
        }
    }
}

impl From<ValidationErrors> for Error {
    fn from(errors: ValidationErrors) -> Self {
        Self {
            kind: ErrorKind::Validation("some_errors".to_string()),
            message: errors.to_string(),
        }
    }
}
