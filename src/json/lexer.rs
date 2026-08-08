use alloc::{
    string::String,
    vec::Vec
};

#[derive(Debug)]
pub enum Token {
    // Literals
    String(String),
    Num(f64),
    Bool(bool),
    Null,

    // Paren
    LCurly, RCurly,
    LBrace, RBrace,

    Comma,
    Colon
}