use std::iter::Peekable;

use crate::Expression::*;
use crate::Token::*;
use crate::Value::{Float as VFloat, Ratio};
use crate::{parse, Constant, Error, Expression, Function, Result, Span, Token, TokenStream};

fn sp() -> Span {
    Span {
        file: None,
        line: 1,
        start: 1,
        end: 1,
    }
}

fn spa(start: u32, end: u32) -> Span {
    Span {
        file: None,
        line: 1,
        start,
        end,
    }
}

struct Tokens<'a, T>(Peekable<T>)
where
    T: Iterator<Item = (Token<'a>, Span)>;

impl<'a, T> TokenStream<'a> for Tokens<'a, T>
where
    T: Iterator<Item = (Token<'a>, Span)>,
{
    fn peek(&mut self) -> Result<&(Token<'a>, Span)> {
        Ok(self.0.peek().unwrap())
    }

    fn next(&mut self) -> Result<(Token<'a>, Span)> {
        Ok(self.0.next().unwrap())
    }
}

fn tok<'a>(tokens: Vec<Token<'a>>) -> impl TokenStream<'a> {
    Tokens(tokens.into_iter().map(|t| (t, sp())).peekable())
}

fn tok2<'a>(tokens: Vec<(Token<'a>, Span)>) -> impl TokenStream<'a> {
    Tokens(tokens.into_iter().peekable())
}

#[test]
fn num() {
    let tokens = vec![Float(2.5), Eof];
    assert_eq!(parse(tok(tokens)), Ok((Val(VFloat(2.5)), sp())));
}

#[test]
fn int() {
    let tokens = vec![Integer(64), Eof];
    assert_eq!(parse(tok(tokens)), Ok((Val(Ratio(64.into())), sp())));
}

#[test]
fn paren() {
    let tokens = vec![LeftParen, Float(2.5), RightParen, Eof];
    assert_eq!(parse(tok(tokens)), Ok((Val(VFloat(2.5)), sp())));
}

#[test]
fn neg() {
    let tokens = vec![Minus, Float(2.5), Eof];
    assert_eq!(
        parse(tok(tokens)),
        Ok((Neg(Box::new((Val(VFloat(2.5)), sp()))), sp())),
    );
}

#[test]
fn exp() {
    let tokens = vec![Float(2.5), Exponent, Float(1.5), Eof];
    assert_eq!(
        parse(tok(tokens)),
        Ok((
            Exp(Box::new([
                (Val(VFloat(2.5)), sp()),
                (Val(VFloat(1.5)), sp())
            ])),
            sp()
        )),
    );
}

#[test]
fn exp_multiple() {
    let tokens = vec![Float(2.5), Exponent, Float(1.5), Exponent, Float(0.5), Eof];
    assert_eq!(
        parse(tok(tokens)),
        Ok((
            Exp(Box::new([
                (Val(VFloat(2.5)), sp()),
                (
                    Exp(Box::new([
                        (Val(VFloat(1.5)), sp()),
                        (Val(VFloat(0.5)), sp())
                    ])),
                    sp()
                ),
            ])),
            sp(),
        )),
    );
}

#[test]
fn mul() {
    let tokens = vec![Float(2.5), Times, Float(1.5), Eof];
    assert_eq!(
        parse(tok(tokens)),
        Ok((
            Mul(vec![(Val(VFloat(2.5)), sp()), (Val(VFloat(1.5)), sp())]),
            sp()
        )),
    );
}

#[test]
fn mul_multiple() {
    let tokens = vec![
        Float(2.5),
        Times,
        Float(1.5),
        Times,
        Float(0.5),
        Times,
        Float(1.23),
        Eof,
    ];
    assert_eq!(
        parse(tok(tokens)),
        Ok((
            Mul(vec![
                (Val(VFloat(2.5)), sp()),
                (Val(VFloat(1.5)), sp()),
                (Val(VFloat(0.5)), sp()),
                (Val(VFloat(1.23)), sp()),
            ]),
            sp(),
        )),
    );
}

