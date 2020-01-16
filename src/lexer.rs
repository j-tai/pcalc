#[cfg(test)]
mod tests;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Token<'a> {
    /// A numeric literal.
    Number(f64),
    /// An identifier.
    Ident(&'a str),
    /// A plus sign `+`.
    Add,
    /// A minus sign `-`.
    Subtract,
    /// A multiplication sign `*`.
    Multiply,
    /// A division sign `/`.
    Divide,
    /// An exponentiation sign `^`.
    Exponent,
    /// A left parenthesis `(`.
    LeftParen,
    /// A right parenthesis `)`.
    RightParen,
    /// Catch-all for invalid tokens.
    String(&'a str),
}

pub fn lex(input: &str) -> impl Iterator<Item = Token> {
    Lex { input }
}

struct Lex<'a> {
    input: &'a str,
}

/// Return whether this character is acceptable in an identifier.
fn is_ident_char(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_'
}

impl<'a> Lex<'a> {
    fn read_while<F: FnMut(char) -> bool>(&mut self, mut f: F) -> &'a str {
        let mut end;
        let mut char_indices = self.input.char_indices();
        loop {
            if let Some((i, ch)) = char_indices.next() {
                end = i;
                if !f(ch) {
                    // End of token reached
                    break;
                }
            } else {
                end = self.input.len();
                break;
            }
        }
        let s = &self.input[..end];
        self.input = &self.input[end..];
        s
    }

    /// Read a numeric literal.
    fn read_number(&mut self) -> Token<'a> {
        let s = self.read_while(|c| is_ident_char(c) || c == '.');
        match s.parse() {
            Ok(n) => Token::Number(n),
            Err(_) => Token::String(s),
        }
    }

    /// Read an identifier.
    fn read_ident(&mut self) -> Token<'a> {
        Token::Ident(self.read_while(is_ident_char))
    }

    /// Read an operator.
    fn read_operator(&mut self, ch: char) -> Option<Token<'a>> {
        use Token::*;
        let op = match ch {
            '+' => Add,
            '-' => Subtract,
            '*' => Multiply,
            '/' => Divide,
            '^' => Exponent,
            '(' => LeftParen,
            ')' => RightParen,
            _ => return None,
        };
        self.input = &self.input[1..];
        Some(op)
    }

    /// Read a token.
    fn read_token(&mut self) -> Option<Token<'a>> {
        self.input = self.input.trim_start();
        // End of input?
        if self.input.is_empty() {
            return None;
        }
        let ch = self.input.chars().next().unwrap();
        // Is this a token?
        if let Some(tok) = self.read_operator(ch) {
            return Some(tok);
        }
        // Is this a numeric literal?
        if ch.is_digit(10) || ch == '.' {
            return Some(self.read_number());
        }
        // Is this an identifier?
        if is_ident_char(ch) {
            return Some(self.read_ident());
        }
        // Backup plan...
        let len = ch.len_utf8();
        let s = &self.input[..len];
        self.input = &self.input[len..];
        Some(Token::String(s))
    }
}

impl<'a> Iterator for Lex<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Token<'a>> {
        self.read_token()
    }
}
