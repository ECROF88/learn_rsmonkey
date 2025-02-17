use crate::token::token::{Token, TokenType};
use std::collections::HashMap;

pub struct Lexer {
    keywords: HashMap<String, TokenType>,
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut keywords = HashMap::new();
        keywords.insert("fn".to_string(), TokenType::FN);
        keywords.insert("let".to_string(), TokenType::LET);
        keywords.insert("if".to_string(), TokenType::IF);
        keywords.insert("else".to_string(), TokenType::ELSE);
        keywords.insert("return".to_string(), TokenType::RETURN);
        keywords.insert("true".to_string(), TokenType::TRUE);
        keywords.insert("false".to_string(), TokenType::FALSE);

        let mut l = Lexer {
            input: input,
            position: 0,
            read_position: 0,
            ch: '\0',
            keywords,
        };
        l.read_char();
        l
    }
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap_or('\0');
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn is_letter(ch: char) -> bool {
        ch.is_ascii_alphabetic() || ch == '_'
    }

    fn is_digit(ch: char) -> bool {
        ch.is_ascii_digit()
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input.chars().nth(self.read_position).unwrap_or('\0')
        }
    }

    fn read_identifier(&mut self) -> String {
        let identifier_start_position = self.position;
        while Self::is_letter(self.ch) {
            self.read_char();
        }
        self.input[identifier_start_position..self.position].to_string()
    }

    fn read_number(&mut self) -> String {
        let number_start_position = self.position;
        while Self::is_digit(self.ch) {
            self.read_char();
        }
        self.input[number_start_position..self.position].to_string()
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }

    fn read_string(&mut self) -> String {
        let string_start_position = self.position + 1;
        self.read_char();
        while self.ch != '"' && self.ch != '\0' {
            self.read_char();
        }
        let str = self.input[string_start_position..self.position].to_string();
        // 跳过结束的引号
        self.read_char();
        str
    }
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok: Token;

        match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    tok = Token {
                        token_type: TokenType::EQ,
                        literal: "==".to_string(),
                    };
                } else {
                    tok = Token {
                        token_type: TokenType::ASSIGN,
                        literal: "=".to_string(),
                    };
                }
            }
            '+' => {
                tok = Token {
                    token_type: TokenType::PLUS,
                    literal: "+".to_string(),
                };
            }
            '(' => {
                tok = Token {
                    token_type: TokenType::LPAREN,
                    literal: "(".to_string(),
                };
            }
            ')' => {
                tok = Token {
                    token_type: TokenType::RPAREN,
                    literal: ")".to_string(),
                };
            }
            '{' => {
                tok = Token {
                    token_type: TokenType::LBRACE,
                    literal: "{".to_string(),
                };
            }
            '}' => {
                tok = Token {
                    token_type: TokenType::RBRACE,
                    literal: "}".to_string(),
                };
            }
            ',' => {
                tok = Token {
                    token_type: TokenType::COMMA,
                    literal: ",".to_string(),
                };
            }
            ';' => {
                tok = Token {
                    token_type: TokenType::SEMICOLON,
                    literal: ";".to_string(),
                };
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    tok = Token {
                        token_type: TokenType::NOTEQ,
                        literal: "!=".to_string(),
                    };
                } else {
                    tok = Token {
                        token_type: TokenType::ILLEGAL,
                        literal: self.ch.to_string(),
                    };
                }
            }
            '-' => {
                tok = Token {
                    token_type: TokenType::MINUS,
                    literal: "-".to_string(),
                };
            }
            '*' => {
                tok = Token {
                    token_type: TokenType::ASTERISK,
                    literal: "*".to_string(),
                };
            }
            '/' => {
                tok = Token {
                    token_type: TokenType::SLASH,
                    literal: "/".to_string(),
                };
            }
            '<' => {
                tok = Token {
                    token_type: TokenType::LT,
                    literal: "<".to_string(),
                };
            }
            '>' => {
                tok = Token {
                    token_type: TokenType::GT,
                    literal: ">".to_string(),
                };
            }
            '[' => {
                tok = Token {
                    token_type: TokenType::LBRACKET,
                    literal: "[".to_string(),
                };
            }
            ']' => {
                tok = Token {
                    token_type: TokenType::RBRACKET,
                    literal: "]".to_string(),
                };
            }
            ':' => {
                tok = Token {
                    token_type: TokenType::COLON,
                    literal: ":".to_string(),
                };
            }
            '"' => {
                let str_value = self.read_string();
                tok = Token {
                    token_type: TokenType::STRING,
                    literal: str_value,
                };
                return tok;
            }
            '\0' => {
                tok = Token {
                    token_type: TokenType::EOF,
                    literal: "".to_string(),
                };
            }
            _ => {
                if Self::is_letter(self.ch) {
                    let identifier = self.read_identifier();
                    let token_type = self
                        .keywords
                        .get(&identifier)
                        .cloned()
                        .unwrap_or(TokenType::IDENT);
                    // 如果不是关键字就是IDENT
                    return Token {
                        token_type,
                        literal: identifier,
                    };
                } else if Self::is_digit(self.ch) {
                    return Token {
                        token_type: TokenType::INT,
                        literal: self.read_number(),
                    };
                } else {
                    tok = Token {
                        token_type: TokenType::ILLEGAL,
                        literal: self.ch.to_string(),
                    };
                }
            }
        }

        self.read_char();
        tok
    }
}
