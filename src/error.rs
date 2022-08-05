use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

pub enum ErrorKind {
    NotFound,
    Repository(String),
    Configuration,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::NotFound => write!(f, "NotFound"),
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
