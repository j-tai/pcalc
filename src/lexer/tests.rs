use crate::Token::*;
use crate::{lex, Span, Token};

/// Collect the tokens from the input string.
fn v(s: &str) -> Vec<Token> {
    // Take only 64 as a failsafe against infinite loops
    lex(s, None).take(64).map(|(t, _)| t).collect()
}

/// Collect the spans from the input string.
fn s(s: &str) -> Vec<Span> {
    lex(s, None).take(64).map(|(_, s)| s).collect()
}

/// Create a span at line 1 of standard input with the given start and end
/// columns.
fn sp(start: u32, end: u32) -> Span {
    Span {
        file: None,
        line: 1,
        start,
        end,
    }
}

#[test]
fn numbers() {
    assert_eq!(v("1.2345 9876"), vec![Number(1.2345), Number(9876.0), Eof]);
}

#[test]
fn number_with_exponent() {
    assert_eq!(
        v("1.23e4 9876e-4 1e+9"),
        vec![Number(1.23e4), Number(9876e-4), Number(1e+9), Eof],
    );
}

#[test]
fn number_missing_digits() {
    assert_eq!(v(".5 1."), vec![Number(0.5), Number(1.0), Eof]);
}

#[test]
fn idents() {
    assert_eq!(
        v("foo bar xX_Baz_Xx"),
        vec![Ident("foo"), Ident("bar"), Ident("xX_Baz_Xx"), Eof],
    );
}

#[test]
fn operators() {
    assert_eq!(
        v("+ - * / ^ ( ) ="),
        vec![Plus, Minus, Times, Divide, Exponent, LeftParen, RightParen, Equals, Eof],
    );
}

#[test]
fn operators_no_space() {
    assert_eq!(
        v("+-*/^()="),
        vec![Plus, Minus, Times, Divide, Exponent, LeftParen, RightParen, Equals, Eof],
    );
}

#[test]
fn excess_spaces() {
    assert_eq!(v("   3 +5  "), vec![Number(3.0), Plus, Number(5.0), Eof]);
}

#[test]
fn span() {
    assert_eq!(
        s("1.2e3  + (foo)/       ="),
        vec![
            sp(1, 5),
            sp(8, 8),
            sp(10, 10),
            sp(11, 13),
            sp(14, 14),
            sp(15, 15),
            sp(23, 23),
            sp(24, 24),
        ],
    );
    assert_eq!(
        s("3^^3"),
        vec![sp(1, 1), sp(2, 2), sp(3, 3), sp(4, 4), sp(5, 5)],
    );
}
