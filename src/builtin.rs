//! Built-in constants and functions.

use std::f64::consts;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::{Context, Error};

#[cfg(test)]
mod tests;

/// A built-in mathematical constant, such as pi.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Constant {
    /// Pi, which is the ratio of a circle's semicircumference to its radius.
    Pi,
    /// E, also known as Euler's number, which is the base of the natural
    /// logarithm.
    E,
}

impl Constant {
    /// Get the value of the constant.
    pub fn value(self) -> f64 {
        match self {
            Constant::Pi => consts::PI,
            Constant::E => consts::E,
        }
    }
}

impl FromStr for Constant {
    type Err = ();

    fn from_str(s: &str) -> Result<Constant, ()> {
        match s {
            "pi" => Ok(Constant::Pi),
            "e" => Ok(Constant::E),
            _ => Err(()),
        }
    }
}

impl Display for Constant {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let s = match self {
            Constant::Pi => "pi",
            Constant::E => "e",
        };
        write!(f, "{}", s)
    }
}

/// A built-in, native function.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Function {
    /// Absolute value.
    Abs,
    /// Sine.
    Sin,
    /// Cosine.
    Cos,
    /// Tangent.
    Tan,
    /// Arcsine, also known as inverse sine.
    Asin,
    /// Arccosine, also known as inverse cosine.
    Acos,
    /// Arctangent, also known as inverse tangent.
    Atan,
    /// Hyperbolic sine.
    Sinh,
    /// Hyperbolic cosine.
    Cosh,
    /// Hyperbolic tangent.
    Tanh,
    /// Inverse hyperbolic sine.
    Asinh,
    /// Inverse hyperbolic cosine.
    Acosh,
    /// Inverse hyperbolic tangent.
    Atanh,
}

impl Function {
    /// Apply the function to a number given a context.
    ///
    /// The context is primarily used to determine the angle with which the
    /// calculation should be performed (i.e., degrees or radians).
    pub fn apply(self, x: f64, ctx: &Context) -> Result<f64, Error> {
        match self {
            Function::Abs => Ok(x.abs()),
            Function::Sin => Ok(ctx.angle.to_rad(x).sin()),
            Function::Cos => Ok(ctx.angle.to_rad(x).cos()),
            Function::Tan => Ok(ctx.angle.to_rad(x).tan()),
            Function::Asin => Ok(ctx.angle.from_rad(x.asin())),
            Function::Acos => Ok(ctx.angle.from_rad(x.acos())),
            Function::Atan => Ok(ctx.angle.from_rad(x.atan())),
            Function::Sinh => Ok(x.sinh()),
            Function::Cosh => Ok(x.cosh()),
            Function::Tanh => Ok(x.tanh()),
            Function::Asinh => Ok(x.asinh()),
            Function::Acosh => Ok(x.acosh()),
            Function::Atanh => Ok(x.atanh()),
        }
    }
}

impl FromStr for Function {
    type Err = ();

    fn from_str(s: &str) -> Result<Function, ()> {
        use Function::*;
        match s {
            "abs" => Ok(Abs),
            "sin" => Ok(Sin),
            "cos" => Ok(Cos),
            "tan" => Ok(Tan),
            "asin" => Ok(Asin),
            "acos" => Ok(Acos),
            "atan" => Ok(Atan),
            "sinh" => Ok(Sinh),
            "cosh" => Ok(Cosh),
            "tanh" => Ok(Tanh),
            "asinh" => Ok(Asinh),
            "acosh" => Ok(Acosh),
            "atanh" => Ok(Atanh),
            _ => Err(()),
        }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let s = match self {
            Function::Abs => "abs",
            Function::Sin => "sin",
            Function::Cos => "cos",
            Function::Tan => "tan",
            Function::Asin => "asin",
            Function::Acos => "acos",
            Function::Atan => "atan",
            Function::Sinh => "sinh",
            Function::Cosh => "cosh",
            Function::Tanh => "tanh",
            Function::Asinh => "asinh",
            Function::Acosh => "acosh",
            Function::Atanh => "atanh",
        };
        write!(f, "{}", s)
    }
}
