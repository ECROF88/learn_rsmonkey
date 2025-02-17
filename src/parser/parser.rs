use std::thread::sleep;

use crate::ast::ast::{Identifier, LetStatement, NodeType, Program};
use crate::lexer::lexer::Lexer;
use crate::token::token::{Token, TokenType};

pub struct Parser {
    l: Lexer,
    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(l: Lexer) -> Self {
        let mut p = Parser {
            l,
            // 初始化为空的Token，将在构造函数中更新
            cur_token: Token {
                token_type: TokenType::ILLEGAL,
                literal: String::new(),
            },
            peek_token: Token {
                token_type: TokenType::ILLEGAL,
                literal: String::new(),
            },
        };

        // 读取两个词法单元，以设置cur_token和peek_token
        p.next_token();
        p.next_token();
        p
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::new();

        // 循环直到遇到 EOF token
        while self.cur_token.token_type != TokenType::EOF {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            }
            self.next_token();
        }

        program
    }

    fn parse_statement(&mut self) -> Option<NodeType> {
        match self.cur_token.token_type {
            TokenType::LET => self.parse_let_statement(),
            // TokenType::RETURN => self.parse_return_statement(),
            _ => None, // 暂时忽略其他类型的语句
        }
    }

    fn parse_let_statement(&mut self) -> Option<NodeType> {
        // 期望下一个 token 是标识符
        if !self.expect_peek(TokenType::IDENT) {
            return None;
        }

        let name = Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        };

        // 期望下一个 token 是赋值符号
        if !self.expect_peek(TokenType::ASSIGN) {
            return None;
        }

        // TODO: 跳过对表达式的处理，直到遇到分号
        while !self.cur_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        Some(NodeType::Statement(Box::new(LetStatement {
            token: Token {
                token_type: TokenType::LET,
                literal: "let".to_string(),
            },
            name: Box::new(name),
            value: Box::new(Identifier {
                // Temporary placeholder until expression parsing is implemented
                token: Token {
                    token_type: TokenType::IDENT,
                    literal: String::new(),
                },
                value: String::new(),
            }),
        })))
    }

    // 辅助方法
    fn cur_token_is(&self, t: TokenType) -> bool {
        self.cur_token.token_type == t
    }

    fn peek_token_is(&self, t: TokenType) -> bool {
        self.peek_token.token_type == t
    }

    fn expect_peek(&mut self, t: TokenType) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            true
        } else {
            false
        }
    }
}
