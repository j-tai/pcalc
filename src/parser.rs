//! Parse a string into an AST.

use crate::{Error, Expression, Result, Span, Token, TokenStream, Value};

#[cfg(test)]
mod tests;

/// Parse a zeroth-level expression: comman operators.
fn parse_0<'a>(it: &mut impl TokenStream<'a>) -> Result<(Expression, Span)> {
    let mut expr = parse_1(it)?;
    loop {
        if let (Token::Comma, _) = it.peek()? {
            let (_, span) = it.next()?;
            let rhs = parse_1(it)?;
            match expr {
                (Expression::Comma(ref mut v), _) => v.push(rhs),
                _ => expr = (Expression::Comma(vec![expr, rhs]), span),
            }
        } else {
            break Ok(expr);
        }
    }
}

/// Parse a first-level expression: variable assignment.
fn parse_1<'a>(it: &mut impl TokenStream<'a>) -> Result<(Expression, Span)> {
    let expr = parse_2(it)?;
    if let (Token::Equals, _) = it.peek()? {
        let (_, span) = it.next()?;
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
fn parse_2<'a>(it: &mut impl TokenStream<'a>) -> Result<(Expression, Span)> {
    let mut expr = parse_3(it)?;
    // Keep grabbing additions and subtractions (left associative)
    loop {
        match it.peek()? {
            (Token::Plus, _) => {
                let (_, span) = it.next()?;
                let rhs = parse_3(it)?;
                if let (Expression::Add(ref mut v), _) = expr {
                    v.push(rhs);
                } else {
                    expr = (Expression::Add(vec![expr, rhs]), span);
                }
            }
            (Token::Minus, _) => {
                let (_, span) = it.next()?;
                let rhs = parse_3(it)?;
                expr = (Expression::Sub(Box::new([expr, rhs])), span);
            }
            _ => break Ok(expr),
        }
    }
}

/// Parse a third-level expression: multiplication and division.
fn parse_3<'a>(it: &mut impl TokenStream<'a>) -> Result<(Expression, Span)> {
    let mut expr = parse_4(it)?;
    // Keep grabbing multiplications and divisions (left associative)
    loop {
        match it.peek()? {
            (Token::Times, _) => {
                let (_, span) = it.next()?;
                let rhs = parse_4(it)?;
                if let (Expression::Mul(ref mut v), _) = expr {
                    v.push(rhs);
                } else {
                    expr = (Expression::Mul(vec![expr, rhs]), span);
                }
            }
            (Token::Divide, _) => {
                let (_, span) = it.next()?;
                let rhs = parse_4(it)?;
                expr = (Expression::Frac(Box::new([expr, rhs])), span);
            }
            _ => break Ok(expr),
        }
    }
}

/// Parse a fourth-level expression: functions and prefix unary operators.
fn parse_4<'a>(it: &mut impl TokenStream<'a>) -> Result<(Expression, Span)> {
    match it.peek()? {
        (Token::Minus, _) => {
            let (_, span) = it.next()?;
            Ok((Expression::Neg(Box::new(parse_4(it)?)), span))
        }
        (Token::Ident(id), _) => {
            if let Ok(func) = id.parse() {
                let (_, span) = it.next()?;
                let expr = parse_4(it)?;
                Ok((Expression::Func(func, Box::new(expr)), span))
            } else {
                parse_5(it)
            }
        }
        _ => parse_5(it),
    }
}

/// Parse a fifth-level expression: exponentiation.
fn parse_5<'a>(it: &mut impl TokenStream<'a>) -> Result<(Expression, Span)> {
    let lhs = parse_6(it)?;
    match it.peek()? {
        (Token::Exponent, _) => {
            let (_, span) = it.next()?;
            // Right associative
            let rhs = parse_5(it)?;
            Ok((Expression::Exp(Box::new([lhs, rhs])), span))
        }
        _ => Ok(lhs),
    }
}

/// Parse a sixth-level expression: numeric literals and parentheses.
fn parse_6<'a>(it: &mut impl TokenStream<'a>) -> Result<(Expression, Span)> {
    let (tok, span) = it.next()?;
    match tok {
        Token::LeftParen => {
            let expr = parse_0(it)?;
            let (tok, span) = it.next()?;
            if tok != Token::RightParen {
                Err((Error::Syntax, span))
            } else {
                Ok(expr)
            }
        }
        Token::Integer(i) => Ok((Expression::Val(Value::Ratio((i as i64).into())), span)),
        Token::Float(n) => Ok((Expression::Val(Value::Float(n)), span)),
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

/// Parse a stream of tokens into an abstract syntax tree.
pub fn parse<'a>(mut it: impl TokenStream<'a>) -> Result<(Expression, Span)> {
    let expr = parse_0(&mut it)?;
    let (tok, span) = it.next()?;
    if tok != Token::Eof {
        Err((Error::Syntax, span))
    } else {
        Ok(expr)
    }
}
