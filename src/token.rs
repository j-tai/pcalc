#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Token<'a> {
    /// A numeric literal.
    Number(f64),
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
