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

impl NodeType {
    pub fn clone_node(&self) -> Self {
        match self {
            NodeType::Statement(stmt) => {
                if let Some(expr_stmt) = stmt.as_any().downcast_ref::<ExpressionStatement>() {
                    NodeType::Statement(Box::new(ExpressionStatement {
                        token: expr_stmt.token.clone(),
                        expression: Box::new(expr_stmt.expression.clone_node()),
                    }))
                } else if let Some(let_stmt) = stmt.as_any().downcast_ref::<LetStatement>() {
                    NodeType::Statement(Box::new(LetStatement {
                        token: let_stmt.token.clone(),
                        name: let_stmt.name.clone(),
                        value: Box::new(let_stmt.value.clone_node()),
                    }))
                } else if let Some(ret_stmt) = stmt.as_any().downcast_ref::<ReturnStatement>() {
                    NodeType::Statement(Box::new(ReturnStatement {
                        token: ret_stmt.token.clone(),
                        return_value: Box::new(ret_stmt.return_value.clone_node()),
                    }))
                } else if let Some(block_stmt) = stmt.as_any().downcast_ref::<BlockStatement>() {
                    let cloned_statements = block_stmt
                        .statements
                        .iter()
                        .map(|s| s.clone_node())
                        .collect();
                    NodeType::Statement(Box::new(BlockStatement {
                        token: block_stmt.token.clone(),
                        statements: cloned_statements,
                    }))
                } else {
                    panic!("Unknown statement type in clone_node")
                }
            }
            NodeType::Expression(expr) => {
                if let Some(int_lit) = expr.as_any().downcast_ref::<IntegerLiteral>() {
                    NodeType::Expression(Box::new(IntegerLiteral {
                        token: int_lit.token.clone(),
                        value: int_lit.value,
                    }))
                } else if let Some(bool_expr) = expr.as_any().downcast_ref::<Boolean>() {
                    NodeType::Expression(Box::new(Boolean {
                        token: bool_expr.token.clone(),
                        value: bool_expr.value,
                    }))
                } else if let Some(ident) = expr.as_any().downcast_ref::<Identifier>() {
                    NodeType::Expression(Box::new(Identifier {
                        token: ident.token.clone(),
                        value: ident.value.clone(),
                    }))
                }
                // 为其他表达式类型添加类似的匹配分支
                else {
                    panic!("Unknown expression type in clone_node")
                }
            }
        }
    }
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
    // pub fn string(&self) -> String {
    //     self.statements
    //         .iter()
    //         .map(|stmt| stmt.to_string())
    //         .collect()
    // }
}
impl AsRef<dyn Node> for Program {
    fn as_ref(&self) -> &(dyn Node + 'static) {
        self as &dyn Node
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
    fn to_string(&self) -> String {
        self.statements.iter().map(|i| i.to_string()).collect()
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
        out.push(' ');
        out.push_str(&self.name.value);
        out.push_str(" = ");
        match &*self.value {
            NodeType::Expression(e) => out.push_str(&e.to_string()),
            NodeType::Statement(s) => out.push_str(&s.to_string()),
        }
        out.push(';');
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
        out.push(' ');
        match &*self.return_value {
            NodeType::Expression(e) => out.push_str(&e.to_string()),
            NodeType::Statement(s) => out.push_str(&s.to_string()),
        }
        out.push(';');
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
        out.push('(');
        out.push_str(&self.operator);
        match &*self.right {
            NodeType::Expression(e) => out.push_str(&e.to_string()),
            NodeType::Statement(s) => out.push_str(&s.to_string()),
        }
        out.push(')');
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
        out.push('(');
        match &*self.left {
            NodeType::Expression(e) => out.push_str(&e.to_string()),
            NodeType::Statement(s) => out.push_str(&s.to_string()),
        };
        out.push(' ');
        out.push_str(&self.operator);
        out.push(' ');
        match &*self.right {
            NodeType::Expression(e) => out.push_str(&e.to_string()),
            NodeType::Statement(s) => out.push_str(&s.to_string()),
        };
        out.push(')');

        out
    }
}
impl Expression for InfixExpression {
    fn expression_node(&self) {}
}

#[derive(Debug)]
pub struct Boolean {
    pub token: Token,
    pub value: bool,
}

impl Node for Boolean {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_string(&self) -> String {
        self.token.literal.to_string()
    }
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }
}
impl Expression for Boolean {
    fn expression_node(&self) {}
}

// if 表达式
#[derive(Debug)]
pub struct IfExpression {
    pub token: Token,                       // 'if'词法单元
    pub condition: Box<NodeType>,           // 条件表达式
    pub consequence: Box<NodeType>,         // 如果条件为真时执行的语句块
    pub alternative: Option<Box<NodeType>>, // 可选的else语句块
}

impl Node for IfExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn to_string(&self) -> String {
        let mut out = String::new();

        out.push_str("if");
        out.push_str(&self.condition.to_string());
        out.push_str(" ");
        out.push_str(&self.consequence.to_string());

        // 如果有else分支，则添加
        if let Some(alt) = &self.alternative {
            out.push_str("else ");
            out.push_str(&alt.to_string());
        }

        out
    }
}

impl Expression for IfExpression {
    fn expression_node(&self) {}
}

// 定义BlockStatement结构体
#[derive(Debug)]
pub struct BlockStatement {
    pub token: Token, // { 词法单元
    pub statements: Vec<NodeType>,
}

impl Node for BlockStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn to_string(&self) -> String {
        let mut out = String::new();

        for stmt in &self.statements {
            out.push_str(&stmt.to_string());
        }

        out
    }
}

impl Statement for BlockStatement {
    fn statement_node(&self) {}
}
#[derive(Debug)]
pub struct FunctionLiteral {
    pub token: Token,
    // pub parameters: Vec<Identifier>,
    pub parameters: Vec<NodeType>,
    pub body: Box<NodeType>, // BlockStatement
}

impl Node for FunctionLiteral {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_string(&self) -> String {
        let mut out = String::new();
        // let mut params = String::new();
        // for p in &self.parameters {
        //     params.push_str(&p.to_string());
        // }
        let params: Vec<String> = self.parameters.iter().map(|p| p.to_string()).collect();

        out.push_str(&self.token_literal());
        out.push('(');
        out.push_str(&params.join(", "));
        out.push_str(") ");
        out.push_str(&self.body.to_string());

        out
    }
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }
}

impl Expression for FunctionLiteral {
    fn expression_node(&self) {}
}

#[derive(Debug)]
pub struct CallExpression {
    pub token: Token,
    pub function: Box<NodeType>,
    pub arguments: Vec<NodeType>,
}

impl Node for CallExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_string(&self) -> String {
        let mut out = String::new();

        let args: Vec<String> = self.arguments.iter().map(|a| a.to_string()).collect();

        out.push_str(&self.function.to_string());
        out.push_str("(");
        out.push_str(&args.join(", "));
        out.push_str(")");

        out
    }
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }
}

impl Expression for CallExpression {
    fn expression_node(&self) {}
}
