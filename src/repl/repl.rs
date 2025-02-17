use std::io::{self, Write};

use crate::{lexer::lexer::Lexer, token::token::TokenType};

const PROMPT: &str = ">> ";

pub fn start() {
    let stdin = io::stdin();

    loop {
        print!("{}", PROMPT);
        io::stdout().flush().expect("fail");

        let mut input = String::new();
        match stdin.read_line(&mut input) {
            Ok(n) if n > 0 => {
                let mut l = Lexer::new(input.trim().to_string());
                loop {
                    let tok = l.next_token();
                    if tok.token_type == TokenType::EOF {
                        break;
                    }
                    println!("{:?}", tok);
                }
            }
            _ => break,
        }
    }
}
