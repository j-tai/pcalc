use crate::Expression::*;
use crate::Token::*;
use crate::{parse, Constant, Function};

#[test]
fn num() {
    let tokens = vec![Number(2.5)];
    assert_eq!(parse(tokens.into_iter()), Num(2.5))
}

#[test]
fn paren() {
    let tokens = vec![LeftParen, Number(2.5), RightParen];
    assert_eq!(parse(tokens.into_iter()), Num(2.5))
}

#[test]
fn neg() {
    let tokens = vec![Minus, Number(2.5)];
    assert_eq!(parse(tokens.into_iter()), Neg(Box::new(Num(2.5))))
}

#[test]
fn exp() {
    let tokens = vec![Number(2.5), Exponent, Number(1.5)];
    assert_eq!(
        parse(tokens.into_iter()),
        Exp(Box::new([Num(2.5), Num(1.5)])),
    )
}

#[test]
fn exp_multiple() {
    let tokens = vec![Number(2.5), Exponent, Number(1.5), Exponent, Number(0.5)];
    assert_eq!(
        parse(tokens.into_iter()),
        Exp(Box::new([Num(2.5), Exp(Box::new([Num(1.5), Num(0.5)]))])),
    )
}

#[test]
fn mul() {
    let tokens = vec![Number(2.5), Times, Number(1.5)];
    assert_eq!(parse(tokens.into_iter()), Mul(vec![Num(2.5), Num(1.5)]))
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
    ];
    assert_eq!(
        parse(tokens.into_iter()),
        Mul(vec![Num(2.5), Num(1.5), Num(0.5), Num(1.23)]),
    )
}

#[test]
fn frac() {
    let tokens = vec![Number(2.5), Divide, Number(1.5)];
    assert_eq!(
        parse(tokens.into_iter()),
        Frac(Box::new([Num(2.5), Num(1.5)])),
    )
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
    ];
    assert_eq!(
        parse(tokens.into_iter()),
        Frac(Box::new([
            Frac(Box::new([Frac(Box::new([Num(2.5), Num(1.5)])), Num(0.5)])),
            Num(1.23),
        ])),
    )
}

#[test]
fn add() {
    let tokens = vec![Number(2.5), Plus, Number(1.5)];
    assert_eq!(parse(tokens.into_iter()), Add(vec![Num(2.5), Num(1.5)]))
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
    ];
    assert_eq!(
        parse(tokens.into_iter()),
        Add(vec![Num(2.5), Num(1.5), Num(0.5), Num(1.23)]),
    )
}

#[test]
fn sub() {
    let tokens = vec![Number(2.5), Minus, Number(1.5)];
    assert_eq!(
        parse(tokens.into_iter()),
        Sub(Box::new([Num(2.5), Num(1.5)])),
    )
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
    ];
    assert_eq!(
        parse(tokens.into_iter()),
        Sub(Box::new([
            Sub(Box::new([Sub(Box::new([Num(2.5), Num(1.5)])), Num(0.5)])),
            Num(1.23),
        ])),
    )
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
    ];
    assert_eq!(
        parse(tokens.into_iter()),
        Add(vec![Num(1.0), Mul(vec![Num(2.0), Num(3.0)]), Num(4.0)]),
    )
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
    ];
    assert_eq!(
        parse(tokens.into_iter()),
        Mul(vec![Num(1.0), Add(vec![Num(2.0), Num(3.0)]), Num(4.0)]),
    )
}

#[test]
fn constants() {
    let tokens = vec![Ident("pi"), Times, Ident("e")];
    assert_eq!(
        parse(tokens.into_iter()),
        Mul(vec![Const(Constant::Pi), Const(Constant::E)]),
    )
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
    ];
    assert_eq!(
        parse(tokens.into_iter()),
        Add(vec![
            Func(Function::Sin, Box::new(Num(12.34))),
            Func(
                Function::Atan,
                Box::new(Sub(Box::new([Num(5.6), Num(5.7)]))),
            ),
        ])
    )
}

#[test]
fn var() {
    let tokens = vec![Ident("x"), Plus, Ident("foo"), Plus, Ident("hello_world")];
    assert_eq!(
        parse(tokens.into_iter()),
        Add(vec![
            Var("x".to_string()),
            Var("foo".to_string()),
            Var("hello_world".to_string()),
        ])
    )
}
