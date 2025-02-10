pub mod json_basic;
pub mod json_lexer;
pub mod json_parser;

pub use json_lexer::parse_all as lex_string_to_tokens;
pub use json_parser::parse_all as parse_tokens_to_json;

pub fn parse_json(input: &str) -> Option<json_basic::Json> {
    let (tokens, success) = lex_string_to_tokens(input);
    if success {
        parse_tokens_to_json(tokens)
    } else {
        None
    }
}
