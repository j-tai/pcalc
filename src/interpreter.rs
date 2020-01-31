use std::convert::TryFrom;

use if_chain::if_chain;
use num::pow::checked_pow;
use num::rational::Ratio;
use num::traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub};

use crate::Value::*;
use crate::{Context, Error, Expression, Result, Span, Value};

#[cfg(test)]
mod tests;

type ExprSpan = (Expression, Span);

fn neg(expr: &ExprSpan, c: &mut Context, span: &Span) -> Result<Value> {
    match eval(expr, c)? {
        Float(f) => Ok((-f).into()),
        Ratio(r) => Ok((-r).into()),
        _ => Err((Error::Type, span.clone())),
    }
}

/// Converts the value to a float.
fn to_f64(val: &Value, span: &Span) -> Result<f64> {
    match val {
        Value::Ratio(f) => Ok(*f.numer() as f64 / *f.denom() as f64),
        Value::Float(f) => Ok(*f),
        _ => Err((Error::Type, span.clone())),
    }
}

fn apply<F, G>(
    mut f: F,
    mut g: G,
    exprs: &[ExprSpan],
    c: &mut Context,
    span: &Span,
) -> Result<Value>
where
    F: FnMut(f64, f64) -> f64,
    G: FnMut(Ratio<i64>, Ratio<i64>) -> Option<Ratio<i64>>,
{
    debug_assert!(!exprs.is_empty());
    let mut acc = eval(&exprs[0], c)?;
    for expr in &exprs[1..] {
        let rhs = eval(expr, c)?;
        acc = match (&acc, &rhs) {
            (Ratio(left), Ratio(right)) => match g(*left, *right) {
                Some(x) => x.into(),
                None => f(to_f64(&acc, span)?, to_f64(&rhs, span)?).into(),
            },
            (lhs, rhs) => f(to_f64(&lhs, span)?, to_f64(&rhs, span)?).into(),
        };
    }
    Ok(acc)
}

fn eval_exp(lhs: &ExprSpan, rhs: &ExprSpan, c: &mut Context, span: &Span) -> Result<Value> {
    let lhs = eval(&lhs, c)?;
    let rhs = eval(&rhs, c)?;
    do_exp(lhs, rhs, span)
}

fn eval_root(lhs: &ExprSpan, rhs: &ExprSpan, c: &mut Context, span: &Span) -> Result<Value> {
    let lhs = eval(&lhs, c)?;
    let rhs = match eval(&rhs, c)? {
        Float(f) => f.recip().into(),
        Ratio(r) => r.recip().into(),
        _ => return Err((Error::Type, span.clone())),
    };
    do_exp(lhs, rhs, span)
}

fn do_exp(lhs: Value, rhs: Value, span: &Span) -> Result<Value> {
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
    Ok(to_f64(&lhs, span)?.powf(to_f64(&rhs, span)?).into())
}

/// Evaluate the expression in the given context.
pub fn eval((expr, span): &(Expression, Span), c: &mut Context) -> Result<Value> {
    use crate::Expression::*;
    match expr {
        Val(v) => Ok(v.clone()),
        Neg(expr) => neg(expr, c, span),
        Add(exprs) => apply(|a, b| a + b, |a, b| a.checked_add(&b), exprs, c, span),
        Sub(args) => apply(|a, b| a - b, |a, b| a.checked_sub(&b), &args[..], c, span),
        Mul(exprs) => apply(|a, b| a * b, |a, b| a.checked_mul(&b), exprs, c, span),
        Frac(args) => apply(|a, b| a / b, |a, b| a.checked_div(&b), &args[..], c, span),
        Exp(args) => eval_exp(&args[0], &args[1], c, span),
        Root(args) => eval_root(&args[0], &args[1], c, span),
        Const(con) => Ok(con.value()),
        Func(f, expr) => f.apply(eval(expr, c)?, c, span),
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
        Call(name, args) => {
            let args = args
                .iter()
                .map(|a| eval(a, c))
                .collect::<Result<Vec<_>>>()?;
            let (params, expr) = match c.vars.get(name) {
                Some(Value::Func(params, expr)) => (params, expr),
                Some(_) => return Err((Error::Type, span.clone())),
                None => return Err((Error::Undefined(name.to_string()), span.clone())),
            };
            if args.len() != params.len() {
                return Err((Error::Syntax, span.clone()));
            }
            let mut inner_ctx = Context::default();
            for (name, val) in params.iter().zip(args.into_iter()) {
                inner_ctx.vars.insert(name.to_string(), val);
            }
            eval(expr, &mut inner_ctx)
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
