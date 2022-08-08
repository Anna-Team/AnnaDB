use crate::tyson::de::Rule;
use pest::error::Error;
use std::ffi::OsString;
use std::io::Error as IoError;
use std::num::{ParseFloatError, ParseIntError};
use uuid::ParseError;

#[derive(Debug)]
pub struct DBError {
    pub(crate) msg: String,
}

impl DBError {
    pub(crate) fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }

    pub fn unexpected_parsing() -> Self {
        Self::new("Unexpected parsing error")
    }
}

impl From<Error<Rule>> for DBError {
    fn from(error: Error<Rule>) -> Self {
        Self {
            msg: error.to_string(),
        }
    }
}

impl From<IoError> for DBError {
    fn from(_: IoError) -> Self {
        Self::new("IO internal error")
    }
}

impl From<ParseIntError> for DBError {
    fn from(_: ParseIntError) -> Self {
        Self::new("Parse int internal error")
    }
}

impl From<OsString> for DBError {
    fn from(_: OsString) -> Self {
        Self::new("OsString internal error")
    }
}

impl From<ParseError> for DBError {
    fn from(_e: ParseError) -> Self {
        Self::new("UUID parse internal error")
    }
}

impl From<ParseFloatError> for DBError {
    fn from(_: ParseFloatError) -> Self {
        Self::new("Number parse internal error")
    }
}
