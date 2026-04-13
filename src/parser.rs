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

    // ----- String parser -----
    fn parse_string(&mut self) -> ParseResult<JsonValue> {
        self.expect('"')?;

        let mut result = String::new();

        loop {
            match self.next() {
                Some('"') => return Ok(JsonValue::String(result)),
                Some('\\') => {
                    match self.next() {
                        Some('"') => result.push('"'),
                        Some('\\') => result.push('\\'),
                        Some('/') => result.push('/'),
                        Some('b') => result.push('\u{0008}'),  // backspace
                        Some('f') => result.push('\u{000C}'),  // form feed
                        Some('n') => result.push('\n'),
                        Some('r') => result.push('\r'),
                        Some('t') => result.push('\t'),
                        Some('u') => {
                            // Unicode escape: \uXXXX
                            // Need to parse 4 hex digits and convert to char
                            let ch = self.parse_unicode_escape()?;
                            result.push(ch);
                        },
                        Some(ch) => {
                            return Err(ParseError::new(
                                format!("Invalid escape sequence '\\{}'", ch), 
                                self.position - 1));
                        },
                        None => {
                            return Err(ParseError::new(
                                "Unexpected end of input in escape sequence",
                                self.pos
                            ));
                        }
                    }
                },
                Some(ch) if ch < ' ' => {
                    return Err(ParseError::new(
                        format!("Unescaped control character (code: {})", ch as u32),
                        self.position - 1,
                    ));
                },
                Some(ch) => {
                    result.push(ch);
                },
                None => {
                    return Err(ParseError::new(
                        "Unexpected end of input in string",
                        self.position
                    ));
                },
            }
        }
    }

    fn parse_unicode_escape(&mut self) -> ParseResult<char> {
        let mut hex_str = String::new();

        for _ in 0..4 {
            match self.next() {
                Some(ch) if ch.is_ascii_hexdigit() => {
                    hex_str.push(ch);
                },
                Some(ch) => {
                    return Err(ParseError::new(
                        format!("Expected hex digit in Unicode escape, found '{}'", ch), 
                        self.position - 1
                    ));
                },
                None => {
                    return Err(ParseError::new(
                        "Unexpected end of input in Unicode escape",
                        self.position,
                    ));
                },
            }
        }
        
        // Can't fail - we verified all chars are hex above
        let code = u32::from_str_radix(&hex_str, 16).unwrap();
        
        char::from_u32(code).ok_or_else(|| {
            ParseError::new(
                format!("Invalid Unicode code point: U+{:04X}", code),
                self.position - 4,
            )
        })
    }

}
