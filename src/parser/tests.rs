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
        Ok(self.0.peek().expect("No more tokens--forgot Token::Eof?"))
    }

    fn next(&mut self) -> Result<(Token<'a>, Span)> {
        Ok(self.0.next().expect("No more tokens--forgot Token::Eof?"))
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
    let tokens = vec![2.5.into(), Eof];
    assert_eq!(parse(tok(tokens)), Ok((2.5.into(), sp())));
}

#[test]
fn int() {
    let tokens = vec![64.into(), Eof];
    assert_eq!(parse(tok(tokens)), Ok((64.into(), sp())));
}

#[test]
fn paren() {
    let tokens = vec![LeftParen, 2.5.into(), RightParen, Eof];
    assert_eq!(parse(tok(tokens)), Ok((2.5.into(), sp())));
}

#[test]
fn neg() {
    let tokens = vec![Minus, 2.5.into(), Eof];
    assert_eq!(
        parse(tok(tokens)),
        Ok((Neg(Box::new((2.5.into(), sp()))), sp())),
    );
}

#[test]
fn exp() {
    let tokens = vec![2.5.into(), Exponent, 1.5.into(), Eof];
    assert_eq!(
        parse(tok(tokens)),
        Ok((
            Exp(Box::new([(2.5.into(), sp()), (1.5.into(), sp())])),
            sp()
        )),
    );
}

#[test]
fn exp_multiple() {
    let tokens = vec![2.5.into(), Exponent, 1.5.into(), Exponent, 0.5.into(), Eof];
    assert_eq!(
        parse(tok(tokens)),
        Ok((
            Exp(Box::new([
                (2.5.into(), sp()),
                (
                    Exp(Box::new([(1.5.into(), sp()), (0.5.into(), sp())])),
                    sp()
                ),
            ])),
            sp(),
        )),
    );
}

#[test]
fn mul() {
    let tokens = vec![2.5.into(), Times, 1.5.into(), Eof];
    assert_eq!(
        parse(tok(tokens)),
        Ok((Mul(vec![(2.5.into(), sp()), (1.5.into(), sp())]), sp())),
    );
}

#[test]
fn mul_multiple() {
    let tokens = vec![
        2.5.into(),
        Times,
        1.5.into(),
        Times,
        0.5.into(),
        Times,
        1.23.into(),
        Eof,
    ];
    assert_eq!(
        parse(tok(tokens)),
        Ok((
            Mul(vec![
                (2.5.into(), sp()),
                (1.5.into(), sp()),
                (0.5.into(), sp()),
                (1.23.into(), sp()),
            ]),
            sp(),
        )),
    );
}

#[test]
fn frac() {
    let tokens = vec![2.5.into(), Divide, 1.5.into(), Eof];
    assert_eq!(
        parse(tok(tokens)),
        Ok((
            Frac(Box::new([(2.5.into(), sp()), (1.5.into(), sp())])),
            sp()
        )),
    );
}

#[test]
fn frac_multiple() {
    let tokens = vec![
        2.5.into(),
        Divide,
        1.5.into(),
        Divide,
        0.5.into(),
        Divide,
        1.23.into(),
        Eof,
    ];
    assert_eq!(
        parse(tok(tokens)),
        Ok((
            Frac(Box::new([
                (
                    Frac(Box::new([
                        (
                            Frac(Box::new([(2.5.into(), sp()), (1.5.into(), sp())])),
                            sp()
                        ),
                        (0.5.into(), sp()),
                    ])),
                    sp(),
                ),
                (1.23.into(), sp()),
            ])),
            sp(),
        )),
    );
}

#[test]
fn add() {
    let tokens = vec![2.5.into(), Plus, 1.5.into(), Eof];
    assert_eq!(
        parse(tok(tokens)),
        Ok((Add(vec![(2.5.into(), sp()), (1.5.into(), sp())]), sp())),
    );
}

#[test]
fn add_multiple() {
    let tokens = vec![
        2.5.into(),
        Plus,
        1.5.into(),
        Plus,
        0.5.into(),
        Plus,
        1.23.into(),
        Eof,
    ];
    assert_eq!(
        parse(tok(tokens)),
        Ok((
            Add(vec![
                (2.5.into(), sp()),
                (1.5.into(), sp()),
                (0.5.into(), sp()),
                (1.23.into(), sp()),
            ]),
            sp(),
        )),
    );
}

#[test]
fn sub() {
    let tokens = vec![2.5.into(), Minus, 1.5.into(), Eof];
    assert_eq!(
        parse(tok(tokens)),
        Ok((
            Sub(Box::new([(2.5.into(), sp()), (1.5.into(), sp())])),
            sp()
        )),
    );
}

#[test]
fn sub_multiple() {
    let tokens = vec![
        2.5.into(),
        Minus,
        1.5.into(),
        Minus,
        0.5.into(),
        Minus,
        1.23.into(),
        Eof,
    ];
    assert_eq!(
        parse(tok(tokens)),
        Ok((
            Sub(Box::new([
                (
                    Sub(Box::new([
                        (
                            Sub(Box::new([(2.5.into(), sp()), (1.5.into(), sp())])),
                            sp()
                        ),
                        (0.5.into(), sp()),
                    ])),
                    sp(),
                ),
                (1.23.into(), sp()),
            ])),
            sp(),
        )),
    );
}

