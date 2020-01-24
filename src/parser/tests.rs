use std::iter::Peekable;

use crate::Expression::*;
use crate::Token::*;
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
    let tokens = vec![Number(2.5), Eof];
    assert_eq!(parse(tok(tokens)), Ok((Num(2.5), sp())));
}

#[test]
fn paren() {
    let tokens = vec![LeftParen, Number(2.5), RightParen, Eof];
    assert_eq!(parse(tok(tokens)), Ok((Num(2.5), sp())));
}

#[test]
fn neg() {
    let tokens = vec![Minus, Number(2.5), Eof];
    assert_eq!(
        parse(tok(tokens)),
        Ok((Neg(Box::new((Num(2.5), sp()))), sp())),
    );
}

#[test]
fn exp() {
    let tokens = vec![Number(2.5), Exponent, Number(1.5), Eof];
    assert_eq!(
        parse(tok(tokens)),
        Ok((Exp(Box::new([(Num(2.5), sp()), (Num(1.5), sp())])), sp())),
    );
}

#[test]
fn exp_multiple() {
    let tokens = vec![
        Number(2.5),
        Exponent,
        Number(1.5),
        Exponent,
        Number(0.5),
        Eof,
    ];
    assert_eq!(
        parse(tok(tokens)),
        Ok((
            Exp(Box::new([
                (Num(2.5), sp()),
                (Exp(Box::new([(Num(1.5), sp()), (Num(0.5), sp())])), sp()),
            ])),
            sp(),
        )),
    );
}

#[test]
fn mul() {
    let tokens = vec![Number(2.5), Times, Number(1.5), Eof];
    assert_eq!(
        parse(tok(tokens)),
        Ok((Mul(vec![(Num(2.5), sp()), (Num(1.5), sp())]), sp())),
    );
}

#[test]
fn mul_multiple() {
    let tokens = vec![
        Number(2.5),
        Times,
        Number(1.5),
        Times,
        Number(0.5),
        Times,
        Number(1.23),
        Eof,
    ];
    assert_eq!(
        parse(tok(tokens)),
        Ok((
            Mul(vec![
                (Num(2.5), sp()),
                (Num(1.5), sp()),
                (Num(0.5), sp()),
                (Num(1.23), sp()),
            ]),
            sp(),
        )),
    );
}

#[test]
fn frac() {
    let tokens = vec![Number(2.5), Divide, Number(1.5), Eof];
    assert_eq!(
        parse(tok(tokens)),
        Ok((Frac(Box::new([(Num(2.5), sp()), (Num(1.5), sp())])), sp())),
    );
}

#[test]
fn frac_multiple() {
    let tokens = vec![
        Number(2.5),
        Divide,
        Number(1.5),
        Divide,
        Number(0.5),
        Divide,
        Number(1.23),
        Eof,
    ];
    assert_eq!(
        parse(tok(tokens)),
        Ok((
            Frac(Box::new([
                (
                    Frac(Box::new([
                        (Frac(Box::new([(Num(2.5), sp()), (Num(1.5), sp())])), sp()),
                        (Num(0.5), sp()),
                    ])),
                    sp(),
                ),
                (Num(1.23), sp()),
            ])),
            sp(),
        )),
    );
}

#[test]
fn add() {
    let tokens = vec![Number(2.5), Plus, Number(1.5), Eof];
    assert_eq!(
        parse(tok(tokens)),
        Ok((Add(vec![(Num(2.5), sp()), (Num(1.5), sp())]), sp())),
    );
}

#[test]
fn add_multiple() {
    let tokens = vec![
        Number(2.5),
        Plus,
        Number(1.5),
        Plus,
        Number(0.5),
        Plus,
        Number(1.23),
        Eof,
    ];
    assert_eq!(
        parse(tok(tokens)),
        Ok((
            Add(vec![
                (Num(2.5), sp()),
                (Num(1.5), sp()),
                (Num(0.5), sp()),
                (Num(1.23), sp()),
            ]),
            sp(),
        )),
    );
}

#[test]
fn sub() {
    let tokens = vec![Number(2.5), Minus, Number(1.5), Eof];
    assert_eq!(
        parse(tok(tokens)),
        Ok((Sub(Box::new([(Num(2.5), sp()), (Num(1.5), sp())])), sp())),
    );
}

