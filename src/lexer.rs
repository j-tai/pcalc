#[cfg(test)]
mod tests;

use std::rc::Rc;

use crate::{Error, Result, Span, Token, TokenStream};

/// Lex the `input` into a stream of tokens.
///
/// The `file` parameter specifies the filename from which the input
/// originated. This is to produce better diagnostic messages.
pub fn lex(input: &str, file: Option<String>) -> impl TokenStream {
    Lex {
        input,
        file: file.map(Rc::new),
        line: 1,
        col: 1,
        peeked: None,
    }
}

struct Lex<'a> {
    input: &'a str,
    file: Option<Rc<String>>,
    line: u32,
    col: u32,
    peeked: Option<(Token<'a>, Span)>,
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
        let mut is_float = false;
        end = self.read_while(end, |c| c.is_digit(10));
        if self.input[end..].starts_with('.') {
            end += 1;
            end = self.read_while(end, |c| c.is_digit(10));
            is_float = true;
        }
        if let Some('e') | Some('E') = self.input[end..].chars().next() {
            end += 1;
            // Consume a + or -
            if let Some('+') | Some('-') = self.input[end..].chars().next() {
                end += 1;
            }
            // Consume exponent
            end = self.read_while(end, |c| c.is_digit(10));
            is_float = true;
        }
        let s = &self.input[..end];
        let tok = if is_float {
            Token::Float(s.parse().expect("invalid float literal"))
        } else {
            Token::Integer(s.parse().expect("invalid integer literal"))
        };
        (tok, self.advance_span(end))
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
            ',' => Comma,
            _ => return None,
        };
        Some((op, self.advance_span(1)))
    }

    /// Read a token.
    fn read_token(&mut self) -> Result<(Token<'a>, Span)> {
        // Trim whitespace from input
        let n = self.read_while(0, |c| c.is_whitespace());
        self.advance(n);
        // End of input?
        if self.input.is_empty() {
            return Ok((
                Token::Eof,
                Span {
                    file: self.file.clone(),
                    line: self.line,
                    start: self.col,
                    end: self.col,
                },
            ));
        }
        let ch = self.input.chars().next().unwrap();
        // Is this an operator?
        if let Some(tok) = self.read_operator(ch) {
            return Ok(tok);
        }
        // Is this a numeric literal?
        if ch.is_digit(10) || ch == '.' {
            return Ok(self.read_number());
        }
        // Is this an identifier?
        if is_ident_char(ch) {
            return Ok(self.read_ident());
        }
        // Invalid character
        Err((
            Error::Syntax,
            Span {
                file: self.file.clone(),
                line: self.line,
                start: self.col,
                end: self.col,
            },
        ))
    }
}

impl<'a> TokenStream<'a> for Lex<'a> {
    fn peek(&mut self) -> Result<&(Token<'a>, Span)> {
        if let Some(ref tok) = self.peeked {
            Ok(tok)
        } else {
            self.peeked = Some(self.next()?);
            Ok(self.peeked.as_ref().unwrap())
        }
    }

    fn next(&mut self) -> Result<(Token<'a>, Span)> {
        if let Some(tok) = self.peeked.take() {
            Ok(tok)
        } else {
            self.read_token()
        }
    }
}
