mod json_basic;
mod json_dumper;
mod json_impl;
mod json_lexer;
mod json_parser;

pub use json_basic::FromAndToJson;
pub use json_basic::JsonNode;
pub use macros::JsonType;

pub fn parse_json(input: &str) -> Option<JsonNode> {
    json_parser::parse_all(json_lexer::parse_all(input)?)
}

impl JsonNode {
    pub fn dump(&self, indent: usize) -> String {
        json_dumper::dump_json_node(&self, indent)
    }
}
