//! Parse a string into an AST.

use std::iter::Peekable;

use crate::{Error, Expression, Result, Span, Token};

#[cfg(test)]
mod tests;

/// Parse a first-level expression: variable assignment.
fn parse_1<'a>(
    it: &mut Peekable<impl Iterator<Item = (Token<'a>, Span)>>,
) -> Result<(Expression, Span)> {
    let expr = parse_2(it)?;
    if let (Token::Equals, _) = it.peek().unwrap() {
        let (_, span) = it.next().unwrap();
        match expr {
            (Expression::Var(s), _) => {
                let rhs = parse_2(it)?;
                Ok((Expression::Let(s, Box::new(rhs)), span))
            }
            _ => Err((Error::Syntax, span)),
        }
    } else {
        Ok(expr)
    }
}

/// Parse a second-level expression: addition and subtraction.
fn parse_2<'a>(
    it: &mut Peekable<impl Iterator<Item = (Token<'a>, Span)>>,
) -> Result<(Expression, Span)> {
    let mut expr = parse_3(it)?;
    // Keep grabbing additions and subtractions (left associative)
    loop {
        match it.peek().unwrap() {
            (Token::Plus, _) => {
                let (_, span) = it.next().unwrap();
                let rhs = parse_3(it)?;
                if let (Expression::Add(ref mut v), _) = expr {
                    v.push(rhs);
                } else {
                    expr = (Expression::Add(vec![expr, rhs]), span);
                }
            }
            (Token::Minus, _) => {
                let (_, span) = it.next().unwrap();
                let rhs = parse_3(it)?;
                expr = (Expression::Sub(Box::new([expr, rhs])), span);
            }
            _ => break Ok(expr),
        }
    }
}

/// Parse a third-level expression: multiplication and division.
fn parse_3<'a>(
    it: &mut Peekable<impl Iterator<Item = (Token<'a>, Span)>>,
) -> Result<(Expression, Span)> {
    let mut expr = parse_4(it)?;
    // Keep grabbing multiplications and divisions (left associative)
    loop {
        match it.peek().unwrap() {
            (Token::Times, _) => {
                let (_, span) = it.next().unwrap();
                let rhs = parse_4(it)?;
                if let (Expression::Mul(ref mut v), _) = expr {
                    v.push(rhs);
                } else {
                    expr = (Expression::Mul(vec![expr, rhs]), span);
                }
            }
            (Token::Divide, _) => {
                let (_, span) = it.next().unwrap();
                let rhs = parse_4(it)?;
                expr = (Expression::Frac(Box::new([expr, rhs])), span);
            }
            _ => break Ok(expr),
        }
    }
}

/// Parse a fourth-level expression: exponentiation.
fn parse_4<'a>(
    it: &mut Peekable<impl Iterator<Item = (Token<'a>, Span)>>,
) -> Result<(Expression, Span)> {
    let lhs = parse_5(it)?;
    match it.peek().unwrap() {
        (Token::Exponent, _) => {
            let (_, span) = it.next().unwrap();
            // Right associative
            let rhs = parse_4(it)?;
            Ok((Expression::Exp(Box::new([lhs, rhs])), span))
        }
        _ => Ok(lhs),
    }
}

/// Parse a fifth-level expression: functions and prefix unary operators.
fn parse_5<'a>(
    it: &mut Peekable<impl Iterator<Item = (Token<'a>, Span)>>,
) -> Result<(Expression, Span)> {
    match it.peek().unwrap() {
        (Token::Minus, _) => {
            let (_, span) = it.next().unwrap();
            Ok((Expression::Neg(Box::new(parse_5(it)?)), span))
        }
        (Token::Ident(id), _) => {
            if let Ok(func) = id.parse() {
                let (_, span) = it.next().unwrap();
                let expr = parse_6(it)?;
                Ok((Expression::Func(func, Box::new(expr)), span))
            } else {
                parse_6(it)
            }
        }
        _ => parse_6(it),
    }
}

/// Parse a sixth-level expression: numeric literals and parentheses.
fn parse_6<'a>(
    it: &mut Peekable<impl Iterator<Item = (Token<'a>, Span)>>,
) -> Result<(Expression, Span)> {
    let (tok, span) = it.next().unwrap();
    match tok {
        Token::LeftParen => {
            let expr = parse_2(it)?;
            let (tok, span) = it.next().unwrap();
            if tok != Token::RightParen {
                Err((Error::Syntax, span))
            } else {
                Ok(expr)
            }
        }
        Token::Number(n) => Ok((Expression::Num(n), span)),
        Token::Ident(id) => {
            if let Ok(con) = id.parse() {
                Ok((Expression::Const(con), span))
            } else {
                Ok((Expression::Var(id.to_string()), span))
            }
        }
        _ => Err((Error::Syntax, span)),
    }
}

/// Parse an expression into an abstract syntax tree.
pub fn parse<'a>(it: impl Iterator<Item = (Token<'a>, Span)>) -> Result<(Expression, Span)> {
    let mut it = it.fuse().peekable();
    let expr = parse_1(&mut it)?;
    let (tok, span) = it
        .next()
        .expect("Unexpected end of token stream (missing Token::Eof?)");
    if tok != Token::Eof {
        Err((Error::Syntax, span.clone()))
    } else {
        Ok(expr)
    }
}
