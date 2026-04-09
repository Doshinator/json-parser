use crate::error::{ParseError, ParseResult};
use crate::types::JsonValue;

// Used to store input json and keep track of position
pub struct Parser {
    input: Vec<char>,
    position: usize,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        Parser { 
            input: input.chars().collect(), 
            position: 0 
        }
    }


    fn parse(input: &str) -> ParseResult<JsonValue> {
        let mut parse = Parser::new(input);
        Ok(JsonValue::Null)
    }

    fn peek(&self) -> Option<char> {
        if self.position < self.input.len() {
            Some(self.input[self.position])
        }
        else {
            None
        }
    }

    fn next(&mut self) -> Option<char> {
        if self.position < self.input.len() {
            let ch = self.input[self.position];
            self.position += 1;
            Some(ch)
        }
        else {
            None
        }
    }

    fn expect(&mut self, expected: char) -> ParseResult<()> {
        match self.next() {
            Some(ch) if ch == expected => Ok(()),
            Some(ch) => Err(ParseError::new(
                format!("Expected '{}', found '{}'", expected, ch),
                self.position - 1 // we've advanced so -1 for err char
            )),
            None => Err(ParseError::new(
                format!("Expected '{}', found end of input", expected),
                self.position
            )),
        }
    }
}
