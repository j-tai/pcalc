use crate::{Constant, Function};

/// An abstract syntax tree for a mathematical expression.
#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    /// A constant.
    Num(f64),
    /// Negation of an expression.
    Neg(Box<Expression>),

    /// Addition of two or more expressions.
    Add(Vec<Expression>),
    /// Subtraction of two expressions.
    Sub(Box<Expression>, Box<Expression>),
    /// Multipication of two or more expressions.
    Mul(Vec<Expression>),
    /// A fraction with a numerator expression and denominator expression.
    Frac(Box<Expression>, Box<Expression>),
    /// An exponent with a base expression and an exponent expression.
    Exp(Box<Expression>, Box<Expression>),
    /// A radical with a radicand expression and an index expression.
    Root(Box<Expression>, Box<Expression>),
    /// A logarithm with an expression and a base expression.
    Log(Box<Expression>, Box<Expression>),

    /// A mathematical constant.
    Const(Constant),

    /// A call to a function.
    Func(Function, Box<Expression>),
}
