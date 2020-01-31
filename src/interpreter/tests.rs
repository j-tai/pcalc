use std::f64::consts;
use std::i64;

use crate::Expression::*;
use crate::{eval, Constant, Context, Error, Span, Value};

fn sp() -> Span {
    Span {
        file: None,
        line: 1,
        start: 1,
        end: 1,
    }
}

fn ctx() -> Context {
    Context::default()
}

#[test]
fn num() {
    let x = (4.2.into(), sp());
    assert_eq!(eval(&x, &mut ctx()), Ok(4.2.into()));
    let x = (42.185.into(), sp());
    assert_eq!(eval(&x, &mut ctx()), Ok(42.185.into()));
    let x = (55.into(), sp());
    assert_eq!(eval(&x, &mut ctx()), Ok(55.into()));
}

#[test]
fn add() {
    let x = (Add(vec![(5.0.into(), sp()), (3.0.into(), sp())]), sp());
    assert_eq!(eval(&x, &mut ctx()), Ok(8.0.into()));
    let x = (
        Add(vec![
            (0.0.into(), sp()),
            (1.0.into(), sp()),
            (2.0.into(), sp()),
            (3.0.into(), sp()),
        ]),
        sp(),
    );
    assert_eq!(eval(&x, &mut ctx()), Ok(6.0.into()));
    let x = (Add(vec![(3.into(), sp()), (5.into(), sp())]), sp());
    assert_eq!(eval(&x, &mut ctx()), Ok(8.into()));
}

#[test]
fn sub() {
    let x = (
        Sub(Box::new([(5.0.into(), sp()), (3.0.into(), sp())])),
        sp(),
    );
    assert_eq!(eval(&x, &mut ctx()), Ok(2.0.into()));
    let x = (Sub(Box::new([(5.into(), sp()), (3.into(), sp())])), sp());
    assert_eq!(eval(&x, &mut ctx()), Ok(2.into()));
}

#[test]
fn mul() {
    let x = (Mul(vec![(5.0.into(), sp()), (3.0.into(), sp())]), sp());
    assert_eq!(eval(&x, &mut ctx()), Ok(15.0.into()));
    let x = (
        Mul(vec![
            (1.0.into(), sp()),
            (2.0.into(), sp()),
            (3.0.into(), sp()),
            (4.0.into(), sp()),
        ]),
        sp(),
    );
    assert_eq!(eval(&x, &mut ctx()), Ok(24.0.into()));
    let x = (Mul(vec![(5.into(), sp()), (3.into(), sp())]), sp());
    assert_eq!(eval(&x, &mut ctx()), Ok(15.into()));
}

#[test]
fn frac() {
    let x = (
        Frac(Box::new([(5.0.into(), sp()), (2.0.into(), sp())])),
        sp(),
    );
    assert_eq!(eval(&x, &mut ctx()), Ok(2.5.into()));
    let x = (Frac(Box::new([(5.into(), sp()), (2.into(), sp())])), sp());
    assert_eq!(eval(&x, &mut ctx()), Ok((5, 2).into()));
}

#[test]
fn exp() {
    let x = (
        Exp(Box::new([(3.0.into(), sp()), (2.0.into(), sp())])),
        sp(),
    );
    assert_eq!(eval(&x, &mut ctx()), Ok(9.0.into()));
    let x = (
        Exp(Box::new([(2.0.into(), sp()), (3.0.into(), sp())])),
        sp(),
    );
    assert_eq!(eval(&x, &mut ctx()), Ok(8.0.into()));
    let x = (Exp(Box::new([(2.into(), sp()), (3.into(), sp())])), sp());
    assert_eq!(eval(&x, &mut ctx()), Ok(8.into()));
}

#[test]
fn root() {
    let x = (
        Root(Box::new([(9.0.into(), sp()), (2.0.into(), sp())])),
        sp(),
    );
    assert_eq!(eval(&x, &mut ctx()), Ok(3.0.into()));
    let x = (
        Root(Box::new([(8.0.into(), sp()), (3.0.into(), sp())])),
        sp(),
    );
    assert_eq!(eval(&x, &mut ctx()), Ok(2.0.into()));
    // 8 root 3 == 2.0
    let x = (Root(Box::new([(8.into(), sp()), (3.into(), sp())])), sp());
    assert_eq!(eval(&x, &mut ctx()), Ok(2.0.into()));
    // 8 root (1/2) = 64 (not 64.0)
    let x = (
        Root(Box::new([(8.into(), sp()), ((1, 2).into(), sp())])),
        sp(),
    );
    assert_eq!(eval(&x, &mut ctx()), Ok(64.into()));
}

