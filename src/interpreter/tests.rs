use crate::Expression::*;
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
    let x = (Num(4.2), sp());
    assert_eq!(eval(&x, &mut ctx()), Ok(4.2));
    let x = (Num(42.185), sp());
    assert_eq!(eval(&x, &mut ctx()), Ok(42.185));
}

#[test]
fn add() {
    let x = (Add(vec![(Num(5.0), sp()), (Num(3.0), sp())]), sp());
    assert_eq!(eval(&x, &mut ctx()), Ok(8.0));
    let x = (
        Add(vec![
            (Num(0.0), sp()),
            (Num(1.0), sp()),
            (Num(2.0), sp()),
            (Num(3.0), sp()),
        ]),
        sp(),
    );
    assert_eq!(eval(&x, &mut ctx()), Ok(6.0));
}

#[test]
fn sub() {
    let x = (Sub(Box::new([(Num(5.0), sp()), (Num(3.0), sp())])), sp());
    assert_eq!(eval(&x, &mut ctx()), Ok(2.0));
}

#[test]
fn mul() {
    let x = (Mul(vec![(Num(5.0), sp()), (Num(3.0), sp())]), sp());
    assert_eq!(eval(&x, &mut ctx()), Ok(15.0));
    let x = (
        Mul(vec![
            (Num(1.0), sp()),
            (Num(2.0), sp()),
            (Num(3.0), sp()),
            (Num(4.0), sp()),
        ]),
        sp(),
    );
    assert_eq!(eval(&x, &mut ctx()), Ok(24.0));
}

#[test]
fn frac() {
    let x = (Frac(Box::new([(Num(5.0), sp()), (Num(2.0), sp())])), sp());
    assert_eq!(eval(&x, &mut ctx()), Ok(2.5));
}

#[test]
fn exp() {
    let x = (Exp(Box::new([(Num(3.0), sp()), (Num(2.0), sp())])), sp());
    assert_eq!(eval(&x, &mut ctx()), Ok(9.0));
    let x = (Exp(Box::new([(Num(2.0), sp()), (Num(3.0), sp())])), sp());
    assert_eq!(eval(&x, &mut ctx()), Ok(8.0));
}

#[test]
fn root() {
    let x = (Root(Box::new([(Num(9.0), sp()), (Num(2.0), sp())])), sp());
    assert_eq!(eval(&x, &mut ctx()), Ok(3.0));
    let x = (Root(Box::new([(Num(8.0), sp()), (Num(3.0), sp())])), sp());
    assert_eq!(eval(&x, &mut ctx()), Ok(2.0));
}

#[test]
fn log() {
    let x = (Log(Box::new([(Num(9.0), sp()), (Num(3.0), sp())])), sp());
    assert_eq!(eval(&x, &mut ctx()), Ok(2.0));
    let x = (Log(Box::new([(Num(8.0), sp()), (Num(2.0), sp())])), sp());
    assert_eq!(eval(&x, &mut ctx()), Ok(3.0));
}

#[test]
fn const_pi() {
    let x = (Const(Constant::Pi), sp());
    assert_eq!(eval(&x, &mut ctx()), Ok(std::f64::consts::PI));
}

#[test]
fn const_e() {
    let x = (Const(Constant::E), sp());
    assert_eq!(eval(&x, &mut ctx()), Ok(std::f64::consts::E));
}

#[test]
fn var() {
    let mut ctx = ctx();
    ctx.vars.insert("x".to_string(), 1.5);
    ctx.vars.insert("foo".to_string(), 2.5);
    let x = (
        Add(vec![
            (Var("x".to_string()), sp()),
            (Var("foo".to_string()), sp()),
        ]),
        sp(),
    );
    assert_eq!(eval(&x, &mut ctx), Ok(4.0));
}

#[test]
fn r#let() {
    let mut ctx = ctx();
    let x = (Let("foo".to_string(), Box::new((Num(2.5), sp()))), sp());
    assert_eq!(eval(&x, &mut ctx), Ok(2.5));
    assert_eq!(ctx.vars.get("foo"), Some(&2.5));
}

#[test]
fn comma() {
    let mut ctx = ctx();
    let x = (
        Comma(vec![(Num(1.0), sp()), (Num(2.0), sp()), (Num(3.0), sp())]),
        sp(),
    );
    assert_eq!(eval(&x, &mut ctx), Ok(3.0));
}
