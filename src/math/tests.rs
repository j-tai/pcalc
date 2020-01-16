use crate::math::Expression::*;
use crate::math::*;

#[test]
fn eval_num() {
    let x = Num(4.2);
    assert_eq!(x.eval(&Context::default()), Ok(4.2));
    let x = Num(42.185);
    assert_eq!(x.eval(&Context::default()), Ok(42.185));
}

#[test]
fn eval_add() {
    let x = Add(vec![Num(5.0), Num(3.0)]);
    assert_eq!(x.eval(&Context::default()), Ok(8.0));
    let x = Add(vec![Num(0.0), Num(1.0), Num(2.0), Num(3.0)]);
    assert_eq!(x.eval(&Context::default()), Ok(6.0));
}

#[test]
fn eval_sub() {
    let x = Sub(Box::new(Num(5.0)), Box::new(Num(3.0)));
    assert_eq!(x.eval(&Context::default()), Ok(2.0));
}

#[test]
fn eval_mul() {
    let x = Mul(vec![Num(5.0), Num(3.0)]);
    assert_eq!(x.eval(&Context::default()), Ok(15.0));
    let x = Mul(vec![Num(1.0), Num(2.0), Num(3.0), Num(4.0)]);
    assert_eq!(x.eval(&Context::default()), Ok(24.0));
}

#[test]
fn eval_frac() {
    let x = Frac(Box::new(Num(5.0)), Box::new(Num(2.0)));
    assert_eq!(x.eval(&Context::default()), Ok(2.5));
}

#[test]
fn eval_exp() {
    let x = Exp(Box::new(Num(3.0)), Box::new(Num(2.0)));
    assert_eq!(x.eval(&Context::default()), Ok(9.0));
    let x = Exp(Box::new(Num(2.0)), Box::new(Num(3.0)));
    assert_eq!(x.eval(&Context::default()), Ok(8.0));
}

#[test]
fn eval_root() {
    let x = Root(Box::new(Num(9.0)), Box::new(Num(2.0)));
    assert_eq!(x.eval(&Context::default()), Ok(3.0));
    let x = Root(Box::new(Num(8.0)), Box::new(Num(3.0)));
    assert_eq!(x.eval(&Context::default()), Ok(2.0));
}

#[test]
fn eval_log() {
    let x = Log(Box::new(Num(9.0)), Box::new(Num(3.0)));
    assert_eq!(x.eval(&Context::default()), Ok(2.0));
    let x = Log(Box::new(Num(8.0)), Box::new(Num(2.0)));
    assert_eq!(x.eval(&Context::default()), Ok(3.0));
}

#[test]
fn eval_pi() {
    let x = Pi;
    assert_eq!(x.eval(&Context::default()), Ok(std::f64::consts::PI));
}

#[test]
fn eval_e() {
    let x = E;
    assert_eq!(x.eval(&Context::default()), Ok(std::f64::consts::E));
}
