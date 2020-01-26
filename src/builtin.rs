//! Built-in constants and functions.

use std::f64::consts;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use num::traits::Signed;

use crate::{Context, Error, Value};

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
    pub fn value(self) -> Value {
        match self {
            Constant::Pi => Value::Float(consts::PI),
            Constant::E => Value::Float(consts::E),
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
    pub fn apply(self, x: Value, ctx: &Context) -> Result<Value, Error> {
        use Value::Float;
        match self {
            Function::Abs => match x {
                Value::Float(f) => Ok(Value::Float(f.abs())),
                Value::Ratio(f) => Ok(Value::Ratio(f.abs())),
            },
            Function::Sin => Ok(Float(ctx.angle.to_rad(x.to_f64()).sin())),
            Function::Cos => Ok(Float(ctx.angle.to_rad(x.to_f64()).cos())),
            Function::Tan => Ok(Float(ctx.angle.to_rad(x.to_f64()).tan())),
            Function::Asin => Ok(Float(ctx.angle.from_rad(x.to_f64().asin()))),
            Function::Acos => Ok(Float(ctx.angle.from_rad(x.to_f64().acos()))),
            Function::Atan => Ok(Float(ctx.angle.from_rad(x.to_f64().atan()))),
            Function::Sinh => Ok(Float(x.to_f64().sinh())),
            Function::Cosh => Ok(Float(x.to_f64().cosh())),
            Function::Tanh => Ok(Float(x.to_f64().tanh())),
            Function::Asinh => Ok(Float(x.to_f64().asinh())),
            Function::Acosh => Ok(Float(x.to_f64().acosh())),
            Function::Atanh => Ok(Float(x.to_f64().atanh())),
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
