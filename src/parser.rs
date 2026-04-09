use crate::error::{ParseError, ParseResult};
use crate::types::JsonValue;

pub struct Parser {
    input: Vec<char>,
    position: usize,
}

impl Parser {
    fn new(input: &str) -> Self {
        Parser { 
            input: input.chars().collect(), 
            position: 0 
        }
    }
}

fn parse(input: &str) -> ParseResult<JsonValue> {
    let mut parse = Parser::new(input);
    Ok(JsonValue::Null)
}