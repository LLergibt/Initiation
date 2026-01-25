use std::fmt;

#[derive(Debug)]
pub enum StorageError {
    // Generic categories that work for FS + Cloud
    NotFound { what: &'static str},
    AlreadyExists { what: &'static str },
    PathConflict { details: String },
    InvalidInput { details: String },
    WrongKind { expected: &'static str, actual: &'static str },

    Unauthorized,
    RateLimited,
    Timeout,
    Unavailable { details: String },

    Corrupted { details: String },

    Io(std::io::Error),
    Serde(serde_json::Error),

    Other { details: String },

}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StorageError::NotFound { what } => write!(f, "not found: {what}"),
            StorageError::AlreadyExists { what } => write!(f, "already exists: {what}"),
            StorageError::PathConflict { details } => write!(f, "path conflict: {details}"),
            StorageError::InvalidInput { details } => write!(f, "invalid input: {details}"),
            StorageError::WrongKind { expected, actual } => {
                write!(f, "wrong kind: expected {expected}, got {actual}")
            }
            StorageError::Unauthorized => write!(f, "unauthorized"),
            StorageError::RateLimited => write!(f, "rate limited"),
            StorageError::Timeout => write!(f, "timeout"),
            StorageError::Unavailable { details } => write!(f, "unavailable: {details}"),
            StorageError::Corrupted { details } => write!(f, "corrupted: {details}"),
            StorageError::Io(e) => write!(f, "io error: {e}"),
            StorageError::Serde(e) => write!(f, "serde error: {e}"),
            StorageError::Other { details } => write!(f, "{details}"),
        }
    }
}

impl std::error::Error for StorageError {}

impl From<std::io::Error> for StorageError {
    fn from(e: std::io::Error) -> Self {
        StorageError::Io(e)
    }
}

impl From<serde_json::Error> for StorageError {
    fn from(e: serde_json::Error) -> Self {
        StorageError::Serde(e)
    }
}

pub type Result<T> = std::result::Result<T, StorageError>;