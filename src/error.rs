//! Error handling.

use std::fmt;
use std::fmt::{Display, Formatter};
use std::result::Result as StdResult;

use crate::Span;

/// A parse or execution error.
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    /// Syntax error.
    Syntax,
    /// Tried to access an undefined variable.
    Undefined(String),
    /// Invalid argument type.
    Type,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Error::Syntax => write!(f, "Syntax error"),
            Error::Undefined(v) => write!(f, "Undefined variable '{}'", v),
            Error::Type => write!(f, "Invalid argument type"),
        }
    }
}

pub type Result<T> = StdResult<T, (Error, Span)>;
