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
    fn read_while<F: FnMut(char) -> bool>(&mut self, start: usize, mut f: F) -> usize {
        let mut end;
        let mut char_indices = self.input[start..].char_indices();
        loop {
            if let Some((i, ch)) = char_indices.next() {
                end = i;
                if !f(ch) {
                    // End of token reached
                    break;
                }
            } else {
                return self.input.len();
            }
        }
        start + end
    }

    /// Read a numeric literal.
    fn read_number(&mut self) -> Token<'a> {
        let mut end = 0;
        end = self.read_while(end, |c| c.is_digit(10));
        if self.input[end..].chars().next() == Some('.') {
            end = end + 1;
            end = self.read_while(end, |c| c.is_digit(10));
        }
        if let Some('e') | Some('E') = self.input[end..].chars().next() {
            end = end + 1;
            // Consume a + or -
            if let Some('+') | Some('-') = self.input[end..].chars().next() {
                end = end + 1;
            }
            // Consume exponent
            end = self.read_while(end, |c| c.is_digit(10));
        }
        let s = &self.input[..end];
        self.input = &self.input[end..];
        Token::Number(s.parse().expect("invalid numeric literal"))
    }

    /// Read an identifier.
    fn read_ident(&mut self) -> Token<'a> {
        let end = self.read_while(0, is_ident_char);
        let s = &self.input[..end];
        self.input = &self.input[end..];
        Token::Ident(s)
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
        panic!("invalid character: {:?}", ch)
    }
}

impl<'a> Iterator for Lex<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Token<'a>> {
        self.read_token()
    }
}