#[test]
fn nested() {
    let tokens = vec![
        1.0.into(),
        Plus,
        2.0.into(),
        Times,
        3.0.into(),
        Plus,
        4.0.into(),
        Eof,
    ];
    assert_eq!(
        parse(tok(tokens)),
        Ok((
            Add(vec![
                (1.0.into(), sp()),
                (Mul(vec![(2.0.into(), sp()), (3.0.into(), sp())]), sp()),
                (4.0.into(), sp()),
            ]),
            sp(),
        )),
    );
}

#[test]
fn nested_paren() {
    let tokens = vec![
        1.0.into(),
        Times,
        LeftParen,
        2.0.into(),
        Plus,
        3.0.into(),
        RightParen,
        Times,
        4.0.into(),
        Eof,
    ];
    assert_eq!(
        parse(tok(tokens)),
        Ok((
            Mul(vec![
                (1.0.into(), sp()),
                (Add(vec![(2.0.into(), sp()), (3.0.into(), sp())]), sp()),
                (4.0.into(), sp()),
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
        12.34.into(),
        Plus,
        Ident("atan"),
        LeftParen,
        5.6.into(),
        Minus,
        5.7.into(),
        RightParen,
        Eof,
    ];
    assert_eq!(
        parse(tok(tokens)),
        Ok((
            Add(vec![
                (Func(Function::Sin, Box::new((12.34.into(), sp()))), sp()),
                (
                    Func(
                        Function::Atan,
                        Box::new((
                            Sub(Box::new([(5.6.into(), sp()), (5.7.into(), sp())])),
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
    let tokens = vec![Ident("x"), Equals, 2.0.into(), Plus, 2.0.into(), Eof];
    assert_eq!(
        parse(tok(tokens)),
        Ok((
            Let(
                "x".to_string(),
                Box::new((Add(vec![(2.0.into(), sp()), (2.0.into(), sp())]), sp())),
            ),
            sp(),
        )),
    );
}

#[test]
fn comma() {
    let tokens = vec![
        1.0.into(),
        Token::Comma,
        2.0.into(),
        Token::Comma,
        3.0.into(),
        Eof,
    ];
    assert_eq!(
        parse(tok(tokens)),
        Ok((
            Expression::Comma(vec![
                (1.0.into(), sp()),
                (2.0.into(), sp()),
                (3.0.into(), sp())
            ]),
            sp(),
        )),
    );
}

#[test]
fn span() {
    let tokens = vec![
        (1.0.into(), spa(1, 1)),
        (Exponent, spa(2, 2)),
        (2.0.into(), spa(3, 3)),
        (Plus, spa(4, 4)),
        (3.0.into(), spa(5, 5)),
        (Eof, spa(6, 6)),
    ];
    assert_eq!(
        parse(tok2(tokens)),
        Ok((
            Add(vec![
                (
                    Exp(Box::new([(1.0.into(), spa(1, 1)), (2.0.into(), spa(3, 3))])),
                    spa(2, 2),
                ),
                (3.0.into(), spa(5, 5)),
            ]),
            spa(4, 4),
        )),
    );
}

#[test]
fn err_span() {
    let tokens = vec![
        (1.0.into(), spa(1, 1)),
        (Exponent, spa(2, 2)),
        (Exponent, spa(3, 3)),
        (1.0.into(), spa(4, 4)),
        (Eof, spa(6, 6)),
    ];
    assert_eq!(parse(tok2(tokens)), Err((Error::Syntax, spa(3, 3))));
}

#[test]
fn func_neg() {
    let tokens = vec![Ident("sin"), Minus, 1.0.into(), Eof];
    assert_eq!(
        parse(tok(tokens)),
        Ok((
            Func(
                Function::Sin,
                Box::new((Neg(Box::new((1.0.into(), sp()))), sp())),
            ),
            sp(),
        )),
    );
}

#[test]
fn neg_exp() {
    let tokens = vec![Minus, 1.into(), Exponent, 2.into(), Eof];
    assert_eq!(
        parse(tok(tokens)),
        Ok((
            Neg(Box::new((
                Exp(Box::new([(1.into(), sp()), (2.into(), sp())])),
                sp(),
            ))),
            sp(),
        ))
    )
}

#[test]
fn call() {
    let tokens = vec![Ident("foo"), LeftParen, RightParen, Eof];
    assert_eq!(
        parse(tok(tokens)),
        Ok((Call("foo".to_string(), vec![]), sp())),
    );
    let tokens = vec![Ident("foo"), LeftParen, 1.into(), RightParen, Eof];
    assert_eq!(
        parse(tok(tokens)),
        Ok((Call("foo".to_string(), vec![(1.into(), sp())]), sp())),
    );
    let tokens = vec![
        Ident("foo"),
        LeftParen,
        1.into(),
        Token::Comma,
        2.into(),
        RightParen,
        Eof,
    ];
    assert_eq!(
        parse(tok(tokens)),
        Ok((
            Call("foo".to_string(), vec![(1.into(), sp()), (2.into(), sp())]),
            sp()
        )),
    );
}
