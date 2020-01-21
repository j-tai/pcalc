use crate::Token::*;
use crate::{lex, Token};

fn v(s: &str) -> Vec<Token> {
    // Take only 64 as a failsafe against infinite loops
    lex(s).take(64).collect()
}

#[test]
fn numbers() {
    assert_eq!(v("1.2345 9876"), vec![Number(1.2345), Number(9876.0)]);
}

#[test]
fn number_with_exponent() {
    assert_eq!(
        v("1.23e4 9876e-4 1e+9"),
        vec![Number(1.23e4), Number(9876e-4), Number(1e+9)],
    );
}

#[test]
fn number_missing_digits() {
    assert_eq!(v(".5 1."), vec![Number(0.5), Number(1.0)]);
}

#[test]
fn idents() {
    assert_eq!(
        v("foo bar xX_Baz_Xx"),
        vec![Ident("foo"), Ident("bar"), Ident("xX_Baz_Xx")],
    );
}

#[test]
fn operators() {
    assert_eq!(
        v("+ - * / ^ ( )"),
        vec![Plus, Minus, Times, Divide, Exponent, LeftParen, RightParen],
    );
}

#[test]
fn operators_no_space() {
    assert_eq!(
        v("+-*/^()"),
        vec![Plus, Minus, Times, Divide, Exponent, LeftParen, RightParen],
    );
}

#[test]
fn excess_spaces() {
    assert_eq!(v("   3 +5  "), vec![Number(3.0), Plus, Number(5.0)])
}
