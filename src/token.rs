use crate::{Result, Span};

/// A token.
#[derive(Clone, Debug, PartialEq)]
pub enum Token<'a> {
    /// An integer literal.
    Integer(u64),
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

impl From<u64> for Token<'static> {
    fn from(num: u64) -> Token<'static> {
        Token::Integer(num)
    }
}

impl From<f64> for Token<'static> {
    fn from(num: f64) -> Token<'static> {
        Token::Float(num)
    }
}

impl<'a> From<&'a str> for Token<'a> {
    fn from(num: &'a str) -> Token<'a> {
        Token::Ident(num)
    }
}
