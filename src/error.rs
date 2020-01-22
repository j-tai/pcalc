use std::error::Error as StdError;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::result::Result as StdResult;

#[derive(Clone, Debug, PartialEq)]
pub struct Error {
    pub kind: ErrorKind,
}

impl StdError for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ErrorKind {
    /// Syntax error.
    Syntax,
    /// Argument is outside the domain of the function.
    Domain,
    /// Division or modulo by zero.
    DivZero,
    /// Tried to access an undefined variable.
    Undefined,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type Result<T> = StdResult<T, Error>;
