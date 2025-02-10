use std::iter::Map;

#[derive(Debug)]
pub enum JsonToken {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
    Comma,
    Colon,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Error,
}

#[derive(Debug)]
pub enum JsonNode {
    Object(Map<String, JsonNode>),
    Array(Vec<JsonNode>),
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}
