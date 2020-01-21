use std::f64::consts;

use crate::Constant::*;
use crate::Function::*;
use crate::{AngleMeasure, Context};

// Constants

#[test]
fn pi() {
    assert_eq!(Pi.value(), consts::PI);
}

#[test]
fn e() {
    assert_eq!(E.value(), consts::E);
}

// Functions

static RAD: Context = Context {
    angle: AngleMeasure::Radians,
    notation_range: (0.0, std::f64::INFINITY),
};

static DEG: Context = Context {
    angle: AngleMeasure::Degrees,
    notation_range: (0.0, std::f64::INFINITY),
};

#[test]
fn abs() {
    assert_eq!(Abs.apply(-1.6, &RAD), Ok(1.6));
    assert_eq!(Abs.apply(3.1, &RAD), Ok(3.1));
}

// Trig functions in radians mode

#[test]
fn sin() {
    assert_eq!(Sin.apply(0.31, &RAD), Ok(0.31_f64.sin()));
}

#[test]
fn cos() {
    assert_eq!(Cos.apply(0.31, &RAD), Ok(0.31_f64.cos()));
}

#[test]
fn tan() {
    assert_eq!(Tan.apply(0.31, &RAD), Ok(0.31_f64.tan()));
}

#[test]
fn asin() {
    assert_eq!(Asin.apply(0.31, &RAD), Ok(0.31_f64.asin()));
}

#[test]
fn acos() {
    assert_eq!(Acos.apply(0.31, &RAD), Ok(0.31_f64.acos()));
}

#[test]
fn atan() {
    assert_eq!(Atan.apply(0.31, &RAD), Ok(0.31_f64.atan()));
}

// Trig functions in degrees

#[test]
fn sin_deg() {
    assert_eq!(Sin.apply(31.0, &DEG), Ok(31.0_f64.to_radians().sin()));
}

#[test]
fn cos_deg() {
    assert_eq!(Cos.apply(31.0, &DEG), Ok(31.0_f64.to_radians().cos()));
}

#[test]
fn tan_deg() {
    assert_eq!(Tan.apply(31.0, &DEG), Ok(31.0_f64.to_radians().tan()));
}

#[test]
fn asin_deg() {
    assert_eq!(Asin.apply(0.31, &DEG), Ok(0.31_f64.asin().to_degrees()));
}

#[test]
fn acos_deg() {
    assert_eq!(Acos.apply(0.31, &DEG), Ok(0.31_f64.acos().to_degrees()));
}

#[test]
fn atan_deg() {
    assert_eq!(Atan.apply(0.31, &DEG), Ok(0.31_f64.atan().to_degrees()));
}

// Hyperbolic functions

#[test]
fn sinh() {
    assert_eq!(Sinh.apply(0.31, &RAD), Ok(0.31_f64.sinh()));
    assert_eq!(Sinh.apply(0.31, &DEG), Ok(0.31_f64.sinh()));
}

#[test]
fn cosh() {
    assert_eq!(Cosh.apply(0.31, &RAD), Ok(0.31_f64.cosh()));
    assert_eq!(Cosh.apply(0.31, &DEG), Ok(0.31_f64.cosh()));
}

#[test]
fn tanh() {
    assert_eq!(Tanh.apply(0.31, &RAD), Ok(0.31_f64.tanh()));
    assert_eq!(Tanh.apply(0.31, &DEG), Ok(0.31_f64.tanh()));
}

#[test]
fn asinh() {
    assert_eq!(Asinh.apply(0.31, &RAD), Ok(0.31_f64.asinh()));
    assert_eq!(Asinh.apply(0.31, &DEG), Ok(0.31_f64.asinh()));
}

#[test]
fn acosh() {
    assert_eq!(Acosh.apply(1.23, &RAD), Ok(1.23_f64.acosh()));
    assert_eq!(Acosh.apply(1.23, &DEG), Ok(1.23_f64.acosh()));
}

#[test]
fn atanh() {
    assert_eq!(Atanh.apply(0.31, &RAD), Ok(0.31_f64.atanh()));
    assert_eq!(Atanh.apply(0.31, &DEG), Ok(0.31_f64.atanh()));
}
