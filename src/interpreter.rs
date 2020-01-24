use crate::{Context, Error, Expression, Result, Span};

#[cfg(test)]
mod tests;

/// Evaluate the expression in the given context.
pub fn eval((expr, span): &(Expression, Span), c: &mut Context) -> Result<f64> {
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
        Func(f, expr) => f.apply(eval(expr, c)?, c).map_err(|e| (e, span.clone())),
        Var(var) => {
            if let Some(val) = c.vars.get(var.as_str()) {
                Ok(*val)
            } else {
                Err((Error::Undefined(var.to_string()), span.clone()))
            }
        }
        Let(var, expr) => {
            let x = eval(expr, c)?;
            c.vars.insert(var.clone(), x);
            Ok(x)
        }
        Comma(exprs) => {
            debug_assert!(!exprs.is_empty());
            let len = exprs.len();
            for expr in &exprs[..len - 1] {
                eval(expr, c)?;
            }
            eval(exprs.last().unwrap(), c)
        }
    }
}
