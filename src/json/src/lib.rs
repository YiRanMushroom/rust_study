mod json_basic;
mod json_lexer;
mod json_parser;

use json_lexer::parse_all as lex_string_to_tokens;
use json_parser::parse_all as parse_tokens_to_json;
#[macro_export]
extern crate macros;

pub use macros::JsonStruct;

pub use json_basic::Json as Json;
pub use json_basic::JsonNode as JsonNode;
pub use json_basic::FromAndToJson as FromAndToJson;

pub fn parse_json(input: &str) -> Option<json_basic::Json> {
    let (tokens, success) = lex_string_to_tokens(input);
    if success {
        parse_tokens_to_json(tokens)
    } else {
        None
    }
}
