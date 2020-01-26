use std::convert::TryFrom;

use if_chain::if_chain;
use num::pow::checked_pow;
use num::traits::{CheckedAdd, CheckedSub, CheckedMul, CheckedDiv};
use num::rational::Ratio;

use crate::Value::*;
use crate::{Context, Error, Expression, Result, Span, Value};

#[cfg(test)]
mod tests;

type ExprSpan = (Expression, Span);

fn neg(expr: &ExprSpan, c: &mut Context) -> Result<Value> {
    match eval(expr, c)? {
        Float(f) => Ok((-f).into()),
        Ratio(r) => Ok((-r).into()),
    }
}

fn apply<F, G>(mut f: F, mut g: G, exprs: &[ExprSpan], c: &mut Context) -> Result<Value>
where
    F: FnMut(f64, f64) -> f64,
    G: FnMut(Ratio<i64>, Ratio<i64>) -> Option<Ratio<i64>>,
{
    debug_assert!(!exprs.is_empty());
    let mut acc = eval(&exprs[0], c)?;
    for expr in &exprs[1..] {
        let rhs = eval(expr, c)?;
        acc = match (&acc, &rhs) {
            (Ratio(a), Ratio(b)) => {
                match g(*a, *b) {
                    Some(x) => x.into(),
                    None => f(acc.to_f64(), rhs.to_f64()).into()
                }
            }
            (lhs, rhs) => f(lhs.to_f64(), rhs.to_f64()).into(),
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
        Float(f) => f.recip().into(),
        Ratio(r) => r.recip().into(),
    };
    do_exp(lhs, rhs)
}

fn do_exp(lhs: Value, rhs: Value) -> Result<Value> {
    if_chain! {
        if let Ratio(lhs) = lhs;
        if let Ratio(rhs) = rhs;
        if rhs.is_integer();
        if let Ok(rhs) = usize::try_from(*rhs.numer());
        if let Some(rhs) = checked_pow(lhs, rhs);
        then {
            return Ok(rhs.into());
        }
    }
    Ok(lhs.to_f64().powf(rhs.to_f64()).into())
}

/// Evaluate the expression in the given context.
pub fn eval((expr, span): &(Expression, Span), c: &mut Context) -> Result<Value> {
    use crate::Expression::*;
    match expr {
        Val(v) => Ok(v.clone()),
        Neg(expr) => neg(expr, c),
        Add(exprs) => apply(|a, b| a + b, |a, b| a.checked_add(&b), exprs, c),
        Sub(args) => apply(|a, b| a - b, |a, b| a.checked_sub(&b), &args[..], c),
        Mul(exprs) => apply(|a, b| a * b, |a, b| a.checked_mul(&b), exprs, c),
        Frac(args) => apply(|a, b| a / b, |a, b| a.checked_div(&b), &args[..], c),
        Exp(args) => eval_exp(&args[0], &args[1], c),
        Root(args) => eval_root(&args[0], &args[1], c),
        Log(args) => Ok(eval(&args[0], c)?
            .to_f64()
            .log(eval(&args[1], c)?.to_f64())
            .into()),
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
