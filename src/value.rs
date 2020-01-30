use num::rational::Ratio;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{Expression, Span};

/// A value that an expression can return, such as a float.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Value {
    /// A rational number.
    Ratio(Ratio<i64>),
    /// A floating-point number.
    Float(f64),
    /// A function.
    Func(Vec<String>, Box<(Expression, Span)>),
}

impl From<i64> for Value {
    fn from(val: i64) -> Value {
        Value::Ratio(val.into())
    }
}

impl From<Ratio<i64>> for Value {
    fn from(val: Ratio<i64>) -> Value {
        Value::Ratio(val)
    }
}

impl From<(i64, i64)> for Value {
    fn from(val: (i64, i64)) -> Value {
        Value::Ratio(val.into())
    }
}

impl From<f64> for Value {
    fn from(val: f64) -> Value {
        Value::Float(val)
    }
}
