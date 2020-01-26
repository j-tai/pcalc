use std::i32;

use num::pow::Pow;
use num::rational::Ratio;

use crate::Value::*;
use crate::{Context, Error, Expression, Result, Span, Value};

#[cfg(test)]
mod tests;

type ExprSpan = (Expression, Span);

fn neg(expr: &ExprSpan, c: &mut Context) -> Result<Value> {
    match eval(expr, c)? {
        Float(f) => Ok(Float(-f)),
        Ratio(r) => Ok(Ratio(-r)),
    }
}

fn apply<F, G>(mut f: F, mut g: G, exprs: &[ExprSpan], c: &mut Context) -> Result<Value>
where
    F: FnMut(f64, f64) -> f64,
    G: FnMut(Ratio<i64>, Ratio<i64>) -> Ratio<i64>,
{
    debug_assert!(!exprs.is_empty());
    let mut acc = eval(&exprs[0], c)?;
    for expr in &exprs[1..] {
        acc = match (acc, eval(expr, c)?) {
            (Ratio(lhs), Ratio(rhs)) => Ratio(g(lhs, rhs)),
            (lhs, rhs) => Float(f(lhs.to_f64(), rhs.to_f64())),
        };
    }
    Ok(acc)
}

fn eval_exp(lhs: &ExprSpan, rhs: &ExprSpan, c: &mut Context) -> Result<Value> {
    let lhs = eval(&lhs, c)?;
    let rhs = eval(&rhs, c)?;
    do_exp(lhs, rhs)
}

fn eval_root(lhs: &ExprSpan, rhs: &ExprSpan, c: &mut Context) -> Result<Value> {
    let lhs = eval(&lhs, c)?;
    let rhs = match eval(&rhs, c)? {
        Float(f) => Float(f.recip()),
        Ratio(r) => Ratio(r.recip()),
    };
    do_exp(lhs, rhs)
}

fn do_exp(lhs: Value, rhs: Value) -> Result<Value> {
    match (lhs, rhs) {
        (Ratio(lhs), Ratio(rhs)) if rhs.is_integer() && *rhs.numer() < i32::MAX.into() => {
            Ok(Ratio(lhs.pow(*rhs.numer() as i32)))
        }
        (lhs, rhs) => Ok(Float(lhs.to_f64().powf(rhs.to_f64()))),
    }
}

/// Evaluate the expression in the given context.
pub fn eval((expr, span): &(Expression, Span), c: &mut Context) -> Result<Value> {
    use crate::Expression::*;
    match expr {
        Val(v) => Ok(v.clone()),
        Neg(expr) => neg(expr, c),
        Add(exprs) => apply(|a, b| a + b, |a, b| a + b, exprs, c),
        Sub(args) => apply(|a, b| a - b, |a, b| a - b, &args[..], c),
        Mul(exprs) => apply(|a, b| a * b, |a, b| a * b, exprs, c),
        Frac(args) => apply(|a, b| a / b, |a, b| a / b, &args[..], c),
        Exp(args) => eval_exp(&args[0], &args[1], c),
        Root(args) => eval_root(&args[0], &args[1], c),
        Log(args) => Ok(Float(
            eval(&args[0], c)?.to_f64().log(eval(&args[1], c)?.to_f64()),
        )),
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
