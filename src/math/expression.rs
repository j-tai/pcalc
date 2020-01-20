use crate::math::{Context, Result};

/// An abstract syntax tree for a mathematical expression.
#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    /// A constant.
    Num(f64),
    /// Negation of an expression.
    Neg(Box<Expression>),

    /// Addition of two or more expressions.
    Add(Vec<Expression>),
    /// Subtraction of two expressions.
    Sub(Box<Expression>, Box<Expression>),
    /// Multipication of two or more expressions.
    Mul(Vec<Expression>),
    /// A fraction with a numerator expression and denominator expression.
    Frac(Box<Expression>, Box<Expression>),
    /// An exponent with a base expression and an exponent expression.
    Exp(Box<Expression>, Box<Expression>),
    /// A radical with a radicand expression and an index expression.
    Root(Box<Expression>, Box<Expression>),
    /// A logarithm with an expression and a base expression.
    Log(Box<Expression>, Box<Expression>),

    /// The mathematical constant pi.
    Pi,
    /// The mathematical constant e.
    E,

    /// The absolute value function.
    Abs(Box<Expression>),

    /// The sine function.
    Sin(Box<Expression>),
    /// The cosine function.
    Cos(Box<Expression>),
    /// The tangent function.
    Tan(Box<Expression>),
    /// The inverse sine function.
    Asin(Box<Expression>),
    /// The inverse cosine function.
    Acos(Box<Expression>),
    /// The inverse tangent function.
    Atan(Box<Expression>),

    /// The hyperbolic sine function.
    Sinh(Box<Expression>),
    /// The hyperbolic cosine function.
    Cosh(Box<Expression>),
    /// The hyperbolic tangent function.
    Tanh(Box<Expression>),
    /// The inverse hyperbolic sine function.
    Asinh(Box<Expression>),
    /// The inverse hyperbolic cosine function.
    Acosh(Box<Expression>),
    /// The inverse hyperbolic tangent function.
    Atanh(Box<Expression>),
}

impl Expression {
    /// Evaluate the expression.
    pub fn eval(&self, c: &Context) -> Result<f64> {
        use Expression::*;
        match self {
            Num(n) => Ok(*n),
            Neg(expr) => Ok(-expr.eval(c)?),
            Add(exprs) => exprs.iter().try_fold(0.0, |a, i| i.eval(c).map(|x| a + x)),
            Sub(lhs, rhs) => Ok(lhs.eval(c)? - rhs.eval(c)?),
            Mul(exprs) => exprs.iter().try_fold(1.0, |a, i| i.eval(c).map(|x| a * x)),
            Frac(num, den) => Ok(num.eval(c)? / den.eval(c)?),
            Exp(base, exp) => Ok(base.eval(c)?.powf(exp.eval(c)?)),
            Root(rad, idx) => Ok(rad.eval(c)?.powf(1.0 / idx.eval(c)?)),
            Log(num, base) => Ok(num.eval(c)?.log(base.eval(c)?)),
            Pi => Ok(std::f64::consts::PI),
            E => Ok(std::f64::consts::E),
            Abs(expr) => Ok(expr.eval(c)?.abs()),
            Sin(expr) => Ok(c.angle.to_rad(expr.eval(c)?).sin()),
            Cos(expr) => Ok(c.angle.to_rad(expr.eval(c)?).cos()),
            Tan(expr) => Ok(c.angle.to_rad(expr.eval(c)?).tan()),
            Asin(expr) => Ok(c.angle.from_rad(expr.eval(c)?.asin())),
            Acos(expr) => Ok(c.angle.from_rad(expr.eval(c)?.acos())),
            Atan(expr) => Ok(c.angle.from_rad(expr.eval(c)?.atan())),
            Sinh(expr) => Ok(c.angle.to_rad(expr.eval(c)?).sinh()),
            Cosh(expr) => Ok(c.angle.to_rad(expr.eval(c)?).cosh()),
            Tanh(expr) => Ok(c.angle.to_rad(expr.eval(c)?).tanh()),
            Asinh(expr) => Ok(c.angle.from_rad(expr.eval(c)?.asinh())),
            Acosh(expr) => Ok(c.angle.from_rad(expr.eval(c)?.acosh())),
            Atanh(expr) => Ok(c.angle.from_rad(expr.eval(c)?.atanh())),
        }
    }
}
