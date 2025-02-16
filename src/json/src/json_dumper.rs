use crate::json_basic::*;

struct JsonDumper {
    current_indent: usize,
    indent_size: usize,
    result: String,
}

pub fn dump_json_node(json: &JsonNode, indent: usize) -> String {
    let mut dumper = JsonDumper {
        current_indent: 0,
        indent_size: indent,
        result: String::new(),
    };
    dumper.process(json);
    dumper.result
}

impl JsonDumper {
    fn indent(&mut self) {
        for _ in 0..self.current_indent {
            self.result.push(' ');
        }
    }
    pub fn process(&mut self, json: &JsonNode) {
        match json {
            JsonNode::Object(obj) => {
                if obj.len() == 0 {
                    self.result.push_str("{}");
                    return;
                }

                self.result.push_str("{\n");
                self.current_indent += self.indent_size;
                for (idx, (key, value)) in obj.iter().enumerate() {
                    self.indent();
                    self.result.push('"');
                    self.result.push_str(key);
                    self.result.push_str("\": ");
                    self.process(value);
                    if idx < obj.len() - 1 {
                        self.result.push_str(",\n");
                    } else {
                        self.result.push('\n');
                    }
                }
                self.current_indent -= self.indent_size;
                self.indent();
                self.result.push('}');
            }
            JsonNode::Array(arr) => {
                if arr.len() == 0 {
                    self.result.push_str("[]");
                    return;
                }

                self.result.push_str("[\n");
                self.current_indent += self.indent_size;
                for (idx, value) in arr.iter().enumerate() {
                    self.indent();
                    self.process(value);
                    if idx < arr.len() - 1 {
                        self.result.push_str(",\n");
                    } else {
                        self.result.push('\n');
                    }
                }
                self.current_indent -= self.indent_size;
                self.indent();
                self.result.push(']');
            }
            JsonNode::String(s) => {
                self.result.push('"');
                self.result.push_str(s);
                self.result.push('"');
            }
            JsonNode::Number(n) => {
                self.result.push_str(&n.to_string());
            }
            JsonNode::Boolean(b) => {
                self.result.push_str(if *b { "true" } else { "false" });
            }
            JsonNode::Null => {
                self.result.push_str("null");
            }
        }
    }
}
