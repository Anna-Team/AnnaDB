use crate::tyson::de::Rule;
use pest::error::Error;
use std::ffi::OsString;
use std::num::{ParseFloatError, ParseIntError};
use uuid::Error as UuidError;

#[derive(Debug, thiserror::Error)]
pub enum DBError {
    // IO and system errors
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("OsString conversion error")]
    OsStringConversion,

    // Parsing errors
    #[error("TySON parse error: {0}")]
    TysonParse(String),

    #[error("Parse int error: {0}")]
    ParseInt(#[from] ParseIntError),

    #[error("Parse float error: {0}")]
    ParseFloat(#[from] ParseFloatError),

    #[error("UUID parse error: {0}")]
    UuidParse(#[from] UuidError),

    #[error("Unexpected parsing error")]
    UnexpectedParsing,

    #[error("Deserialization error")]
    Deserialization,

    #[error("Bool parsing error")]
    BoolParse,

    // Storage errors
    #[error("Collection not found: {0}")]
    CollectionNotFound(String),

    #[error("Invalid collection name: {0}")]
    InvalidCollectionName(String),

    #[error("Item not found in collection")]
    ItemNotFound,

    #[error("Internal storage read error")]
    StorageRead,

    #[error("Fetch recursion limit exceeded")]
    FetchRecursion,

    // Query errors
    #[error("Query unavailable: {0}")]
    QueryUnavailable(String),

    #[error("Unexpected query type")]
    UnexpectedQueryType,

    #[error("Unexpected type: {0}")]
    UnexpectedType(String),

    #[error("Unsupported operation: {0}")]
    UnsupportedOperation(String),

    #[error("Type mismatch: {0}")]
    TypeMismatch(String),

    // Validation errors
    #[error("Validation error: {0}")]
    Validation(String),
}

impl DBError {
    pub(crate) fn new(msg: &str) -> Self {
        // Backwards compatibility - route known messages to proper variants
        // New code should use specific variants directly
        Self::UnsupportedOperation(msg.to_string())
    }

    pub fn unexpected_parsing() -> Self {
        Self::UnexpectedParsing
    }

    pub fn msg(&self) -> String {
        self.to_string()
    }
}

impl From<Error<Rule>> for DBError {
    fn from(error: Error<Rule>) -> Self {
        Self::TysonParse(error.to_string())
    }
}

impl From<OsString> for DBError {
    fn from(_: OsString) -> Self {
        Self::OsStringConversion
    }
}
