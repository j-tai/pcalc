use std::f64::consts;

use crate::Constant::*;
use crate::Function::*;
use crate::Value::*;
use crate::{AngleMeasure, Context};

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
    assert_eq!(Abs.apply(Float(-1.6), &rad()), Ok(Float(1.6)));
    assert_eq!(Abs.apply(Float(3.1), &rad()), Ok(Float(3.1)));
}

// Trig functions in radians mode

#[test]
fn sin() {
    assert_eq!(Sin.apply(Float(0.31), &rad()), Ok(Float(0.31_f64.sin())));
}

#[test]
fn cos() {
    assert_eq!(Cos.apply(Float(0.31), &rad()), Ok(Float(0.31_f64.cos())));
}

#[test]
fn tan() {
    assert_eq!(Tan.apply(Float(0.31), &rad()), Ok(Float(0.31_f64.tan())));
}

#[test]
fn asin() {
    assert_eq!(Asin.apply(Float(0.31), &rad()), Ok(Float(0.31_f64.asin())));
}

#[test]
fn acos() {
    assert_eq!(Acos.apply(Float(0.31), &rad()), Ok(Float(0.31_f64.acos())));
}

#[test]
fn atan() {
    assert_eq!(Atan.apply(Float(0.31), &rad()), Ok(Float(0.31_f64.atan())));
}

// Trig functions in degrees

#[test]
fn sin_deg() {
    assert_eq!(Sin.apply(Float(31.0), &deg()), Ok(Float(31.0_f64.to_radians().sin())));
}

#[test]
fn cos_deg() {
    assert_eq!(Cos.apply(Float(31.0), &deg()), Ok(Float(31.0_f64.to_radians().cos())));
}

#[test]
fn tan_deg() {
    assert_eq!(Tan.apply(Float(31.0), &deg()), Ok(Float(31.0_f64.to_radians().tan())));
}

#[test]
fn asin_deg() {
    assert_eq!(Asin.apply(Float(0.31), &deg()), Ok(Float(0.31_f64.asin().to_degrees())));
}

#[test]
fn acos_deg() {
    assert_eq!(Acos.apply(Float(0.31), &deg()), Ok(Float(0.31_f64.acos().to_degrees())));
}

#[test]
fn atan_deg() {
    assert_eq!(Atan.apply(Float(0.31), &deg()), Ok(Float(0.31_f64.atan().to_degrees())));
}

// Hyperbolic functions

#[test]
fn sinh() {
    assert_eq!(Sinh.apply(Float(0.31), &rad()), Ok(Float(0.31_f64.sinh())));
    assert_eq!(Sinh.apply(Float(0.31), &deg()), Ok(Float(0.31_f64.sinh())));
}

#[test]
fn cosh() {
    assert_eq!(Cosh.apply(Float(0.31), &rad()), Ok(Float(0.31_f64.cosh())));
    assert_eq!(Cosh.apply(Float(0.31), &deg()), Ok(Float(0.31_f64.cosh())));
}

#[test]
fn tanh() {
    assert_eq!(Tanh.apply(Float(0.31), &rad()), Ok(Float(0.31_f64.tanh())));
    assert_eq!(Tanh.apply(Float(0.31), &deg()), Ok(Float(0.31_f64.tanh())));
}

#[test]
fn asinh() {
    assert_eq!(Asinh.apply(Float(0.31), &rad()), Ok(Float(0.31_f64.asinh())));
    assert_eq!(Asinh.apply(Float(0.31), &deg()), Ok(Float(0.31_f64.asinh())));
}

#[test]
fn acosh() {
    assert_eq!(Acosh.apply(Float(1.23), &rad()), Ok(Float(1.23_f64.acosh())));
    assert_eq!(Acosh.apply(Float(1.23), &deg()), Ok(Float(1.23_f64.acosh())));
}

#[test]
fn atanh() {
    assert_eq!(Atanh.apply(Float(0.31), &rad()), Ok(Float(0.31_f64.atanh())));
    assert_eq!(Atanh.apply(Float(0.31), &deg()), Ok(Float(0.31_f64.atanh())));
}
