//! Execution context, options, and variable storage.

use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::Value;

/// Execution context, options, and variables.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Context {
    pub angle: AngleMeasure,
    pub notation_range: (f64, f64),
    pub vars: HashMap<String, Value>,
}

impl Context {
    /// Format a number for displaying purposes.
    pub fn display<'a>(&'a self, num: &'a Value) -> impl Display + 'a {
        Format { ctx: self, num }
    }
}

impl Default for Context {
    fn default() -> Self {
        Context {
            angle: Default::default(),
            notation_range: (1.0e-3, 1.0e+7),
            vars: Default::default(),
        }
    }
}

/// Angle measurement unit: degrees or radians.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AngleMeasure {
    Degrees,
    Radians,
}

impl AngleMeasure {
    /// Convert the angle from `self`'s angle measure to radians.
    pub fn to_rad(self, value: f64) -> f64 {
        match self {
            AngleMeasure::Degrees => value.to_radians(),
            AngleMeasure::Radians => value,
        }
    }

    /// Convert the angle from radians to `self`'s angle measure.
    pub fn from_rad(self, value: f64) -> f64 {
        match self {
            AngleMeasure::Degrees => value.to_degrees(),
            AngleMeasure::Radians => value,
        }
    }
}

impl Default for AngleMeasure {
    fn default() -> Self {
        AngleMeasure::Radians
    }
}

// Formatting numbers

struct Format<'a> {
    ctx: &'a Context,
    num: &'a Value,
}

impl Display for Format<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let Value::Float(num) = self.num;
        let mag = num.abs();
        if self.ctx.notation_range.0 < mag && mag < self.ctx.notation_range.1 || mag == 0.0 {
            // Show number normally (no scientific notation) if within the range
            // or equal to zero
            write!(f, "{}", num)
        } else if mag < 1.0 {
            write!(f, "{:e}", num)
        } else {
            // Force '+' on exponent
            let s = format!("{:e}", num);
            if let Some(e) = s.find('e') {
                write!(f, "{}e+{}", &s[..e], &s[(e + 1)..])
            } else {
                // No 'e' found -- probably +/- infinity
                write!(f, "{}", s)
            }
        }
    }
}
