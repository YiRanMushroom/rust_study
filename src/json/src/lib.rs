mod json_basic;
mod json_lexer;
mod json_parser;
mod json_impl;

pub use json_basic::FromAndToJson;
pub use json_basic::Json;
pub use json_basic::JsonNode;
pub use macros::JsonType;

pub fn parse_json(input: &str) -> Option<Json> {
    json_parser::parse_all(json_lexer::parse_all(input)?)
}