#[test]
fn frac() {
    let tokens = vec![Float(2.5), Divide, Float(1.5), Eof];
    assert_eq!(
        parse(tok(tokens)),
        Ok((
            Frac(Box::new([
                (Val(VFloat(2.5)), sp()),
                (Val(VFloat(1.5)), sp())
            ])),
            sp()
        )),
    );
}

#[test]
fn frac_multiple() {
    let tokens = vec![
        Float(2.5),
        Divide,
        Float(1.5),
        Divide,
        Float(0.5),
        Divide,
        Float(1.23),
        Eof,
    ];
    assert_eq!(
        parse(tok(tokens)),
        Ok((
            Frac(Box::new([
                (
                    Frac(Box::new([
                        (
                            Frac(Box::new([
                                (Val(VFloat(2.5)), sp()),
                                (Val(VFloat(1.5)), sp())
                            ])),
                            sp()
                        ),
                        (Val(VFloat(0.5)), sp()),
                    ])),
                    sp(),
                ),
                (Val(VFloat(1.23)), sp()),
            ])),
            sp(),
        )),
    );
}

#[test]
fn add() {
    let tokens = vec![Float(2.5), Plus, Float(1.5), Eof];
    assert_eq!(
        parse(tok(tokens)),
        Ok((
            Add(vec![(Val(VFloat(2.5)), sp()), (Val(VFloat(1.5)), sp())]),
            sp()
        )),
    );
}

#[test]
fn add_multiple() {
    let tokens = vec![
        Float(2.5),
        Plus,
        Float(1.5),
        Plus,
        Float(0.5),
        Plus,
        Float(1.23),
        Eof,
    ];
    assert_eq!(
        parse(tok(tokens)),
        Ok((
            Add(vec![
                (Val(VFloat(2.5)), sp()),
                (Val(VFloat(1.5)), sp()),
                (Val(VFloat(0.5)), sp()),
                (Val(VFloat(1.23)), sp()),
            ]),
            sp(),
        )),
    );
}

#[test]
fn sub() {
    let tokens = vec![Float(2.5), Minus, Float(1.5), Eof];
    assert_eq!(
        parse(tok(tokens)),
        Ok((
            Sub(Box::new([
                (Val(VFloat(2.5)), sp()),
                (Val(VFloat(1.5)), sp())
            ])),
            sp()
        )),
    );
}

#[test]
fn sub_multiple() {
    let tokens = vec![
        Float(2.5),
        Minus,
        Float(1.5),
        Minus,
        Float(0.5),
        Minus,
        Float(1.23),
        Eof,
    ];
    assert_eq!(
        parse(tok(tokens)),
        Ok((
            Sub(Box::new([
                (
                    Sub(Box::new([
                        (
                            Sub(Box::new([
                                (Val(VFloat(2.5)), sp()),
                                (Val(VFloat(1.5)), sp())
                            ])),
                            sp()
                        ),
                        (Val(VFloat(0.5)), sp()),
                    ])),
                    sp(),
                ),
                (Val(VFloat(1.23)), sp()),
            ])),
            sp(),
        )),
    );
}

#[test]
fn nested() {
    let tokens = vec![
        Float(1.0),
        Plus,
        Float(2.0),
        Times,
        Float(3.0),
        Plus,
        Float(4.0),
        Eof,
    ];
    assert_eq!(
        parse(tok(tokens)),
        Ok((
            Add(vec![
                (Val(VFloat(1.0)), sp()),
                (
                    Mul(vec![(Val(VFloat(2.0)), sp()), (Val(VFloat(3.0)), sp())]),
                    sp()
                ),
                (Val(VFloat(4.0)), sp()),
            ]),
            sp(),
        )),
    );
}

#[test]
fn nested_paren() {
    let tokens = vec![
        Float(1.0),
        Times,
        LeftParen,
        Float(2.0),
        Plus,
        Float(3.0),
        RightParen,
        Times,
        Float(4.0),
        Eof,
    ];
    assert_eq!(
        parse(tok(tokens)),
        Ok((
            Mul(vec![
                (Val(VFloat(1.0)), sp()),
                (
                    Add(vec![(Val(VFloat(2.0)), sp()), (Val(VFloat(3.0)), sp())]),
                    sp()
                ),
                (Val(VFloat(4.0)), sp()),
            ]),
            sp(),
        )),
    );
}

