use std::f64::consts;

use crate::Expression::*;
use crate::Value::*;
use crate::{eval, Constant, Context, Span};

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
    let x = (Val(Float(4.2)), sp());
    assert_eq!(eval(&x, &mut ctx()), Ok(Float(4.2)));
    let x = (Val(Float(42.185)), sp());
    assert_eq!(eval(&x, &mut ctx()), Ok(Float(42.185)));
}

#[test]
fn add() {
    let x = (
        Add(vec![(Val(Float(5.0)), sp()), (Val(Float(3.0)), sp())]),
        sp(),
    );
    assert_eq!(eval(&x, &mut ctx()), Ok(Float(8.0)));
    let x = (
        Add(vec![
            (Val(Float(0.0)), sp()),
            (Val(Float(1.0)), sp()),
            (Val(Float(2.0)), sp()),
            (Val(Float(3.0)), sp()),
        ]),
        sp(),
    );
    assert_eq!(eval(&x, &mut ctx()), Ok(Float(6.0)));
}

#[test]
fn sub() {
    let x = (
        Sub(Box::new([(Val(Float(5.0)), sp()), (Val(Float(3.0)), sp())])),
        sp(),
    );
    assert_eq!(eval(&x, &mut ctx()), Ok(Float(2.0)));
}

#[test]
fn mul() {
    let x = (
        Mul(vec![(Val(Float(5.0)), sp()), (Val(Float(3.0)), sp())]),
        sp(),
    );
    assert_eq!(eval(&x, &mut ctx()), Ok(Float(15.0)));
    let x = (
        Mul(vec![
            (Val(Float(1.0)), sp()),
            (Val(Float(2.0)), sp()),
            (Val(Float(3.0)), sp()),
            (Val(Float(4.0)), sp()),
        ]),
        sp(),
    );
    assert_eq!(eval(&x, &mut ctx()), Ok(Float(24.0)));
}

#[test]
fn frac() {
    let x = (
        Frac(Box::new([(Val(Float(5.0)), sp()), (Val(Float(2.0)), sp())])),
        sp(),
    );
    assert_eq!(eval(&x, &mut ctx()), Ok(Float(2.5)));
}

#[test]
fn exp() {
    let x = (
        Exp(Box::new([(Val(Float(3.0)), sp()), (Val(Float(2.0)), sp())])),
        sp(),
    );
    assert_eq!(eval(&x, &mut ctx()), Ok(Float(9.0)));
    let x = (
        Exp(Box::new([(Val(Float(2.0)), sp()), (Val(Float(3.0)), sp())])),
        sp(),
    );
    assert_eq!(eval(&x, &mut ctx()), Ok(Float(8.0)));
}

#[test]
fn root() {
    let x = (
        Root(Box::new([(Val(Float(9.0)), sp()), (Val(Float(2.0)), sp())])),
        sp(),
    );
    assert_eq!(eval(&x, &mut ctx()), Ok(Float(3.0)));
    let x = (
        Root(Box::new([(Val(Float(8.0)), sp()), (Val(Float(3.0)), sp())])),
        sp(),
    );
    assert_eq!(eval(&x, &mut ctx()), Ok(Float(2.0)));
}

#[test]
fn log() {
    let x = (
        Log(Box::new([(Val(Float(9.0)), sp()), (Val(Float(3.0)), sp())])),
        sp(),
    );
    assert_eq!(eval(&x, &mut ctx()), Ok(Float(2.0)));
    let x = (
        Log(Box::new([(Val(Float(8.0)), sp()), (Val(Float(2.0)), sp())])),
        sp(),
    );
    assert_eq!(eval(&x, &mut ctx()), Ok(Float(3.0)));
}

#[test]
fn const_pi() {
    let x = (Const(Constant::Pi), sp());
    assert_eq!(eval(&x, &mut ctx()), Ok(Float(consts::PI)));
}

#[test]
fn const_e() {
    let x = (Const(Constant::E), sp());
    assert_eq!(eval(&x, &mut ctx()), Ok(Float(consts::E)));
}

#[test]
fn var() {
    let mut ctx = ctx();
    ctx.vars.insert("x".to_string(), Float(1.5));
    ctx.vars.insert("foo".to_string(), Float(2.5));
    let x = (
        Add(vec![
            (Var("x".to_string()), sp()),
            (Var("foo".to_string()), sp()),
        ]),
        sp(),
    );
    assert_eq!(eval(&x, &mut ctx), Ok(Float(4.0)));
}

#[test]
fn r#let() {
    let mut ctx = ctx();
    let x = (
        Let("foo".to_string(), Box::new((Val(Float(2.5)), sp()))),
        sp(),
    );
    assert_eq!(eval(&x, &mut ctx), Ok(Float(2.5)));
    assert_eq!(ctx.vars.get("foo"), Some(&Float(2.5)));
}

#[test]
fn comma() {
    let mut ctx = ctx();
    let x = (
        Comma(vec![
            (Val(Float(1.0)), sp()),
            (Val(Float(2.0)), sp()),
            (Val(Float(3.0)), sp()),
        ]),
        sp(),
    );
    assert_eq!(eval(&x, &mut ctx), Ok(Float(3.0)));
}
