use std::fmt;
use std::fmt::{Display, Formatter};
use std::result::Result as StdResult;

use crate::Span;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    /// Syntax error.
    Syntax,
    /// Tried to access an undefined variable.
    Undefined(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Error::Syntax => write!(f, "Syntax error"),
            Error::Undefined(v) => write!(f, "Undefined variable '{}'", v),
        }
    }
}

pub type Result<T> = StdResult<T, (Error, Span)>;
