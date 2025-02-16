use crate::json_impl::JsonToken;
use std::collections::LinkedList;

pub struct JsonLexer<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> JsonLexer<'a> {
    pub fn new(input: &'a str) -> Self {
        JsonLexer { input, pos: 0 }
    }

    fn peek_char(&mut self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn move_to_next(&mut self, c: char) {
        assert_eq!(self.peek_char(), Some(c));
        self.pos += c.len_utf8();
    }

    pub fn next_token(&mut self) -> Option<JsonToken> {
        loop {
            match self.peek_char() {
                None => break None,
                Some(c) => match self.handle_char(c) {
                    Some(token) => break Some(token),
                    None => continue,
                },
            }
        }
    }

    fn handle_char(&mut self, c: char) -> Option<JsonToken> {
        loop {
            match c {
                '"' => break self.handle_string(),
                '0'..='9' | 'e' | 'E' | '.' | '+' | '-' => break self.handle_number(c),
                't' | 'f' => break self.handle_boolean(c),
                'n' => break self.handle_null(),
                ',' => {
                    self.move_to_next(',');
                    break Some(JsonToken::Comma);
                }
                ':' => {
                    self.move_to_next(':');
                    break Some(JsonToken::Colon);
                }
                '{' => {
                    self.move_to_next('{');
                    break Some(JsonToken::LeftBrace);
                }
                '}' => {
                    self.move_to_next('}');
                    break Some(JsonToken::RightBrace);
                }
                '[' => {
                    self.move_to_next('[');
                    break Some(JsonToken::LeftBracket);
                }
                ']' => {
                    self.move_to_next(']');
                    break Some(JsonToken::RightBracket);
                }
                _ => {
                    self.move_to_next(c);
                    if c.is_whitespace() {
                        break None;
                    } else {
                        continue;
                    }
                }
            }
        }
    }

    fn handle_escape(&mut self) -> Option<char> {
        assert_eq!(self.peek_char(), Some('\\'));
        self.move_to_next('\\');
        match self.peek_char() {
            Some('"') => {
                self.move_to_next('"');
                Some('"')
            }
            Some('\\') => {
                self.move_to_next('\\');
                Some('\\')
            }
            Some('/') => {
                self.move_to_next('/');
                Some('/')
            }
            Some('b') => {
                self.move_to_next('b');
                Some('\x08')
            }
            Some('f') => {
                self.move_to_next('f');
                Some('\x0c')
            }
            Some('n') => {
                self.move_to_next('n');
                Some('\n')
            }
            Some('r') => {
                self.move_to_next('r');
                Some('\r')
            }
            Some('t') => {
                self.move_to_next('t');
                Some('\t')
            }
            Some('u') => {
                self.move_to_next('u');
                let mut codepoint = 0;
                for _ in 0..4 {
                    match self.peek_char() {
                        Some(c) if c.is_digit(16) => {
                            codepoint = codepoint * 16 + c.to_digit(16).unwrap();
                            self.move_to_next(c);
                        }
                        _ => return None,
                    }
                }
                std::char::from_u32(codepoint).or(Some('\u{fffd}'))
            }
            _ => None,
        }
    }

    fn handle_string(&mut self) -> Option<JsonToken> {
        assert_eq!(self.peek_char(), Some('"'));
        self.move_to_next('"');

        let mut string = String::new();

        loop {
            match self.peek_char() {
                Some('"') => {
                    self.move_to_next('"');
                    break Some(JsonToken::String(string));
                }
                Some('\\') => match self.handle_escape() {
                    Some(c) => string.push(c),
                    None => return None,
                },
                Some(c) => {
                    string.push(c);
                    self.move_to_next(c);
                }
                None => return None,
            }
        }
    }

    fn handle_number(&mut self, c: char) -> Option<JsonToken> {
        let mut number = String::new();
        number.push(c);
        self.move_to_next(c);

        while let Some(c) = self.peek_char() {
            if c.is_digit(10) || c == '.' || c == 'e' || c == 'E' || c == '+' || c == '-' {
                number.push(c);
                self.move_to_next(c);
            } else {
                break;
            }
        }

        match number.parse() {
            Ok(n) => Some(JsonToken::Number(n)),
            Err(_) => None,
        }
    }

    fn handle_boolean(&mut self, c: char) -> Option<JsonToken> {
        let mut boolean = String::new();
        boolean.push(c);
        self.move_to_next(c);

        while let Some(c) = self.peek_char() {
            if c.is_ascii_alphabetic() {
                boolean.push(c);
                self.move_to_next(c);
            } else {
                break;
            }
        }

        match boolean.as_str() {
            "true" => Some(JsonToken::Boolean(true)),
            "false" => Some(JsonToken::Boolean(false)),
            _ => None,
        }
    }

    fn handle_null(&mut self) -> Option<JsonToken> {
        let mut null = String::new();
        null.push('n');
        self.move_to_next('n');

        while let Some(c) = self.peek_char() {
            if c.is_ascii_alphabetic() {
                null.push(c);
                self.move_to_next(c);
            } else {
                break;
            }
        }

        match null.as_str() {
            "null" => Some(JsonToken::Null),
            _ => None,
        }
    }
}

pub fn parse_all(input: &str) -> Option<LinkedList<JsonToken>> {
    let mut lexer = JsonLexer::new(input);
    let mut tokens = LinkedList::new();

    loop {
        match lexer.next_token() {
            Some(token) => tokens.push_back(token),
            None => break Some(tokens),
        }
    }
}