#[test]
fn const_pi() {
    let x = (Const(Constant::Pi), sp());
    assert_eq!(eval(&x, &mut ctx()), Ok(consts::PI.into()));
}

#[test]
fn const_e() {
    let x = (Const(Constant::E), sp());
    assert_eq!(eval(&x, &mut ctx()), Ok(consts::E.into()));
}

#[test]
fn var() {
    let mut ctx = ctx();
    ctx.vars.insert("x".to_string(), 1.5.into());
    ctx.vars.insert("foo".to_string(), 2.into());
    let x = (
        Add(vec![
            (Var("x".to_string()), sp()),
            (Var("foo".to_string()), sp()),
        ]),
        sp(),
    );
    assert_eq!(eval(&x, &mut ctx), Ok(3.5.into()));
}

#[test]
fn r#let() {
    let mut ctx = ctx();
    let x = (Let("foo".to_string(), Box::new((2.5.into(), sp()))), sp());
    assert_eq!(eval(&x, &mut ctx), Ok(2.5.into()));
    assert_eq!(ctx.vars.get("foo"), Some(&2.5.into()));
}

#[test]
fn comma() {
    let x = (
        Comma(vec![
            (1.0.into(), sp()),
            (2.into(), sp()),
            (3.0.into(), sp()),
        ]),
        sp(),
    );
    assert_eq!(eval(&x, &mut ctx()), Ok(3.0.into()));
}

#[test]
fn int_overflow() {
    // i64::MAX + 1 => float
    let x = (Add(vec![(i64::MAX.into(), sp()), (1.into(), sp())]), sp());
    assert_eq!(eval(&x, &mut ctx()), Ok((i64::MAX as f64 + 1.0).into()));
    // i64::MIN - 1 => float
    let x = (
        Sub(Box::new([(i64::MIN.into(), sp()), (1.into(), sp())])),
        sp(),
    );
    assert_eq!(eval(&x, &mut ctx()), Ok((i64::MIN as f64 - 1.0).into()));
    // i64::MAX * 2 => float
    let x = (Mul(vec![(i64::MAX.into(), sp()), (2.into(), sp())]), sp());
    assert_eq!(eval(&x, &mut ctx()), Ok((i64::MAX as f64 * 2.0).into()));
    // i64::MAX / (1/2) => float
    let x = (
        Frac(Box::new([(i64::MAX.into(), sp()), ((1, 2).into(), sp())])),
        sp(),
    );
    assert_eq!(eval(&x, &mut ctx()), Ok((i64::MAX as f64 * 2.0).into()));
    // 2^100 => float
    let x = (Exp(Box::new([(2.into(), sp()), (100.into(), sp())])), sp());
    assert_eq!(eval(&x, &mut ctx()), Ok((2.0_f64.powf(100.0)).into()));
    // (1/100) root 2 => float
    let x = (
        Root(Box::new([(2.into(), sp()), ((1, 100).into(), sp())])),
        sp(),
    );
    assert_eq!(eval(&x, &mut ctx()), Ok((2.0_f64.powf(100.0)).into()));
}

#[test]
fn invalid_types() {
    let func = || (Value::Func(vec![], Box::new((0.into(), sp()))).into(), sp());
    let zero = || (0.into(), sp());
    let x = (Add(vec![func(), zero()]), sp());
    assert_eq!(eval(&x, &mut ctx()), Err((Error::Type, sp())));
    let x = (Sub(Box::new([zero(), func()])), sp());
    assert_eq!(eval(&x, &mut ctx()), Err((Error::Type, sp())));
    let x = (Mul(vec![zero(), func()]), sp());
    assert_eq!(eval(&x, &mut ctx()), Err((Error::Type, sp())));
    let x = (Frac(Box::new([func(), zero()])), sp());
    assert_eq!(eval(&x, &mut ctx()), Err((Error::Type, sp())));
    let x = (Exp(Box::new([func(), zero()])), sp());
    assert_eq!(eval(&x, &mut ctx()), Err((Error::Type, sp())));
    let x = (Root(Box::new([zero(), func()])), sp());
    assert_eq!(eval(&x, &mut ctx()), Err((Error::Type, sp())));
}

#[test]
fn call() {
    let func1 = Value::Func(
        vec!["foo".to_string()],
        Box::new((
            Add(vec![(Var("foo".to_string()), sp()), (1.into(), sp())]),
            sp(),
        )),
    );
    let mut c = ctx();
    c.vars.insert("increment".to_string(), func1);
    let x = (Call("increment".to_string(), vec![(4.into(), sp())]), sp());
    assert_eq!(eval(&x, &mut c), Ok(5.into()));
}
