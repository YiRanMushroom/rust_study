use std::collections::HashMap;
use std::fmt::{Display, Formatter, Pointer};
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
    Object(HashMap<String, JsonNode>),
    Array(Vec<JsonNode>),
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}

pub struct Json {
    root: JsonNode,
}

impl Json {
    pub fn new(root: JsonNode) -> Json {
        Json { root }
    }

    pub fn get_root(&self) -> &JsonNode {
        &self.root
    }

    pub fn get_root_mut(&mut self) -> &mut JsonNode {
        &mut self.root
    }
}

impl Display for JsonNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonNode::Object(obj) => {
                write!(f, "{{")?;
                for (idx, (k, v)) in obj.iter().enumerate() {
                    write!(f, "\"{}\":", k)?;
                    (*v).fmt(f)?;
                    if idx != obj.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "}}")?;
            }
            JsonNode::Array(arr) => {
                write!(f, "[")?;
                for (idx, v) in arr.iter().enumerate() {
                    (*v).fmt(f)?;
                    if idx != arr.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")?;
            }
            JsonNode::String(s) => {
                write!(f, "\"{}\"", s)?;
            }
            JsonNode::Number(n) => {
                write!(f, "{}", n)?;
            }
            JsonNode::Boolean(b) => {
                write!(f, "{}", b)?;
            }
            JsonNode::Null => {
                write!(f, "null")?;
            }
        };

        Ok(())
    }
}

impl Display for Json {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.root.fmt(f)
    }
}
