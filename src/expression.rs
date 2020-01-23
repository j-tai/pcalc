use crate::{Constant, Function, Span};

/// An abstract syntax tree for a mathematical expression.
#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    /// A constant.
    Num(f64),
    /// Negation of an expression.
    Neg(Box<(Expression, Span)>),

    /// Addition of two or more expressions.
    Add(Vec<(Expression, Span)>),
    /// Subtraction of two expressions.
    Sub(Box<[(Expression, Span); 2]>),
    /// Multipication of two or more expressions.
    Mul(Vec<(Expression, Span)>),
    /// A fraction with a numerator expression and denominator expression.
    Frac(Box<[(Expression, Span); 2]>),
    /// An exponent with a base expression and an exponent expression.
    Exp(Box<[(Expression, Span); 2]>),
    /// A radical with a radicand expression and an index expression.
    Root(Box<[(Expression, Span); 2]>),
    /// A logarithm with an expression and a base expression.
    Log(Box<[(Expression, Span); 2]>),

    /// A mathematical constant.
    Const(Constant),
    /// A call to a function.
    Func(Function, Box<(Expression, Span)>),

    /// A reference to a variable.
    Var(String),
    /// An assignment to a variable.
    Let(String, Box<(Expression, Span)>),
}