#[test]
fn constants() {
    let tokens = vec![Ident("pi"), Times, Ident("e"), Eof];
    assert_eq!(
        parse(tok(tokens)),
        Ok((
            Mul(vec![
                (Const(Constant::Pi), sp()),
                (Const(Constant::E), sp()),
            ]),
            sp(),
        )),
    );
}

#[test]
fn functions() {
    let tokens = vec![
        Ident("sin"),
        Float(12.34),
        Plus,
        Ident("atan"),
        LeftParen,
        Float(5.6),
        Minus,
        Float(5.7),
        RightParen,
        Eof,
    ];
    assert_eq!(
        parse(tok(tokens)),
        Ok((
            Add(vec![
                (
                    Func(Function::Sin, Box::new((Val(VFloat(12.34)), sp()))),
                    sp()
                ),
                (
                    Func(
                        Function::Atan,
                        Box::new((
                            Sub(Box::new([
                                (Val(VFloat(5.6)), sp()),
                                (Val(VFloat(5.7)), sp())
                            ])),
                            sp()
                        )),
                    ),
                    sp(),
                ),
            ]),
            sp(),
        )),
    );
}

#[test]
fn var() {
    let tokens = vec![
        Ident("x"),
        Plus,
        Ident("foo"),
        Plus,
        Ident("hello_world"),
        Eof,
    ];
    assert_eq!(
        parse(tok(tokens)),
        Ok((
            Add(vec![
                (Var("x".to_string()), sp()),
                (Var("foo".to_string()), sp()),
                (Var("hello_world".to_string()), sp()),
            ]),
            sp(),
        )),
    );
}

#[test]
fn r#let() {
    let tokens = vec![Ident("x"), Equals, Float(2.0), Plus, Float(2.0), Eof];
    assert_eq!(
        parse(tok(tokens)),
        Ok((
            Let(
                "x".to_string(),
                Box::new((
                    Add(vec![(Val(VFloat(2.0)), sp()), (Val(VFloat(2.0)), sp())]),
                    sp()
                )),
            ),
            sp(),
        )),
    );
}

#[test]
fn comma() {
    let tokens = vec![
        Float(1.0),
        Token::Comma,
        Float(2.0),
        Token::Comma,
        Float(3.0),
        Eof,
    ];
    assert_eq!(
        parse(tok(tokens)),
        Ok((
            Expression::Comma(vec![
                (Val(VFloat(1.0)), sp()),
                (Val(VFloat(2.0)), sp()),
                (Val(VFloat(3.0)), sp())
            ]),
            sp(),
        )),
    );
}

#[test]
fn span() {
    let tokens = vec![
        (Float(1.0), spa(1, 1)),
        (Exponent, spa(2, 2)),
        (Float(2.0), spa(3, 3)),
        (Plus, spa(4, 4)),
        (Float(3.0), spa(5, 5)),
        (Eof, spa(6, 6)),
    ];
    assert_eq!(
        parse(tok2(tokens)),
        Ok((
            Add(vec![
                (
                    Exp(Box::new([
                        (Val(VFloat(1.0)), spa(1, 1)),
                        (Val(VFloat(2.0)), spa(3, 3))
                    ])),
                    spa(2, 2),
                ),
                (Val(VFloat(3.0)), spa(5, 5)),
            ]),
            spa(4, 4),
        )),
    );
}

#[test]
fn err_span() {
    let tokens = vec![
        (Float(1.0), spa(1, 1)),
        (Exponent, spa(2, 2)),
        (Exponent, spa(3, 3)),
        (Float(1.0), spa(4, 4)),
        (Eof, spa(6, 6)),
    ];
    assert_eq!(parse(tok2(tokens)), Err((Error::Syntax, spa(3, 3))));
}

#[test]
fn func_neg() {
    let tokens = vec![Ident("sin"), Minus, Float(1.0), Eof];
    assert_eq!(
        parse(tok(tokens)),
        Ok((
            Func(
                Function::Sin,
                Box::new((Neg(Box::new((Val(VFloat(1.0)), sp()))), sp())),
            ),
            sp(),
        )),
    );
}
