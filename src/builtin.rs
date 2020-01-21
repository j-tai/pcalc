use std::f64::consts;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::{Context, Error};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Constant {
    Pi,
    E,
}

impl Constant {
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Function {
    Abs,
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Sinh,
    Cosh,
    Tanh,
    Asinh,
    Acosh,
    Atanh,
}

impl Function {
    pub fn apply(self, x: f64, ctx: &Context) -> Result<f64, Error<'static>> {
        match self {
            Function::Abs => Ok(x.abs()),
            Function::Sin => Ok(ctx.angle.to_rad(x).sin()),
            Function::Cos => Ok(ctx.angle.to_rad(x).cos()),
            Function::Tan => Ok(ctx.angle.to_rad(x).tan()),
            Function::Asin => Ok(ctx.angle.from_rad(x.asin())),
            Function::Acos => Ok(ctx.angle.from_rad(x.acos())),
            Function::Atan => Ok(ctx.angle.from_rad(x.atan())),
            Function::Sinh => Ok(ctx.angle.to_rad(x).sinh()),
            Function::Cosh => Ok(ctx.angle.to_rad(x).cosh()),
            Function::Tanh => Ok(ctx.angle.to_rad(x).tanh()),
            Function::Asinh => Ok(ctx.angle.from_rad(x.asinh())),
            Function::Acosh => Ok(ctx.angle.from_rad(x.acosh())),
            Function::Atanh => Ok(ctx.angle.from_rad(x.atanh())),
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
