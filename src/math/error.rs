use std::result::Result as StdResult;
use std::error::Error as StdError;
use std::fmt;
use std::fmt::{Display,Formatter};

use crate::math::Expression;

#[derive(Clone, Debug, PartialEq)]
pub struct Error<'a> {
    pub kind: ErrorKind,
    pub expr: &'a Expression,
}

impl StdError for Error<'_> {}

impl Display for Error<'_> {
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
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type Result<'a, T> = StdResult<T, Error<'a>>;
