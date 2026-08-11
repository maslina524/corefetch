mod lexer;
pub mod parser;

use lexer::TokenStream;
use parser::{Value, Parser};
pub use parser::Map;

use core::str::FromStr;

use alloc::{
    string::String,
    collections::BTreeMap
};

use crate::{
    os::path::Path,
    os::fs
};

#[derive(Debug)]
pub struct Json;

impl Json {
    pub fn from_file(path: impl Into<Path>) -> Result<Map, fs::ReadError> {
        let string = fs::read_to_string(path.into())?;
        let map = Self::from_str(&string);
        Ok(map)
    }

    pub fn from_str(s: &str) -> Map {
        let stream = TokenStream::new(s);
        let mut parser = Parser::new(stream);
        parser.parse_object()
    }
}

#[cfg(test)]
mod tests {
    use core::str::FromStr;

    use crate::json::Json;

    #[test]
    fn single_chars_test() {
        let source = r#"{
            "key": "value",
            "array": [80, 42.55, true, null, "string"],
            "dict": {
                "type": "title",
                "idx": 1
            },
            "nullable": null,
            "boolean": true
        }"#;
        
        let json = Json::from_str(source);
        println!("{json:#?}");
    }
}