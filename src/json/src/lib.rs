mod json_basic;
mod json_dumper;
mod json_impl;
mod json_lexer;
mod json_parser;

use json_lexer::parse_all as lex_string_to_tokens;
use json_parser::parse_all as parse_tokens_to_json;
extern crate macros;

pub use json_basic::FromAndToJson;
pub use json_basic::JsonNode;
pub use json_dumper::dump_json_node;
pub use macros::JsonType;
pub use macros::json;
pub use macros::json_array;
pub use macros::json_object;

pub fn parse_json(input: &str) -> Option<json_basic::JsonNode> {
    let tokens = lex_string_to_tokens(input);
    parse_tokens_to_json(tokens?)
}

impl JsonNode {
    pub fn dump(&self, indent: usize, escape_string: bool) -> String {
        dump_json_node(self, indent, escape_string)
    }
}
