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
        Const(con) => Ok(con.value()),
        Func(f, expr) => Ok(f.apply(eval(expr, c)?, c)?),
    }
}
