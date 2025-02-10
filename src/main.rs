use std::io::stdin;

mod app;
use crate::app::json_lexer;

fn main() {
    println!("Hello, world!");

    let mut input = String::new();

    match stdin().read_line(&mut input) {
        Ok(_) => {
            let (tokens, success) = json_lexer::parse_all(&input);
            if success {
                for (idx, token) in tokens.iter().enumerate() {
                    println!("Token {}: {:?}", idx, token);
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
