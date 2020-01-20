use crate::{Context, Expression, Result};

#[cfg(test)]
mod tests;

/// Evaluate the expression.
pub fn eval<'a>(expr: &'a Expression, c: &Context) -> Result<'a, f64> {
    use crate::Expression::*;
    match expr {
        Num(n) => Ok(*n),
        Neg(expr) => Ok(-eval(expr, c)?),
        Add(exprs) => exprs.iter().try_fold(0.0, |a, i| eval(i, c).map(|x| a + x)),
        Sub(lhs, rhs) => Ok(eval(lhs, c)? - eval(rhs, c)?),
        Mul(exprs) => exprs.iter().try_fold(1.0, |a, i| eval(i, c).map(|x| a * x)),
        Frac(num, den) => Ok(eval(num, c)? / eval(den, c)?),
        Exp(base, exp) => Ok(eval(base, c)?.powf(eval(exp, c)?)),
        Root(rad, idx) => Ok(eval(rad, c)?.powf(1.0 / eval(idx, c)?)),
        Log(num, base) => Ok(eval(num, c)?.log(eval(base, c)?)),
        Pi => Ok(std::f64::consts::PI),
        E => Ok(std::f64::consts::E),
        Abs(expr) => Ok(eval(expr, c)?.abs()),
        Sin(expr) => Ok(c.angle.to_rad(eval(expr, c)?).sin()),
        Cos(expr) => Ok(c.angle.to_rad(eval(expr, c)?).cos()),
        Tan(expr) => Ok(c.angle.to_rad(eval(expr, c)?).tan()),
        Asin(expr) => Ok(c.angle.from_rad(eval(expr, c)?.asin())),
        Acos(expr) => Ok(c.angle.from_rad(eval(expr, c)?.acos())),
        Atan(expr) => Ok(c.angle.from_rad(eval(expr, c)?.atan())),
        Sinh(expr) => Ok(c.angle.to_rad(eval(expr, c)?).sinh()),
        Cosh(expr) => Ok(c.angle.to_rad(eval(expr, c)?).cosh()),
        Tanh(expr) => Ok(c.angle.to_rad(eval(expr, c)?).tanh()),
        Asinh(expr) => Ok(c.angle.from_rad(eval(expr, c)?.asinh())),
        Acosh(expr) => Ok(c.angle.from_rad(eval(expr, c)?.acosh())),
        Atanh(expr) => Ok(c.angle.from_rad(eval(expr, c)?.atanh())),
    }
}
