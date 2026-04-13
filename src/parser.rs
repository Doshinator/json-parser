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

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch == ' ' || ch == '\n' || ch == '\r' || ch == '\t' {
                self.next();
            }
            else {
                break;
            }
        }
    }

    // // ==================== Entry Point ====================
    // Json := element 
    // element := ws value ws
    pub fn parse(&mut self) -> ParseResult<JsonValue> {
        self.skip_whitespace();

        let value = self.parse_value();

        self.skip_whitespace();

        if self.position < self.input.len() {
            return Err(ParseError::new(
                "Unexpected characters after JSON value",
                self.position,
            ));
        }

        Ok(value)
    }

    // value := object | array | string | number | "true" | "false" | "null"
    fn parse_value(&mut self) -> ParseResult<JsonValue> {
        match self.peek() {
            Some('{') => self.parse_object(),
            Some('[') => self.parse_array(),
            Some('"') => self.parse_string(),

            Some('-') | Some('0'..'9') => self.parse_number(),

            Some('t') => self.parse_true(),
            Some('f') => self.parse_false(),
            Some('n') => self.parse_null(),

            Some(ch) => Err(ParseError::new(
                format!("Unexpected char '{}'", ch),
                self.position
            )),

            None => Err(ParseError::new(
                format!("Unexpected end of input"),
                self.position
            )),
        }
    }

    // boolean | null
    fn parse_true(&mut self) -> ParseResult<JsonValue> {
        self.expect('t')?;
        self.expect('r')?;
        self.expect('u')?;
        self.expect('e')?;
        Ok(JsonValue::Bool(true))
    }

    fn parse_false(&mut self) -> ParseResult<JsonValue> {
        self.expect('f')?;
        self.expect('a')?;
        self.expect('l')?;
        self.expect('s')?;
        self.expect('e')?;
        Ok(JsonValue::Bool(false))
    }

    fn parse_null(&mut self) -> ParseResult<JsonValue> {
        self.expect('n')?;
        self.expect('u')?;
        self.expect('l')?;
        self.expect('l')?;
        Ok(JsonValue::Null)
    }
}
