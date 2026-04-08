use json_parser::expr::{JsonValue, parse_json};

#[test]
fn parse_null_expr() {
    let s: &str = "null";

    let result = parse_json(&s);

    assert_eq!(Ok(JsonValue::Null), result);
}

#[test]
fn parse_null_with_whitespace() {
    let result = parse_json("  null  ");
    assert_eq!(Ok(JsonValue::Null), result);
}