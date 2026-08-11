use core::str::FromStr;

use alloc::{
    string::String,
    vec::Vec,
    borrow::ToOwned,
    collections::BTreeMap,
};

use crate::json::lexer::{Token, TokenStream};

#[derive(Debug)]
pub enum Value {
    String(String),
    Number(f64),
    Bool(bool),
    Array(Vec<Self>),
    Dict(BTreeMap<String, Self>),
    Null
}

struct Parser {
    iter: TokenStream,
    current: Option<Token>,
}

impl Parser {
    fn new(mut iter: TokenStream) -> Self {
        let current = iter.next();
        Self { iter, current }
    }

    const fn peek(&self) -> Option<&Token> {
        self.current.as_ref()
    }

    fn next(&mut self) -> Option<Token> {
        let token = self.current.take();
        self.current = self.iter.next();
        token
    }

    fn consume(&mut self, expected: &Token) -> Token {
        let token = self.next().expect("Unexpected end of input");
        if token == *expected {
            token
        } else {
            panic!("Expected {expected:?}, got {token:?}");
        }
    }

    fn parse_value(&mut self) -> Value {
        match self.peek() {
            Some(Token::LCurly) => {
                self.next();
                Value::Dict(self.parse_object())
            }
            Some(Token::LBrace) => {
                self.next();
                Value::Array(self.parse_array())
            }
            Some(Token::String(s)) => {
                let s = s.clone();
                self.next();
                let content = s[1..s.len() - 1].to_owned();
                Value::String(content)
            }
            Some(Token::Number(s)) => {
                let s = s.clone();
                self.next();
                let num = f64::from_str(&s).expect("Invalid number format");
                Value::Number(num)
            }
            Some(Token::True) => {
                self.next();
                Value::Bool(true)
            }
            Some(Token::False) => {
                self.next();
                Value::Bool(false)
            }
            Some(Token::Null) => {
                self.next();
                Value::Null
            }
            _ => panic!("Unexpected token while parsing value"),
        }
    }

    fn parse_object(&mut self) -> BTreeMap<String, Value> {
        let mut map = BTreeMap::new();

        if matches!(self.peek(), Some(Token::RCurly)) {
            self.next();
            return map;
        }

        loop {
            let key_token = self.next().expect("Expected object key");
            let key = match key_token {
                Token::String(s) => s[1..s.len() - 1].to_owned(),
                _ => panic!("Object key must be a string"),
            };

            self.consume(&Token::Colon);

            let value = self.parse_value();
            map.insert(key, value);

            match self.peek() {
                Some(Token::Comma) => {
                    self.next();
                }
                Some(Token::RCurly) => {
                    self.next();
                    break;
                }
                _ => panic!("Expected ',' or '}}' in object"),
            }
        }

        map
    }

    fn parse_array(&mut self) -> Vec<Value> {
        let mut vec = Vec::new();

        if matches!(self.peek(), Some(Token::RBrace)) {
            self.next();
            return vec;
        }

        loop {
            let value = self.parse_value();
            vec.push(value);

            match self.peek() {
                Some(Token::Comma) => {
                    self.next();
                }
                Some(Token::RBrace) => {
                    self.next();
                    break;
                }
                _ => panic!("Expected ',' or ']' in array"),
            }
        }

        vec
    }
}
