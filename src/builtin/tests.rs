use std::f64::consts;

use crate::Constant::*;
use crate::Function::*;
use crate::Value::*;
use crate::{AngleMeasure, Context, Span};

fn sp() -> Span {
    Span {
        file: None,
        line: 1,
        start: 1,
        end: 1,
    }
}

// Constants

#[test]
fn pi() {
    assert_eq!(Pi.value(), Float(consts::PI));
}

#[test]
fn e() {
    assert_eq!(E.value(), Float(consts::E));
}

// Functions

fn rad() -> Context {
    Default::default()
}

fn deg() -> Context {
    Context {
        angle: AngleMeasure::Degrees,
        ..Default::default()
    }
}

#[test]
fn abs() {
    assert_eq!(Abs.apply((-1.6).into(), &rad(), &sp()), Ok(1.6.into()));
    assert_eq!(Abs.apply(3.1.into(), &rad(), &sp()), Ok(3.1.into()));
}

// Trig functions in radians mode

#[test]
fn sin() {
    assert_eq!(
        Sin.apply(0.31.into(), &rad(), &sp()),
        Ok(0.31_f64.sin().into())
    );
}

#[test]
fn cos() {
    assert_eq!(
        Cos.apply(0.31.into(), &rad(), &sp()),
        Ok(0.31_f64.cos().into())
    );
}

#[test]
fn tan() {
    assert_eq!(
        Tan.apply(0.31.into(), &rad(), &sp()),
        Ok(0.31_f64.tan().into())
    );
}

#[test]
fn asin() {
    assert_eq!(
        Asin.apply(0.31.into(), &rad(), &sp()),
        Ok(0.31_f64.asin().into())
    );
}

#[test]
fn acos() {
    assert_eq!(
        Acos.apply(0.31.into(), &rad(), &sp()),
        Ok(0.31_f64.acos().into())
    );
}

#[test]
fn atan() {
    assert_eq!(
        Atan.apply(0.31.into(), &rad(), &sp()),
        Ok(0.31_f64.atan().into())
    );
}

// Trig functions in degrees

#[test]
fn sin_deg() {
    assert_eq!(
        Sin.apply(31.0.into(), &deg(), &sp()),
        Ok(31.0_f64.to_radians().sin().into())
    );
}

#[test]
fn cos_deg() {
    assert_eq!(
        Cos.apply(31.0.into(), &deg(), &sp()),
        Ok(31.0_f64.to_radians().cos().into())
    );
}

#[test]
fn tan_deg() {
    assert_eq!(
        Tan.apply(31.0.into(), &deg(), &sp()),
        Ok(31.0_f64.to_radians().tan().into())
    );
}

#[test]
fn asin_deg() {
    assert_eq!(
        Asin.apply(0.31.into(), &deg(), &sp()),
        Ok(0.31_f64.asin().to_degrees().into())
    );
}

#[test]
fn acos_deg() {
    assert_eq!(
        Acos.apply(0.31.into(), &deg(), &sp()),
        Ok(0.31_f64.acos().to_degrees().into())
    );
}

#[test]
fn atan_deg() {
    assert_eq!(
        Atan.apply(0.31.into(), &deg(), &sp()),
        Ok(0.31_f64.atan().to_degrees().into())
    );
}

// Hyperbolic functions

#[test]
fn sinh() {
    assert_eq!(
        Sinh.apply(0.31.into(), &rad(), &sp()),
        Ok(0.31_f64.sinh().into())
    );
    assert_eq!(
        Sinh.apply(0.31.into(), &deg(), &sp()),
        Ok(0.31_f64.sinh().into())
    );
}

#[test]
fn cosh() {
    assert_eq!(
        Cosh.apply(0.31.into(), &rad(), &sp()),
        Ok(0.31_f64.cosh().into())
    );
    assert_eq!(
        Cosh.apply(0.31.into(), &deg(), &sp()),
        Ok(0.31_f64.cosh().into())
    );
}

#[test]
fn tanh() {
    assert_eq!(
        Tanh.apply(0.31.into(), &rad(), &sp()),
        Ok(0.31_f64.tanh().into())
    );
    assert_eq!(
        Tanh.apply(0.31.into(), &deg(), &sp()),
        Ok(0.31_f64.tanh().into())
    );
}

#[test]
fn asinh() {
    assert_eq!(
        Asinh.apply(0.31.into(), &rad(), &sp()),
        Ok(0.31_f64.asinh().into())
    );
    assert_eq!(
        Asinh.apply(0.31.into(), &deg(), &sp()),
        Ok(0.31_f64.asinh().into())
    );
}

#[test]
fn acosh() {
    assert_eq!(
        Acosh.apply(1.23.into(), &rad(), &sp()),
        Ok(1.23_f64.acosh().into())
    );
    assert_eq!(
        Acosh.apply(1.23.into(), &deg(), &sp()),
        Ok(1.23_f64.acosh().into())
    );
}

#[test]
fn atanh() {
    assert_eq!(
        Atanh.apply(0.31.into(), &rad(), &sp()),
        Ok(0.31_f64.atanh().into())
    );
    assert_eq!(
        Atanh.apply(0.31.into(), &deg(), &sp()),
        Ok(0.31_f64.atanh().into())
    );
}
