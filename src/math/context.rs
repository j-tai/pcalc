use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub struct Context {
    pub angle: AngleMeasure,
    pub notation_range: (f64, f64),
}

impl Context {
    /// Format a number for displaying purposes.
    pub fn display<'a>(&'a self, num: f64) -> impl Display + 'a {
        Format { ctx: self, num }
    }
}

impl Default for Context {
    fn default() -> Self {
        Context {
            angle: Default::default(),
            notation_range: (1.0e-3, 1.0e+7),
        }
    }
}

/// Angle measurement unit: degrees or radians.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AngleMeasure {
    Degrees,
    Radians,
}

impl AngleMeasure {
    pub fn to_rad(self, value: f64) -> f64 {
        match self {
            AngleMeasure::Degrees => value.to_radians(),
            AngleMeasure::Radians => value,
        }
    }

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
    num: f64,
}

impl Display for Format<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.ctx.notation_range.0 < self.num && self.num < self.ctx.notation_range.1 {
            write!(f, "{}", self.num)
        } else if self.num < 1.0 {
            write!(f, "{:e}", self.num)
        } else {
            // Force '+' on exponent
            let s = format!("{:e}", self.num);
            let e = s.find('e').unwrap();
            write!(f, "{}e+{}", &s[..e], &s[(e + 1)..])
        }
    }
}
