//! Parse a string into an AST.

use std::iter::Peekable;

use crate::{Expression, Token};

#[cfg(test)]
mod tests;

/// Parse a zeroth-level expression: variable assignment.
fn parse_0<'a>(it: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> Expression {
    let expr = parse_1(it);
    if let Some(&Token::Equals) = it.peek() {
        it.next();
        match expr {
            Expression::Var(s) => {
                let rhs = parse_1(it);
                Expression::Let(s, Box::new(rhs))
            }
            _ => panic!("Unexpected Equals token"),
        }
    } else {
        expr
    }
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
                expr = Expression::Sub(Box::new([expr, rhs]));
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
                expr = Expression::Frac(Box::new([expr, rhs]));
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
            Expression::Exp(Box::new([lhs, rhs]))
        }
        _ => lhs,
    }
}

/// Parse a fourth-level expression: functions and prefix unary operators.
fn parse_4<'a>(it: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> Expression {
    match it.peek() {
        Some(Token::Minus) => {
            it.next();
            Expression::Neg(Box::new(parse_4(it)))
        }
        Some(Token::Ident(id)) => {
            if let Ok(func) = id.parse() {
                it.next();
                let expr = parse_5(it);
                Expression::Func(func, Box::new(expr))
            } else {
                parse_5(it)
            }
        }
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
        Token::Ident(id) => {
            if let Ok(con) = id.parse() {
                Expression::Const(con)
            } else {
                Expression::Var(id.to_string())
            }
        }
        tok => panic!("unexpected token {:?}", tok),
    }
}

// /// Parse a sixth-level expression: a fallback for an invalid expression.
// fn parse_6<'a>(it: &mut impl Iterator<Item = Token<'a>>) -> Expression {
//     todo!()
// }

/// Parse an expression into an abstract syntax tree.
pub fn parse<'a>(it: impl Iterator<Item = Token<'a>>) -> Expression {
    parse_0(&mut it.fuse().peekable())
}
