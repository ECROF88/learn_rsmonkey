use crate::ast::ast::{Identifier, LetStatement, NodeType, Program, ReturnStatement};
use crate::lexer::lexer::Lexer;
use crate::token::token::{Token, TokenType};

pub struct Parser {
    l: Lexer,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<String>,
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
            errors: Vec::<String>::new(),
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

    pub fn errors(&self) -> Vec<String> {
        return self.errors.clone();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::new();

        // 循环直到遇到 EOF token
        while !self.cur_token_is(TokenType::EOF) {
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
            TokenType::RETURN => self.parse_return_statement(),
            _ => {
                // println!("parse_statement get None !");
                None
            }
        }
    }

    fn parse_return_statement(&mut self) -> Option<NodeType> {
        self.next_token();

        let value = match self.parse_expression() {
            Some(expr) => expr,
            None => return None,
        };

        if self.peek_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }
        Some(NodeType::Statement(Box::new(ReturnStatement {
            token: Token {
                token_type: TokenType::RETURN,
                literal: "return".to_string(),
            },
            return_value: Box::new(value),
        })))
    }

    fn parse_let_statement(&mut self) -> Option<NodeType> {
        if !self.expect_peek(TokenType::IDENT) {
            // println!("Not IDENT");
            return None;
        }

        let name = Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        };

        if !self.expect_peek(TokenType::ASSIGN) {
            // println!("Not Assign");
            return None;
        }

        self.next_token();

        let value = match self.parse_expression() {
            Some(expr) => expr,
            None => return None,
        };

        if self.peek_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        Some(NodeType::Statement(Box::new(LetStatement {
            token: Token {
                token_type: TokenType::LET,
                literal: "let".to_string(),
            },
            name: Box::new(name),
            value: Box::new(value),
        })))
    }

    // 需要添加表达式解析的方法
    fn parse_expression(&mut self) -> Option<NodeType> {
        // 暂时只处理最简单的标识符表达式
        match self.cur_token.token_type {
            TokenType::IDENT => Some(NodeType::Expression(Box::new(Identifier {
                token: self.cur_token.clone(),
                value: self.cur_token.literal.clone(),
            }))),
            TokenType::INT => Some(NodeType::Expression(Box::new(Identifier {
                token: self.cur_token.clone(),
                value: self.cur_token.literal.clone(),
            }))),
            // 后续会添加更多表达式类型的处理
            _ => None,
        }
    }

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
            self.peek_error(t);
            false
        }
    }

    fn peek_error(&mut self, t: TokenType) {
        let msg = format!(
            "expected next token to be {:?},got {:?} instead",
            t, self.peek_token.token_type
        );
        self.errors.push(msg);
    }
}
