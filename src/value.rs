use num::rational::Ratio;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A value that an expression can return, such as a float.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Value {
    /// A rational number.
    Ratio(Ratio<i64>),
    /// A floating-point number.
    Float(f64),
}

impl Value {
    /// Convert the value to an `f64`.
    pub fn to_f64(&self) -> f64 {
        match self {
            Value::Ratio(f) => *f.numer() as f64 / *f.denom() as f64,
            Value::Float(f) => *f,
        }
    }
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
        Value::Float(val.into())
    }
}
