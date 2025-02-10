use crate::app::json_basic::Json;
use std::io::stdin;

mod app;
use crate::app::{json_lexer, json_parser};

extern crate macros;
use macros::print_ident;

fn main() {
    println!("Hello, world!");

    let mut input = String::new();

    print_ident!(input);

    match stdin().read_line(&mut input) {
        Ok(_) => {
            let (tokens, success) = json_lexer::parse_all(&input);
            if success {
                for (idx, token) in tokens.iter().enumerate() {
                    println!("Token {}: {:?}", idx, token);
                }
                let json = json_parser::parse_all(tokens);
                match json {
                    Some(j) => println!("Parsed JSON: {}", j.to_string()),
                    None => eprintln!("Error parsing JSON"),
                }
            } else {
                eprintln!("Error parsing input");
                for (idx, token) in tokens.iter().enumerate() {
                    eprintln!("Token {}: {:?}", idx, token);
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading input: {}", e);
        }
    }
}
