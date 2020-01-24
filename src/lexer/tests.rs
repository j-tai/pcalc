use crate::Token::*;
use crate::{lex, Span, Token,TokenStream};

/// Collect the tokens and spans from the input string.
fn collect(s: &str) -> Vec<(Token, Span)> {
    let mut v = vec![];
    let mut l = lex(s, None);
    loop {
        let tok = l.next().unwrap();
        if tok.0 == Eof {
            v.push(tok);
            break;
        } else {
            v.push(tok);
        }
        if v.len() > 64 {
            // Take only 64 as a failsafe against infinite loops
            break;
        }
    }
    v
}

/// Collect the tokens from the input string.
fn v(s: &str) -> Vec<Token> {
    collect(s).into_iter().map(|(t, _)| t).collect()
}

/// Collect the spans from the input string.
fn s(s: &str) -> Vec<Span> {
    collect(s).into_iter().map(|(_, s)| s).collect()
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
        v("+ - * / ^ ( ) = ,"),
        vec![Plus, Minus, Times, Divide, Exponent, LeftParen, RightParen, Equals, Comma, Eof],
    );
}

#[test]
fn operators_no_space() {
    assert_eq!(
        v("+-*/^()=,"),
        vec![Plus, Minus, Times, Divide, Exponent, LeftParen, RightParen, Equals, Comma, Eof],
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

#[test]
fn peek() {
    let mut l = lex("1 2 3 4 5", None);
    assert_eq!(l.peek(), Ok(&(Number(1.0), sp(1, 1))));
    assert_eq!(l.peek(), Ok(&(Number(1.0), sp(1, 1))));
    assert_eq!(l.next(), Ok((Number(1.0), sp(1, 1))));
    assert_eq!(l.next(), Ok((Number(2.0), sp(3, 3))));
    assert_eq!(l.peek(), Ok(&(Number(3.0), sp(5, 5))));
    assert_eq!(l.peek(), Ok(&(Number(3.0), sp(5, 5))));
    assert_eq!(l.next(), Ok((Number(3.0), sp(5, 5))));
    assert_eq!(l.peek(), Ok(&(Number(4.0), sp(7, 7))));
    assert_eq!(l.next(), Ok((Number(4.0), sp(7, 7))));
    assert_eq!(l.next(), Ok((Number(5.0), sp(9, 9))));
    assert_eq!(l.peek(), Ok(&(Eof, sp(10, 10))));
    assert_eq!(l.peek(), Ok(&(Eof, sp(10, 10))));
    assert_eq!(l.next(), Ok((Eof, sp(10, 10))));
}
