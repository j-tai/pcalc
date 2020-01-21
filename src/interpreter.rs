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
        Sub(args) => Ok(eval(&args[0], c)? - eval(&args[1], c)?),
        Mul(exprs) => exprs.iter().try_fold(1.0, |a, i| eval(i, c).map(|x| a * x)),
        Frac(args) => Ok(eval(&args[0], c)? / eval(&args[1], c)?),
        Exp(args) => Ok(eval(&args[0], c)?.powf(eval(&args[1], c)?)),
        Root(args) => Ok(eval(&args[0], c)?.powf(1.0 / eval(&args[1], c)?)),
        Log(args) => Ok(eval(&args[0], c)?.log(eval(&args[1], c)?)),
        Const(con) => Ok(con.value()),
        Func(f, expr) => Ok(f.apply(eval(expr, c)?, c)?),
    }
}
