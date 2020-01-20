use crate::lexer::Token;
use crate::math::Expression;
use crate::parser::parse;

#[test]
fn num() {
    let tokens = vec![Token::Number(2.5)];
    assert_eq!(parse(tokens.into_iter()), Expression::Num(2.5))
}

#[test]
fn paren() {
    let tokens = vec![Token::LeftParen, Token::Number(2.5), Token::RightParen];
    assert_eq!(parse(tokens.into_iter()), Expression::Num(2.5))
}

#[test]
fn neg() {
    let tokens = vec![Token::Minus, Token::Number(2.5)];
    assert_eq!(
        parse(tokens.into_iter()),
        Expression::Neg(Box::new(Expression::Num(2.5))),
    )
}

#[test]
fn exp() {
    let tokens = vec![Token::Number(2.5), Token::Exponent, Token::Number(1.5)];
    assert_eq!(
        parse(tokens.into_iter()),
        Expression::Exp(
            Box::new(Expression::Num(2.5)),
            Box::new(Expression::Num(1.5)),
        ),
    )
}

#[test]
fn exp_multiple() {
    let tokens = vec![
        Token::Number(2.5),
        Token::Exponent,
        Token::Number(1.5),
        Token::Exponent,
        Token::Number(0.5),
    ];
    assert_eq!(
        parse(tokens.into_iter()),
        Expression::Exp(
            Box::new(Expression::Num(2.5)),
            Box::new(Expression::Exp(
                Box::new(Expression::Num(1.5)),
                Box::new(Expression::Num(0.5)),
            )),
        ),
    )
}

#[test]
fn mul() {
    let tokens = vec![Token::Number(2.5), Token::Times, Token::Number(1.5)];
    assert_eq!(
        parse(tokens.into_iter()),
        Expression::Mul(vec![Expression::Num(2.5), Expression::Num(1.5)]),
    )
}

#[test]
fn mul_multiple() {
    let tokens = vec![
        Token::Number(2.5),
        Token::Times,
        Token::Number(1.5),
        Token::Times,
        Token::Number(0.5),
        Token::Times,
        Token::Number(1.23),
    ];
    assert_eq!(
        parse(tokens.into_iter()),
        Expression::Mul(vec![
            Expression::Num(2.5),
            Expression::Num(1.5),
            Expression::Num(0.5),
            Expression::Num(1.23),
        ]),
    )
}

#[test]
fn frac() {
    let tokens = vec![Token::Number(2.5), Token::Divide, Token::Number(1.5)];
    assert_eq!(
        parse(tokens.into_iter()),
        Expression::Frac(
            Box::new(Expression::Num(2.5)),
            Box::new(Expression::Num(1.5)),
        ),
    )
}

#[test]
fn frac_multiple() {
    let tokens = vec![
        Token::Number(2.5),
        Token::Divide,
        Token::Number(1.5),
        Token::Divide,
        Token::Number(0.5),
        Token::Divide,
        Token::Number(1.23),
    ];
    assert_eq!(
        parse(tokens.into_iter()),
        Expression::Frac(
            Box::new(Expression::Frac(
                Box::new(Expression::Frac(
                    Box::new(Expression::Num(2.5)),
                    Box::new(Expression::Num(1.5)),
                )),
                Box::new(Expression::Num(0.5)),
            )),
            Box::new(Expression::Num(1.23)),
        ),
    )
}

#[test]
fn add() {
    let tokens = vec![Token::Number(2.5), Token::Plus, Token::Number(1.5)];
    assert_eq!(
        parse(tokens.into_iter()),
        Expression::Add(vec![Expression::Num(2.5), Expression::Num(1.5)]),
    )
}

#[test]
fn add_multiple() {
    let tokens = vec![
        Token::Number(2.5),
        Token::Plus,
        Token::Number(1.5),
        Token::Plus,
        Token::Number(0.5),
        Token::Plus,
        Token::Number(1.23),
    ];
    assert_eq!(
        parse(tokens.into_iter()),
        Expression::Add(vec![
            Expression::Num(2.5),
            Expression::Num(1.5),
            Expression::Num(0.5),
            Expression::Num(1.23),
        ]),
    )
}

#[test]
fn sub() {
    let tokens = vec![Token::Number(2.5), Token::Minus, Token::Number(1.5)];
    assert_eq!(
        parse(tokens.into_iter()),
        Expression::Sub(
            Box::new(Expression::Num(2.5)),
            Box::new(Expression::Num(1.5)),
        ),
    )
}

#[test]
fn sub_multiple() {
    let tokens = vec![
        Token::Number(2.5),
        Token::Minus,
        Token::Number(1.5),
        Token::Minus,
        Token::Number(0.5),
        Token::Minus,
        Token::Number(1.23),
    ];
    assert_eq!(
        parse(tokens.into_iter()),
        Expression::Sub(
            Box::new(Expression::Sub(
                Box::new(Expression::Sub(
                    Box::new(Expression::Num(2.5)),
                    Box::new(Expression::Num(1.5)),
                )),
                Box::new(Expression::Num(0.5)),
            )),
            Box::new(Expression::Num(1.23)),
        ),
    )
}

#[test]
fn nested() {
    let tokens = vec![
        Token::Number(1.0),
        Token::Plus,
        Token::Number(2.0),
        Token::Times,
        Token::Number(3.0),
        Token::Plus,
        Token::Number(4.0),
    ];
    assert_eq!(
        parse(tokens.into_iter()),
        Expression::Add(vec![
            Expression::Num(1.0),
            Expression::Mul(vec![Expression::Num(2.0), Expression::Num(3.0)]),
            Expression::Num(4.0),
        ]),
    )
}

#[test]
fn nested_paren() {
    let tokens = vec![
        Token::Number(1.0),
        Token::Times,
        Token::LeftParen,
        Token::Number(2.0),
        Token::Plus,
        Token::Number(3.0),
        Token::RightParen,
        Token::Times,
        Token::Number(4.0),
    ];
    assert_eq!(
        parse(tokens.into_iter()),
        Expression::Mul(vec![
            Expression::Num(1.0),
            Expression::Add(vec![Expression::Num(2.0), Expression::Num(3.0)]),
            Expression::Num(4.0),
        ]),
    )
}
