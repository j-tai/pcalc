use num::BigInt;

use crate::{Result, Span};

/// A token.
#[derive(Clone, Debug, PartialEq)]
pub enum Token<'a> {
    /// An integer literal.
    Integer(BigInt),
    /// A floating-point literal.
    Float(f64),
    /// An identifier.
    Ident(&'a str),
    /// A plus sign `+`.
    Plus,
    /// A minus sign `-`.
    Minus,
    /// A multiplication sign `*`.
    Times,
    /// A division sign `/`.
    Divide,
    /// An exponentiation sign `^`.
    Exponent,
    /// A left parenthesis `(`.
    LeftParen,
    /// A right parenthesis `)`.
    RightParen,
    /// An equal sign `=`.
    Equals,
    /// A comma `,`.
    Comma,
    /// End of file or input.
    Eof,
}

/// A stream of tokens, similar to an `Iterator<Item = (Token, Span)`.
///
/// `Token::Eof` signals the end of the token stream, and trying to retrieve
/// further tokens results in a panic. If an `Err` is returned, then `peek` and
/// `next` should not be called again; otherwise, the calls may produce panics
/// or undefined results.
pub trait TokenStream<'a> {
    fn peek(&mut self) -> Result<&(Token<'a>, Span)>;
    fn next(&mut self) -> Result<(Token<'a>, Span)>;
}
