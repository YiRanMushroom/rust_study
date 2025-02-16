use crate::json_basic::*;
use crate::json_impl::JsonToken;
use std::collections::{HashMap, LinkedList};

struct JsonParser {
    tokens: LinkedList<JsonToken>,
}

impl JsonParser {
    pub fn new(tokens: LinkedList<JsonToken>) -> JsonParser {
        JsonParser { tokens }
    }

    pub fn parse(&mut self) -> Option<Json> {
        let root = self.parse_value();
        match root {
            Some(JsonNode::Object(obj)) => Some(JsonNode::Object(obj).move_as_root()),
            Some(JsonNode::Array(arr)) => Some(JsonNode::Array(arr).move_as_root()),
            _ => None,
        }
    }

    pub fn parse_value(&mut self) -> Option<JsonNode> {
        match self.tokens.pop_front() {
            Some(JsonToken::String(s)) => Some(JsonNode::String(s)),
            Some(JsonToken::Number(n)) => Some(JsonNode::Number(n)),
            Some(JsonToken::Boolean(b)) => Some(JsonNode::Boolean(b)),
            Some(JsonToken::Null) => Some(JsonNode::Null),
            Some(JsonToken::LeftBrace) => self.parse_object(),
            Some(JsonToken::LeftBracket) => self.parse_array(),
            _ => None,
        }
    }

    pub fn parse_object(&mut self) -> Option<JsonNode> {
        let mut obj = HashMap::new();

        loop {
            match self.tokens.pop_front() {
                Some(JsonToken::RightBrace) => break Some(JsonNode::Object(obj)),
                Some(JsonToken::String(key)) => match self.tokens.pop_front() {
                    Some(JsonToken::Colon) => match self.parse_value() {
                        None => break None,
                        Some(value) => {
                            obj.insert(key, value);
                            match self.tokens.pop_front() {
                                Some(JsonToken::Comma) => continue,
                                Some(JsonToken::RightBrace) => break Some(JsonNode::Object(obj)),
                                _ => break None,
                            }
                        }
                    },
                    _ => break None,
                },
                _ => break None,
            }
        }
    }

    pub fn parse_array(&mut self) -> Option<JsonNode> {
        let mut arr = Vec::new();

        loop {
            match self.tokens.pop_front() {
                Some(JsonToken::RightBracket) => break Some(JsonNode::Array(arr)),
                Some(JsonToken::Comma) => continue,
                Some(token) => {
                    self.tokens.push_front(token);
                    match self.parse_value() {
                        Some(value) => arr.push(value),
                        None => break None,
                    }
                }
                None => break None,
            }
        }
    }
}

pub fn parse_all(tokens: LinkedList<JsonToken>) -> Option<Json> {
    let mut parser = JsonParser::new(tokens);
    parser.parse()
}
