use core::fmt;
use std::any::Any;

use crate::token::token::Token;

// 定义Node trait作为AST节点的基本特征
pub trait Node: std::fmt::Debug + Any {
    fn token_literal(&self) -> String;
    fn as_any(&self) -> &dyn Any;
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
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
    fn to_string(&self) -> String {
        match self {
            NodeType::Statement(stmt) => stmt.to_string(),
            NodeType::Expression(expr) => expr.to_string(),
        }
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
    pub fn string(&self) -> String {
        let mut out = String::new();

        for stmt in &self.statements {
            out.push_str(&stmt.to_string());
            if !out.ends_with('\n') {
                out.push('\n');
            }
        }
        // 删除最后一个换行符
        if out.ends_with('\n') {
            out.pop();
        }
        out
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
    pub value: Box<NodeType>,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.token_literal());
        out.push_str(" ");
        out.push_str(&self.name.value);
        out.push_str(" = ");
        match &*self.value {
            NodeType::Expression(e) => out.push_str(&e.to_string()),
            NodeType::Statement(s) => out.push_str(&s.to_string()),
        }
        out.push_str(";");
        out
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
    fn to_string(&self) -> String {
        self.value.clone()
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {}
}

/// return <表达式>;
#[derive(Debug)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Box<NodeType>, // NodeType::Expression
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.token_literal());
        out.push_str(" ");
        match &*self.return_value {
            NodeType::Expression(e) => out.push_str(&e.to_string()),
            NodeType::Statement(s) => out.push_str(&s.to_string()),
        }
        out.push_str(";");
        out
    }
}

impl Statement for ReturnStatement {
    fn statement_node(&self) {}
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Box<NodeType>, // NodeType::Statement
}

impl Node for ExpressionStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }
    fn to_string(&self) -> String {
        match &*self.expression {
            NodeType::Expression(e) => e.to_string(),
            NodeType::Statement(s) => s.to_string(),
        }
    }
}

impl Statement for ExpressionStatement {
    fn statement_node(&self) {}
}

#[derive(Debug)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_string(&self) -> String {
        self.token.literal.to_string() // not sure
    }
}

impl Expression for IntegerLiteral {
    fn expression_node(&self) {}
}

impl fmt::Display for IntegerLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token.literal)
    }
}

#[derive(Debug)]
pub struct PrefixExpression {
    pub token: Token, // 前缀词法单元，如!
    pub operator: String,
    pub right: Box<NodeType>,
}
impl Node for PrefixExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }
    fn to_string(&self) -> String {
        let mut out = String::new();
        out.push_str("(");
        out.push_str(&self.operator);
        match &*self.right {
            NodeType::Expression(e) => out.push_str(&e.to_string()),
            NodeType::Statement(s) => out.push_str(&s.to_string()),
        }
        out.push_str(")");
        out
    }
}

impl Expression for PrefixExpression {
    fn expression_node(&self) {}
}

#[derive(Debug)]
pub struct InfixExpression {
    pub token: Token,
    pub left: Box<NodeType>,
    pub operator: String,
    pub right: Box<NodeType>,
}

impl Node for InfixExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }
    fn to_string(&self) -> String {
        let mut out = String::new();
        out.push_str("(");
        match &*self.left {
            NodeType::Expression(e) => out.push_str(&e.to_string()),
            NodeType::Statement(s) => out.push_str(&s.to_string()),
        };
        out.push_str(" ");
        out.push_str(&self.operator);
        out.push_str(" ");
        match &*self.right {
            NodeType::Expression(e) => out.push_str(&e.to_string()),
            NodeType::Statement(s) => out.push_str(&s.to_string()),
        };
        out.push_str(")");

        out
    }
}
impl Expression for InfixExpression {
    fn expression_node(&self) {}
}
