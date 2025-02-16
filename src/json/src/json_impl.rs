#[derive(Debug, Clone)]
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
}