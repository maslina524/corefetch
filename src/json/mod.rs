mod lexer;
mod parser;

use lexer::TokenStream;
use parser::{Value, Parser};

use core::str::FromStr;

use alloc::{
    string::String,
    collections::BTreeMap
};

#[derive(Debug)]
pub struct Json {
    root: BTreeMap<String, Value>
}

impl FromStr for Json {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stream = TokenStream::new(s);
        let mut parser = Parser::new(stream);
        let root = parser.parse_object();

        Ok(Self { root })
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
        
        let json = Json::from_str(source).unwrap();

        println!("{json:#?}");
    }
}