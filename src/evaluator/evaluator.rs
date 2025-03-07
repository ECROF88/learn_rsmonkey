use crate::ast::{
    BlockStatement, Boolean, ExpressionStatement, IntegerLiteral, Node, NodeType, Program,
};
use crate::object::integer::Integer;
use crate::object::null::Null;
use crate::object::{self, Object, boolean};

// pub fn eval(node: &dyn Node) -> Box<dyn Object> {
//     // 先尝试转换为 Program
//     if let Some(program) = node.as_any().downcast_ref::<Program>() {
//         return eval_program(program);
//     }

//     // 如果不是 Program，再尝试转换为 NodeType
//     if let Some(node_type) = node.as_any().downcast_ref::<NodeType>() {
//         return eval_node_type(node_type);
//     }

//     // 尝试其他具体类型
//     if let Some(expr_stmt) = node.as_any().downcast_ref::<ExpressionStatement>() {
//         return eval(expr_stmt.expression.as_ref());
//     }

//     if let Some(int_lit) = node.as_any().downcast_ref::<IntegerLiteral>() {
//         return Box::new(Integer::new(int_lit.value));
//     }

//     // 默认返回 Null
//     Box::new(Null {})
// }

// fn eval_node_type(node_type: &NodeType) -> Box<dyn Object> {
//     match node_type {
//         NodeType::Statement(stmt) => {
//             // 语句类型的处理
//             if let Some(expr_stmt) = stmt.as_any().downcast_ref::<ExpressionStatement>() {
//                 return eval(expr_stmt.expression.clone());
//             }

//             // 可以添加其他语句类型的处理...

//             Box::new(Null {})
//         }
//         NodeType::Expression(expr) => {
//             // 表达式类型的处理
//             if let Some(int_lit) = expr.as_any().downcast_ref::<IntegerLiteral>() {
//                 return Box::new(Integer::new(int_lit.value));
//             }

//             // 可以添加其他表达式类型的处理...

//             Box::new(Null {})
//         }
//     }
// }

// fn eval_program(program: &Program) -> Box<dyn Object> {
//     let mut result: Box<dyn Object> = Box::new(Null {});

//     for statement in &program.statements {
//         result = eval_node_type(statement);
//     }

//     result
// }

// fn eval_block_statement(block: &BlockStatement) -> Box<dyn Object> {
//     let mut result: Box<dyn Object> = Box::new(Null {});

//     for statement in &block.statements {
//         result = eval_node_type(statement);
//     }

//     result
// }
use std::sync::OnceLock;

// 使用OnceLock来创建单例布尔对象
static TRUE: OnceLock<object::Boolean> = OnceLock::new();
static FALSE: OnceLock<object::Boolean> = OnceLock::new();
static NULL: OnceLock<object::null::Null> = OnceLock::new();

fn native_bool_to_boolean_object(input: bool) -> Box<dyn Object> {
    if input {
        let boolean = TRUE.get_or_init(|| object::Boolean::new(true));
        Box::new(boolean.clone())
    } else {
        let boolean = FALSE.get_or_init(|| object::Boolean::new(false));
        Box::new(boolean.clone())
    }
}

fn get_null_object() -> Box<dyn Object> {
    Box::new(NULL.get_or_init(|| object::null::Null {}).clone())
}

pub fn eval(node: &dyn Node) -> Box<dyn Object> {
    // 处理Program
    if let Some(program) = node.as_any().downcast_ref::<Program>() {
        return eval_statements(&program.statements);
    }

    // 处理NodeType
    if let Some(node_type) = node.as_any().downcast_ref::<NodeType>() {
        match node_type {
            // 处理语句
            NodeType::Statement(stmt) => {
                // 处理表达式语句
                if let Some(expr_stmt) = stmt.as_any().downcast_ref::<ExpressionStatement>() {
                    return eval(expr_stmt.expression.as_ref());
                }
                // 其他语句类型...
            }
            // 处理表达式
            NodeType::Expression(expr) => {
                // 处理整数字面量
                if let Some(int_lit) = expr.as_any().downcast_ref::<IntegerLiteral>() {
                    println!("INTEGER!!!!!!!!!!!!!!!!!!!");
                    return Box::new(Integer::new(int_lit.value));
                }
                // Boolean
                if let Some(bool_expr) = expr.as_any().downcast_ref::<Boolean>() {
                    println!("Boolean!!!!!!!!!!!!!!!!!!!");
                    return native_bool_to_boolean_object(bool_expr.value);
                }
            }
        }
    }

    // 如果没有匹配的类型，返回Null
    // Box::new(Null {})
    get_null_object()
}

// 处理语句列表
fn eval_statements(statements: &[NodeType]) -> Box<dyn Object> {
    let mut result: Box<dyn Object> = Box::new(Null {});

    for statement in statements {
        result = eval(statement);
    }

    result
}
