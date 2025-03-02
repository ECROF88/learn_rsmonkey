use crate::ast::ast::{
    ExpressionStatement, Identifier, InfixExpression, IntegerLiteral, LetStatement, NodeType,
    PrefixExpression, Program, ReturnStatement,
};
use crate::lexer::lexer::Lexer;
use crate::token::token::{Token, TokenType};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Precedence {
    LOWEST,
    EQUALS,      // ==
    LESSGREATER, // > or <
    SUM,         // +
    PRODUCT,     // *
    PREFIX,      // -X or !X
    CALL,        // myFunction(X)
}

// 前缀解析函数：不需要其他参数，直接返回一个表达式
type PrefixParseFn = fn(&mut Parser) -> Option<NodeType>;

// 中缀解析函数：接收左侧表达式作为参数，返回一个新表达式
type InfixParseFn = fn(&mut Parser, NodeType) -> Option<NodeType>;
pub struct Parser {
    l: Lexer,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<String>,
    // 前缀解析函数映射表
    prefix_parse_fns: HashMap<TokenType, PrefixParseFn>,
    // 中缀解析函数映射表
    infix_parse_fns: HashMap<TokenType, InfixParseFn>,
}

impl Parser {
    pub fn new(l: Lexer) -> Self {
        let mut p = Parser {
            l,
            cur_token: Token {
                token_type: TokenType::ILLEGAL,
                literal: String::new(),
            },
            peek_token: Token {
                token_type: TokenType::ILLEGAL,
                literal: String::new(),
            },
            errors: Vec::new(),
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };

        // 读取两个词法单元，以设置cur_token和peek_token
        p.next_token();
        p.next_token();

        p.register_prefix(TokenType::IDENT, Parser::parse_identifier);
        p.register_prefix(TokenType::INT, Parser::parse_integer_literal);
        p.register_prefix(TokenType::BANG, Parser::parse_prefix_expression); // 对应 !
        p.register_prefix(TokenType::MINUS, Parser::parse_prefix_expression); // 对应 -

        // 注册中缀解析函数
        p.register_infix(TokenType::PLUS, Parser::parse_infix_expression);
        p.register_infix(TokenType::MINUS, Parser::parse_infix_expression);
        p.register_infix(TokenType::SLASH, Parser::parse_infix_expression);
        p.register_infix(TokenType::ASTERISK, Parser::parse_infix_expression);
        p.register_infix(TokenType::EQ, Parser::parse_infix_expression);
        p.register_infix(TokenType::NOTEQ, Parser::parse_infix_expression);
        p.register_infix(TokenType::LT, Parser::parse_infix_expression);
        p.register_infix(TokenType::GT, Parser::parse_infix_expression);

        p
    }

