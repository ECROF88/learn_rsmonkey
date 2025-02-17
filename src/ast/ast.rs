use std::any::Any;

use crate::token::token::Token;

// 定义Node trait作为AST节点的基本特征
pub trait Node: std::fmt::Debug + Any {
    fn token_literal(&self) -> String;
    fn as_any(&self) -> &dyn Any;
}

// Statement trait代表语句节点
// 任何实现了Statement的必须也实现Node Tarit
pub trait Statement: Node + std::fmt::Debug {
    fn statement_node(&self);
}

// Expression trait代表表达式节点
pub trait Expression: Node + std::fmt::Debug {
    fn expression_node(&self);
}
#[derive(Debug)]
pub enum NodeType {
    Statement(Box<dyn Statement>),
    Expression(Box<dyn Expression>),
}

impl Node for NodeType {
    fn token_literal(&self) -> String {
        match self {
            NodeType::Statement(stmt) => stmt.token_literal(),
            NodeType::Expression(expr) => expr.token_literal(),
        }
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<NodeType>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            statements: Vec::new(),
        }
    }
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if !self.statements.is_empty() {
            self.statements[0].token_literal()
        } else {
            String::new()
        }
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

// LetStatement结构体，对应let语句
#[derive(Debug)]
pub struct LetStatement {
    pub token: Token, // token.LET词法单元
    pub name: Box<Identifier>,
    pub value: Box<dyn Expression>,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
}

// Identifier结构体，表示标识符
#[derive(Debug, Clone)]
pub struct Identifier {
    pub token: Token, // token.IDENT词法单元
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {}
}
