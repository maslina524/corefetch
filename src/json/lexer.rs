use core::str::Chars;

use alloc::{
    string::String,
    vec::Vec,
    borrow::ToOwned
};

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    // Literals
    String(String),
    Num(String),
    False,
    True,
    Null,

    // Paren
    LCurly, RCurly,
    LBrace, RBrace,

    Comma,
    Colon
}

pub struct TokenStream {
    chars: Vec<char>,
    pos: usize
}

impl TokenStream {
    pub fn new(source: &str) -> Self {
        Self { chars: source.chars().collect(), pos: 0 }
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.chars.len() && self.chars[self.pos].is_whitespace() {
            self.pos += 1;
        }
    }

    fn read_single(&self) -> Option<Token> {
        match self.chars[self.pos] {
            '{' => Some(Token::LCurly),
            '}' => Some(Token::RCurly),
            '[' => Some(Token::LBrace),
            ']' => Some(Token::RBrace),
            ',' => Some(Token::Comma),
            ':' => Some(Token::Colon),
            _ => None
        }
    }

    fn read_string(&mut self) -> Token {
        self.pos += 1;
        let mut ret = String::new();

        while self.pos < self.chars.len() && self.chars[self.pos] != '"' {
            ret.push(self.chars[self.pos]);
            self.pos += 1;
        }
        self.pos += 1;

        Token::String(ret)
    }
}

impl Iterator for TokenStream {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        while self.pos < self.chars.len() {
            self.skip_whitespace();

            if let Some(t) = self.read_single() {
                self.pos += 1;
                return Some(t);
            }

            if self.chars[self.pos] == '"' {
                return Some(self.read_string());
            }

            self.pos += 1;
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::json::lexer::{Token, TokenStream};

    #[test]
    fn single_chars_test() {
        let source = "{}: [,]";
        let mut stream = TokenStream::new(source);

        assert_eq!(stream.next(), Some(Token::LCurly));
        assert_eq!(stream.next(), Some(Token::RCurly));
        assert_eq!(stream.next(), Some(Token::Colon));
        assert_eq!(stream.next(), Some(Token::LBrace));
        assert_eq!(stream.next(), Some(Token::Comma));
        assert_eq!(stream.next(), Some(Token::RBrace));
    }

    #[test]
    fn string_test() {
        let source = "{\"key\": \"value\"}";
        let mut stream = TokenStream::new(source);

        assert_eq!(stream.next(), Some(Token::LCurly));
        assert_eq!(stream.next(), Some(Token::String("key".to_owned())));
        assert_eq!(stream.next(), Some(Token::Colon));
        assert_eq!(stream.next(), Some(Token::String("value".to_owned())));
        assert_eq!(stream.next(), Some(Token::RCurly));
    }
}