use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(u64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}


pub fn parse_json(input: &str) -> Result<JsonValue, String> {
    let input = input.trim();
    match input {
        "null" => Ok(JsonValue::Null),
        _ => panic!("Invalid json format"),
    }
}