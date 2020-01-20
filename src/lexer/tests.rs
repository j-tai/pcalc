use crate::lexer::Token::*;
use crate::lexer::*;

fn lex_v(s: &str) -> Vec<Token> {
    // Take only 64 as a failsafe against infinite loops
    lex(s).take(64).collect()
}

#[test]
fn lex_numbers() {
    assert_eq!(lex_v("1.2345 9876"), vec![Number(1.2345), Number(9876.0)]);
}

#[test]
fn lex_number_with_exponent() {
    assert_eq!(
        lex_v("1.23e4 9876e-4 1e+9"),
        vec![Number(1.23e4), Number(9876e-4), Number(1e+9)],
    );
}

#[test]
fn lex_number_missing_digits() {
    assert_eq!(lex_v(".5 1."), vec![Number(0.5), Number(1.0)]);
}

#[test]
fn lex_idents() {
    assert_eq!(
        lex_v("foo bar xX_Baz_Xx"),
        vec![Ident("foo"), Ident("bar"), Ident("xX_Baz_Xx")],
    );
}

#[test]
fn lex_operators() {
    assert_eq!(
        lex_v("+ - * / ^ ( )"),
        vec![Add, Subtract, Multiply, Divide, Exponent, LeftParen, RightParen],
    );
}

#[test]
fn lex_operators_no_space() {
    assert_eq!(
        lex_v("+-*/^()"),
        vec![Add, Subtract, Multiply, Divide, Exponent, LeftParen, RightParen],
    );
}

#[test]
fn lex_excess_spaces() {
    assert_eq!(lex_v("   3 +5  "), vec![Number(3.0), Add, Number(5.0)])
}