#[test]
fn sub_multiple() {
    let tokens = vec![
        Number(2.5),
        Minus,
        Number(1.5),
        Minus,
        Number(0.5),
        Minus,
        Number(1.23),
        Eof,
    ];
    assert_eq!(
        parse(tok(tokens)),
        Ok((
            Sub(Box::new([
                (
                    Sub(Box::new([
                        (Sub(Box::new([(Num(2.5), sp()), (Num(1.5), sp())])), sp()),
                        (Num(0.5), sp()),
                    ])),
                    sp(),
                ),
                (Num(1.23), sp()),
            ])),
            sp(),
        )),
    );
}

#[test]
fn nested() {
    let tokens = vec![
        Number(1.0),
        Plus,
        Number(2.0),
        Times,
        Number(3.0),
        Plus,
        Number(4.0),
        Eof,
    ];
    assert_eq!(
        parse(tok(tokens)),
        Ok((
            Add(vec![
                (Num(1.0), sp()),
                (Mul(vec![(Num(2.0), sp()), (Num(3.0), sp())]), sp()),
                (Num(4.0), sp()),
            ]),
            sp(),
        )),
    );
}

#[test]
fn nested_paren() {
    let tokens = vec![
        Number(1.0),
        Times,
        LeftParen,
        Number(2.0),
        Plus,
        Number(3.0),
        RightParen,
        Times,
        Number(4.0),
        Eof,
    ];
    assert_eq!(
        parse(tok(tokens)),
        Ok((
            Mul(vec![
                (Num(1.0), sp()),
                (Add(vec![(Num(2.0), sp()), (Num(3.0), sp())]), sp()),
                (Num(4.0), sp()),
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
        Number(12.34),
        Plus,
        Ident("atan"),
        LeftParen,
        Number(5.6),
        Minus,
        Number(5.7),
        RightParen,
        Eof,
    ];
    assert_eq!(
        parse(tok(tokens)),
        Ok((
            Add(vec![
                (Func(Function::Sin, Box::new((Num(12.34), sp()))), sp()),
                (
                    Func(
                        Function::Atan,
                        Box::new((Sub(Box::new([(Num(5.6), sp()), (Num(5.7), sp())])), sp())),
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
    let tokens = vec![Ident("x"), Equals, Number(2.0), Plus, Number(2.0), Eof];
    assert_eq!(
        parse(tok(tokens)),
        Ok((
            Let(
                "x".to_string(),
                Box::new((Add(vec![(Num(2.0), sp()), (Num(2.0), sp())]), sp())),
            ),
            sp(),
        )),
    );
}

#[test]
fn comma() {
    let tokens = vec![
        Number(1.0),
        Token::Comma,
        Number(2.0),
        Token::Comma,
        Number(3.0),
        Eof,
    ];
    assert_eq!(
        parse(tok(tokens)),
        Ok((
            Expression::Comma(vec![(Num(1.0), sp()), (Num(2.0), sp()), (Num(3.0), sp())]),
            sp(),
        )),
    );
}

#[test]
fn span() {
    let tokens = vec![
        (Number(1.0), spa(1, 1)),
        (Exponent, spa(2, 2)),
        (Number(2.0), spa(3, 3)),
        (Plus, spa(4, 4)),
        (Number(3.0), spa(5, 5)),
        (Eof, spa(6, 6)),
    ];
    assert_eq!(
        parse(tok2(tokens)),
        Ok((
            Add(vec![
                (
                    Exp(Box::new([(Num(1.0), spa(1, 1)), (Num(2.0), spa(3, 3))])),
                    spa(2, 2),
                ),
                (Num(3.0), spa(5, 5)),
            ]),
            spa(4, 4),
        )),
    );
}

#[test]
fn err_span() {
    let tokens = vec![
        (Number(1.0), spa(1, 1)),
        (Exponent, spa(2, 2)),
        (Exponent, spa(3, 3)),
        (Number(1.0), spa(4, 4)),
        (Eof, spa(6, 6)),
    ];
    assert_eq!(parse(tok2(tokens)), Err((Error::Syntax, spa(3, 3))));
}
