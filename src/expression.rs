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

    /// The mathematical constant pi.
    Pi,
    /// The mathematical constant e.
    E,

    /// The absolute value function.
    Abs(Box<Expression>),

    /// The sine function.
    Sin(Box<Expression>),
    /// The cosine function.
    Cos(Box<Expression>),
    /// The tangent function.
    Tan(Box<Expression>),
    /// The inverse sine function.
    Asin(Box<Expression>),
    /// The inverse cosine function.
    Acos(Box<Expression>),
    /// The inverse tangent function.
    Atan(Box<Expression>),

    /// The hyperbolic sine function.
    Sinh(Box<Expression>),
    /// The hyperbolic cosine function.
    Cosh(Box<Expression>),
    /// The hyperbolic tangent function.
    Tanh(Box<Expression>),
    /// The inverse hyperbolic sine function.
    Asinh(Box<Expression>),
    /// The inverse hyperbolic cosine function.
    Acosh(Box<Expression>),
    /// The inverse hyperbolic tangent function.
    Atanh(Box<Expression>),
}
