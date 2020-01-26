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
