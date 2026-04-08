## Domain
1. What is JSON
2. What are the rules?
3. What does success look like? What are we parsing TO?
4. What are the edge cases?

## Language / Grammar
json := 
    element

elements := 
    element
    element ',' elements

element :=
    ws value ws

ws :=
    ""
    '0020' ws
    '000A' ws
    '000D' ws
    '0009' ws


value := object | array | string | number | boolean | "null"

boolean := true | false

string := '"' characters '"'

array := '[' ws ']' | '[' elements ']'

object := '{' ws '}' | '{' members '}'

members := member | member ',' members 

member := ws string ws ':' element 


## Input

let json_str = r#"{"name" : "Jhon", "age" : "30"}"#;
let parsed = parse_json(json_str);

return type could be 
enum JsonValue {
    Null,
    Boolean,
    Number(u64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>)
}

## Breaking down the problem
1. define the grammar
2. parse Null
3. parse boolean
3. Parse numbers
4. Parse strings
5. Parse arrays
6. Parse objects
7. skip whitespace
8. error handle