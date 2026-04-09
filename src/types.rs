use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    
    // JSON object - unordered key-value pairs
    Object(HashMap<String, JsonValue>),
}