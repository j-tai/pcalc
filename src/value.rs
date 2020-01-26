#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A value that an expression can return, such as a float.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Value {
    /// A floating-point number.
    Float(f64),
}
