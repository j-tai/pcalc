#[cfg(test)]
mod tests;

use std::rc::Rc;

use crate::{Span, Token};

pub fn lex(input: &str, file: Option<String>) -> impl Iterator<Item = (Token, Span)> {
    Lex {
        input,
        file: file.map(Rc::new),
        line: 1,
        col: 1,
    }
}

struct Lex<'a> {
    input: &'a str,
    file: Option<Rc<String>>,
    line: u32,
    col: u32,
}

/// Return whether this character is acceptable in an identifier.
fn is_ident_char(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_'
}

impl<'a> Lex<'a> {
    fn advance(&mut self, amt: usize) {
        let s = &self.input[..amt];
        self.input = &self.input[amt..];
        for ch in s.chars() {
            match ch {
                '\n' => {
                    self.line += 1;
                    self.col = 1;
                }
                _ => self.col += 1,
            }
        }
    }

    fn advance_span(&mut self, amt: usize) -> Span {
        let start = self.col;
        let line = self.line;
        self.advance(amt);
        assert_eq!(self.line, line);
        let end = self.col - 1;
        Span {
            file: self.file.clone(),
            line,
            start,
            end,
        }
    }

    fn read_while<F: FnMut(char) -> bool>(&self, start: usize, mut f: F) -> usize {
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
    fn read_number(&mut self) -> (Token<'a>, Span) {
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
        (
            Token::Number(s.parse().expect("invalid numeric literal")),
            self.advance_span(end),
        )
    }

    /// Read an identifier.
    fn read_ident(&mut self) -> (Token<'a>, Span) {
        let end = self.read_while(0, is_ident_char);
        let s = &self.input[..end];
        (Token::Ident(s), self.advance_span(end))
    }

    /// Read an operator.
    fn read_operator(&mut self, ch: char) -> Option<(Token<'a>, Span)> {
        use Token::*;
        let op = match ch {
            '+' => Plus,
            '-' => Minus,
            '*' => Times,
            '/' => Divide,
            '^' => Exponent,
            '(' => LeftParen,
            ')' => RightParen,
            '=' => Equals,
            _ => return None,
        };
        Some((op, self.advance_span(1)))
    }

    /// Read a token.
    fn read_token(&mut self) -> Option<(Token<'a>, Span)> {
        // Trim whitespace from input
        let n = self.read_while(0, |c| c.is_whitespace());
        self.advance(n);
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
    type Item = (Token<'a>, Span);

    fn next(&mut self) -> Option<(Token<'a>, Span)> {
        self.read_token()
    }
}
