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