    // 将当前词法单元和字面量提供给Identifier的token和value字段
    // 该方法不会调用next_token()方法
    fn parse_identifier(&mut self) -> Option<NodeType> {
        Some(NodeType::Expression(Box::new(Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        })))
    }

    fn token_precedence(&self, token_type: TokenType) -> Precedence {
        match token_type {
            TokenType::EQ | TokenType::NOTEQ => Precedence::EQUALS,
            TokenType::LT | TokenType::GT => Precedence::LESSGREATER,
            TokenType::PLUS | TokenType::MINUS => Precedence::SUM,
            TokenType::SLASH | TokenType::ASTERISK => Precedence::PRODUCT,
            TokenType::LPAREN => Precedence::CALL,
            _ => Precedence::LOWEST,
        }
    }

    fn cur_precedence(&self) -> Precedence {
        self.token_precedence(self.cur_token.token_type)
    }

    fn peek_precedence(&self) -> Precedence {
        self.token_precedence(self.peek_token.token_type)
    }

    // 注册前缀解析函数
    fn register_prefix(&mut self, token_type: TokenType, f: PrefixParseFn) {
        self.prefix_parse_fns.insert(token_type, f);
    }

    // 注册中缀解析函数
    fn register_infix(&mut self, token_type: TokenType, f: InfixParseFn) {
        self.infix_parse_fns.insert(token_type, f);
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    pub fn errors(&self) -> Vec<String> {
        self.errors.clone()
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::new();

        // 循环直到遇到 EOF token
        while !self.cur_token_is(TokenType::EOF) {
            if let Some(stmt) = self.parse_statement() {
                println!("Parsed statement: {:?}", stmt);
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
            TokenType::EOF | TokenType::ILLEGAL if self.cur_token.literal.trim().is_empty() => {
                self.next_token();
                None
            }
            _ => {
                // println!("parse_statement get None !");
                self.parse_expression_statement()
            }
        }
    }

    fn parse_return_statement(&mut self) -> Option<NodeType> {
        let token = self.cur_token.clone();

        // 跳过 return 关键字
        self.next_token();

        let value = self.parse_expression(Precedence::LOWEST)?;

        // 如果有分号，跳过它
        if self.peek_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        Some(NodeType::Statement(Box::new(ReturnStatement {
            token,
            return_value: Box::new(value),
        })))
    }

    fn parse_let_statement(&mut self) -> Option<NodeType> {
        if !self.expect_peek(TokenType::IDENT) {
            return None;
        }

        let name = Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        };

        if !self.expect_peek(TokenType::ASSIGN) {
            return None;
        }

        self.next_token();

        let value = self.parse_expression(Precedence::LOWEST)?;

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

    fn no_prefix_parse_fn_error(&mut self, token: TokenType) {
        let msg = format!("no prefix parse function for {:?} found", token);
        self.errors.push(msg);
    }

    // 需要添加表达式解析的方法
    fn parse_expression(&mut self, precedence: Precedence) -> Option<NodeType> {
        // 查找当前token对应的前缀解析函数
        if let Some(&prefix) = self.prefix_parse_fns.get(&self.cur_token.token_type) {
            let mut left_exp = prefix(self)?;

            while !self.peek_token_is(TokenType::SEMICOLON) && precedence < self.peek_precedence() {
                if let Some(&infix) = self.infix_parse_fns.get(&self.peek_token.token_type) {
                    self.next_token();
                    left_exp = infix(self, left_exp)?;
                } else {
                    return Some(left_exp);
                }
            }

            Some(left_exp)
        } else {
            // println!("simple imple");
            // // 暂时保持现有的简单实现
            // match self.cur_token.token_type {
            //     TokenType::IDENT => Some(NodeType::Expression(Box::new(Identifier {
            //         token: self.cur_token.clone(),
            //         value: self.cur_token.literal.clone(),
            //     }))),
            //     TokenType::INT => Some(NodeType::Expression(Box::new(Identifier {
            //         token: self.cur_token.clone(),
            //         value: self.cur_token.literal.clone(),
            //     }))),
            //     _ => None,
            // }
            self.no_prefix_parse_fn_error(self.cur_token.token_type);
            None
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
            "expected next token to be {:?}, got {:?} instead",
            t, self.peek_token.token_type
        );
        self.errors.push(msg);
    }

    fn parse_expression_statement(&mut self) -> Option<NodeType> {
        let token = self.cur_token.clone();

        if let Some(expression) = self.parse_expression(Precedence::LOWEST) {
            if self.peek_token_is(TokenType::SEMICOLON) {
                self.next_token();
            }

            let stmt = ExpressionStatement {
                token,
                expression: Box::new(expression),
            };

            Some(NodeType::Statement(Box::new(stmt))) // why is Statement？
        } else {
            None
        }
    }

    fn parse_integer_literal(&mut self) -> Option<NodeType> {
        let token = self.cur_token.clone();

        match token.literal.parse::<i64>() {
            Ok(value) => Some(NodeType::Expression(Box::new(IntegerLiteral {
                token,
                value,
            }))),
            Err(_) => {
                let msg = format!("could not parse {} as integer", self.cur_token.literal);
                self.errors.push(msg);
                None
            }
        }
    }

    fn parse_prefix_expression(&mut self) -> Option<NodeType> {
        let token = self.cur_token.clone();
        let operator = self.cur_token.literal.clone();

        self.next_token();

        let right = self.parse_expression(Precedence::PREFIX)?;

        Some(NodeType::Expression(Box::new(PrefixExpression {
            token,
            operator,
            right: Box::new(right),
        })))
    }

    fn parse_infix_expression(&mut self, left: NodeType) -> Option<NodeType> {
        let token = self.cur_token.clone();
        let operator = self.cur_token.literal.clone();

        let precedence = self.cur_precedence();
        self.next_token();
        let right = self.parse_expression(precedence)?;

        Some(NodeType::Expression(Box::new(InfixExpression {
            token,
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })))
    }
}
