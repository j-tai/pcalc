//! Parse a string into an AST.

use std::iter::Peekable;

use crate::lexer::Token;

#[cfg(test)]
mod tests;

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

/// Parse a first-level expression: addition and subtraction.
fn parse_1<'a>(it: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> Expression {
    let mut expr = parse_2(it);
    // Keep grabbing additions and subtractions (left associative)
    loop {
        match it.peek() {
            Some(Token::Plus) => {
                it.next();
                let rhs = parse_2(it);
                if let Expression::Add(ref mut v) = expr {
                    v.push(rhs);
                } else {
                    expr = Expression::Add(vec![expr, rhs]);
                }
            }
            Some(Token::Minus) => {
                it.next();
                let rhs = parse_2(it);
                expr = Expression::Sub(Box::new(expr), Box::new(rhs));
            }
            _ => break,
        }
    }
    expr
}

/// Parse a second-level expression: multiplication and division.
fn parse_2<'a>(it: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> Expression {
    let mut expr = parse_3(it);
    // Keep grabbing multiplications and divisions (left associative)
    loop {
        match it.peek() {
            Some(Token::Times) => {
                it.next();
                let rhs = parse_3(it);
                if let Expression::Mul(ref mut v) = expr {
                    v.push(rhs);
                } else {
                    expr = Expression::Mul(vec![expr, rhs]);
                }
            }
            Some(Token::Divide) => {
                it.next();
                let rhs = parse_3(it);
                expr = Expression::Frac(Box::new(expr), Box::new(rhs));
            }
            _ => break,
        }
    }
    expr
}

/// Parse a third-level expression: exponentiation.
fn parse_3<'a>(it: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> Expression {
    let lhs = parse_4(it);
    match it.peek() {
        Some(Token::Exponent) => {
            it.next();
            // Right associative
            let rhs = parse_3(it);
            Expression::Exp(Box::new(lhs), Box::new(rhs))
        }
        _ => lhs,
    }
}

/// Parse a fourth-level expression: functions and prefix unary operators.
fn parse_4<'a>(it: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> Expression {
    let tok = it.peek().expect("unexpected end of input");
    match tok {
        Token::Minus => {
            it.next();
            Expression::Neg(Box::new(parse_4(it)))
        }
        // TODO: Parse functions
        _ => parse_5(it),
    }
}

/// Parse a fifth-level expression: numeric literals and parentheses.
fn parse_5<'a>(it: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> Expression {
    let tok = it.next().expect("unexpected end of input");
    match tok {
        Token::LeftParen => {
            let expr = parse_1(it);
            assert_eq!(it.next(), Some(Token::RightParen));
            expr
        }
        Token::Number(n) => Expression::Num(n),
        tok => panic!("unexpected token {:?}", tok),
    }
}

// /// Parse a sixth-level expression: a fallback for an invalid expression.
// fn parse_6<'a>(it: &mut impl Iterator<Item = Token<'a>>) -> Expression {
//     todo!()
// }

/// Parse an expression into an abstract syntax tree.
pub fn parse<'a>(it: impl Iterator<Item = Token<'a>>) -> Expression {
    parse_1(&mut it.fuse().peekable())
}
