use crate::Value::*;
use crate::{Context, Error, Expression, Result, Span, Value};

#[cfg(test)]
mod tests;

type ExprSpan = (Expression, Span);

fn neg(expr: &ExprSpan, c: &mut Context) -> Result<Value> {
    match eval(expr, c)? {
        Float(f) => Ok(Float(-f)),
    }
}

fn apply<F>(mut f: F, exprs: &[ExprSpan], c: &mut Context) -> Result<Value>
where
    F: FnMut(f64, f64) -> f64,
{
    debug_assert!(!exprs.is_empty());
    let mut acc = eval(&exprs[0], c)?;
    for expr in &exprs[1..] {
        acc = match (acc, eval(expr, c)?) {
            (Float(lhs), Float(rhs)) => Float(f(lhs, rhs)),
        };
    }
    Ok(acc)
}

/// Evaluate the expression in the given context.
pub fn eval((expr, span): &(Expression, Span), c: &mut Context) -> Result<Value> {
    use crate::Expression::*;

    // macro_rules! apply_op {
    //     ($op:expr, to: $lhs:expr, $rhs:expr) => {{
    //         let lhs = eval($lhs, c)?;
    //         let rhs = eval($rhs, c)?;
    //         match (lhs, rhs) {
    //             (Value::Float(lhs), Value::Float(rhs)) => Ok(Value::Float($op(lhs, rhs))),
    //         }
    //     }};
    //     ($op:expr, init: $acc:expr, to: $exprs:expr) => {{
    //         let mut acc: Value = $acc;
    //         for expr in $exprs.iter() {
    //             let val = eval(expr, c)?;
    //             acc = match (acc, val) {
    //                 (Value::Float(lhs), Value::Float(rhs)) => Value::Float($op(lhs, rhs)),
    //             };
    //         }
    //         Ok(acc)
    //     }}
    // }

    match expr {
        Val(v) => Ok(v.clone()),
        Neg(expr) => neg(expr, c),
        Add(exprs) => apply(|a, b| a + b, exprs, c),
        Sub(args) => apply(|a, b| a - b, &args[..], c),
        Mul(exprs) => apply(|a, b| a * b, exprs, c),
        Frac(args) => apply(|a, b| a / b, &args[..], c),
        Exp(args) => apply(|a, b| a.powf(b), &args[..], c),
        Root(args) => apply(|a, b| a.powf(1.0 / b), &args[..], c),
        Log(args) => apply(|a, b| a.log(b), &args[..], c),
        Const(con) => Ok(con.value()),
        Func(f, expr) => f.apply(eval(expr, c)?, c).map_err(|e| (e, span.clone())),
        Var(var) => {
            if let Some(val) = c.vars.get(var.as_str()) {
                Ok(val.clone())
            } else {
                Err((Error::Undefined(var.to_string()), span.clone()))
            }
        }
        Let(var, expr) => {
            let x = eval(expr, c)?;
            c.vars.insert(var.clone(), x.clone());
            Ok(x)
        }
        Comma(exprs) => apply(|_, b| b, exprs, c),
    }
}
