//! Integration tests.

use std::f64::consts;

use crate::{eval, lex, parse, Result, Value};

fn try_ev(s: &str) -> Result<Value> {
    let tokens = lex(s, None);
    let expr = parse(tokens)?;
    let mut ctx = Default::default();
    eval(&expr, &mut ctx)
}

fn ev(s: &str) -> Value {
    try_ev(s).expect("Eval failed")
}

#[test]
fn basic_int() {
    assert_eq!(ev("2+2"), 4.into());
    assert_eq!(ev("3-2"), 1.into());
    assert_eq!(ev("4*4"), 16.into());
    assert_eq!(ev("6/3"), 2.into());
    assert_eq!(ev("6/4"), (3, 2).into());
    assert_eq!(ev("2^3"), 8.into());
    assert_eq!(ev("5,4"), 4.into());
}

#[test]
fn repeated_ops() {
    assert_eq!(ev("2+2+1"), 5.into());
    assert_eq!(ev("3-2-3"), (-2).into());
    assert_eq!(ev("4*4*5"), 80.into());
    assert_eq!(ev("6/3/2"), 1.into());
    assert_eq!(ev("6/4/2"), (3, 4).into());
    assert_eq!(ev("2^3^2"), 512.into());
    assert_eq!(ev("5,4,3"), 3.into());
}

#[test]
fn vars() {
    assert_eq!(ev("x=3, x"), 3.into());
    assert_eq!(ev("x=4, x*x"), 16.into());
    assert_eq!(ev("x=5, x=6, x=7, x+8"), 15.into());
    assert_eq!(ev("x=4, y=5, x+y"), 9.into());
    assert_eq!(ev("x=3+3, x+x"), 12.into());
    assert_eq!(ev("x=3+3, y=5*5, y-x"), 19.into());
}

#[test]
fn basic_float() {
    assert_eq!(ev("2.0+2.0"), 4.0.into());
    assert_eq!(ev("3.0-2.0"), 1.0.into());
    assert_eq!(ev("4.0*4.0"), 16.0.into());
    assert_eq!(ev("6.0/3.0"), 2.0.into());
    assert_eq!(ev("6.0/4.0"), 1.5.into());
}

#[test]
fn builtin_const() {
    assert_eq!(ev("pi"), consts::PI.into());
    assert_eq!(ev("pi*2"), (consts::PI * 2.0).into());
    assert_eq!(ev("e"), consts::E.into());
}

#[test]
fn builtin_func() {
    assert_eq!(ev("sin(0)"), 0.0.into());
    assert_eq!(ev("sin(pi/2)"), 1.0.into());
    assert_eq!(ev("cos(0.0)"), 1.0.into());
    assert_eq!(ev("cos(pi)"), (-1.0).into());
    assert_eq!(ev("tan(0)"), 0.0.into());
    assert_eq!(ev("asin(0)"), 0.0.into());
    assert_eq!(ev("acos(1.0)"), 0.0.into());
    assert_eq!(ev("atan(1.0)"), (consts::PI / 4.0).into());
}
