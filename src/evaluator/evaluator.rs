use crate::ast::{
    BlockStatement, Boolean, ExpressionStatement, Identifier, IfExpression, InfixExpression,
    IntegerLiteral, LetStatement, Node, NodeType, PrefixExpression, Program, ReturnStatement,
};
use crate::object::environment::Environment;
use crate::object::integer::Integer;
use crate::object::{self, Object, ReturnValue};

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
use std::sync::OnceLock;

// 使用OnceLock来创建单例对象
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

pub fn get_null_object() -> Box<dyn Object> {
    Box::new(NULL.get_or_init(|| object::null::Null {}).clone())
}

pub fn eval(node: &dyn Node, env: &mut Environment) -> Box<dyn Object> {
    // 处理Program
    if let Some(program) = node.as_any().downcast_ref::<Program>() {
        return eval_program(&program.statements, env);
    }

    // 处理NodeType
    if let Some(node_type) = node.as_any().downcast_ref::<NodeType>() {
        match node_type {
            // 处理语句
            NodeType::Statement(stmt) => {
                // 处理表达式语句
                if let Some(expr_stmt) = stmt.as_any().downcast_ref::<ExpressionStatement>() {
                    return eval(expr_stmt.expression.as_ref(), env);
                }
                // 其他语句类型...
                if let Some(block) = stmt.as_any().downcast_ref::<BlockStatement>() {
                    println!("eval block");
                    return eval_block_statement(&block, env);
                }
                if let Some(return_stmt) = stmt.as_any().downcast_ref::<ReturnStatement>() {
                    println!("eval return statement");
                    let val = eval(return_stmt.return_value.as_ref(), env);
                    if is_error(&val) {
                        return val;
                    }
                    return Box::new(ReturnValue::new(val));
                }
                if let Some(let_stmt) = stmt.as_any().downcast_ref::<LetStatement>() {
                    println!("eval let statement");

                    let val = eval(let_stmt.value.as_ref(), env);
                    if is_error(&val) {
                        return val;
                    }

                    // let cloned_obj = val.clone_object();
                    env.set(&let_stmt.name.value, val);
                    // env.set2(&let_stmt.name.value, val2);
                }
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
                if let Some(prefix_epxr) = expr.as_any().downcast_ref::<PrefixExpression>() {
                    println!("Prefix!!!!!!!!!!!!!!!!!!!!");
                    let right = eval(prefix_epxr.right.as_ref(), env);
                    if is_error(&right) {
                        return right;
                    }
                    return eval_prefix_expression(&prefix_epxr.operator, right);
                }
                if let Some(infix_expr) = expr.as_any().downcast_ref::<InfixExpression>() {
                    println!("Infix!!!!!!!!!!!!!!!!!!!!");
                    let left = eval(infix_expr.left.as_ref(), env);
                    if is_error(&left) {
                        return left;
                    }
                    let right = eval(infix_expr.right.as_ref(), env);
                    if is_error(&right) {
                        return right;
                    }
                    return eval_infix_expression(&infix_expr.operator, left, right);
                }
                if let Some(if_expr) = expr.as_any().downcast_ref::<IfExpression>() {
                    println!("eval if expr");
                    return eval_if_expression(if_expr, env);
                }
                if let Some(identifier) = expr.as_any().downcast_ref::<Identifier>() {
                    println!("Identifier: {}", identifier.value);
                    return eval_identifier(identifier, env);
                }
            }
        }
    }

    // 如果没有匹配的类型，返回Null
    // Box::new(Null {})
    println!("GET NULL");
    get_null_object()
}

fn eval_program(statements: &[NodeType], env: &mut Environment) -> Box<dyn Object> {
    let mut result = get_null_object();

    for statement in statements {
        result = eval(statement, env);

        match result.type_obj().as_str() {
            // 如果是返回值，解包并返回内部值
            "RETURN_VALUE" => {
                if let Some(return_value) = result.as_any().downcast_ref::<ReturnValue>() {
                    return return_value.take_value();
                }
            }
            // 如果是错误，直接返回错误对象
            "ERROR" => {
                // 错误对象无需解包，直接返回
                return result;
            }
            // 其他类型继续执行
            _ => {}
        }
    }

    result
}

fn eval_block_statement(block: &BlockStatement, env: &mut Environment) -> Box<dyn Object> {
    let mut result = get_null_object();

    for statement in &block.statements {
        result = eval(statement, env);

        // 块语句中遇到返回值，不解包而是直接返回
        if result.type_obj() == "RETURN_VALUE" || result.type_obj() == "ERROR" {
            return result;
        }
    }

    result
}

fn eval_identifier(node: &Identifier, env: &mut Environment) -> Box<dyn Object> {
    if let Some(val) = env.get(&node.value) {
        if let Some(int) = val.as_any().downcast_ref::<Integer>() {
            println!("{} 的值是整数: {}", node.value, int.value);
            Box::new(Integer::new(int.value))
        } else if let Some(bool_val) = val.as_any().downcast_ref::<object::Boolean>() {
            println!("{} 的值是布尔值: {}", node.value, bool_val.value);
            native_bool_to_boolean_object(bool_val.value)
        } else {
            val.clone_object() // 通用clone方法，但是无法得到具体类型
        }
    } else {
        new_error(format!("identifier not found: {}", node.value))
    }
}

fn eval_prefix_expression(operator: &str, right: Box<dyn Object>) -> Box<dyn Object> {
    match operator {
        "!" => eval_bang_operator_expression(right),
        "-" => eval_minus_prefix_operator_expression(right),
        // _ => get_null_object(),
        _ => {
            let error_msg = format!("unknown operator: {}{}", operator, right.type_obj());
            new_error(error_msg)
        }
    }
}

fn eval_bang_operator_expression(right: Box<dyn Object>) -> Box<dyn Object> {
    // 处理!运算符的逻辑
    match right.type_obj().as_str() {
        "BOOLEAN" => {
            if let Some(bool_obj) = right.as_any().downcast_ref::<object::Boolean>() {
                return native_bool_to_boolean_object(!bool_obj.value);
            }
            get_null_object()
        }
        "NULL" => native_bool_to_boolean_object(true), // NULL取反为true
        _ => native_bool_to_boolean_object(false),     // 其他类型取反为false
    }
}

fn eval_minus_prefix_operator_expression(right: Box<dyn Object>) -> Box<dyn Object> {
    // 处理-运算符的逻辑
    if right.type_obj() != "INTEGER" {
        return new_error(format!("unknown operator: -{}", right.type_obj()));
    }

    if let Some(int_obj) = right.as_any().downcast_ref::<Integer>() {
        return Box::new(Integer::new(-int_obj.value));
    }

    get_null_object()
}

fn eval_infix_expression(
    operator: &str,
    left: Box<dyn Object>,
    right: Box<dyn Object>,
) -> Box<dyn Object> {
    if left.type_obj() == "INTEGER" && right.type_obj() == "INTEGER" {
        return eval_integer_infix_expression(operator, left, right);
    } else if left.type_obj() == "BOOLEAN" && right.type_obj() == "BOOLEAN" {
        return eval_boolean_infix_expression(operator, left, right);
    } else if left.type_obj() != right.type_obj() {
        let error_msg = format!(
            "type mismatch: {} {} {}",
            left.type_obj(),
            operator,
            right.type_obj()
        );
        return new_error(error_msg);
    } else {
        let error_msg = format!(
            "unknown operator: {} {} {}",
            left.type_obj(),
            operator,
            right.type_obj()
        );
        return new_error(error_msg);
    }
}

fn eval_boolean_infix_expression(
    operator: &str,
    left: Box<dyn Object>,
    right: Box<dyn Object>,
) -> Box<dyn Object> {
    let left_val = left
        .as_any()
        .downcast_ref::<object::Boolean>()
        .expect("Left operand is not a Boolean")
        .value;
    let right_val = right
        .as_any()
        .downcast_ref::<object::Boolean>()
        .expect("Right operand is not a Boolean")
        .value;

    match operator {
        "==" => native_bool_to_boolean_object(left_val == right_val),
        "!=" => native_bool_to_boolean_object(left_val != right_val),
        _ => new_error(format!(
            "unknown operator: {} {} {}",
            "BOOLEAN", operator, "BOOLEAN"
        )),
    }
}

fn eval_integer_infix_expression(
    operator: &str,
    left: Box<dyn Object>,
    right: Box<dyn Object>,
) -> Box<dyn Object> {
    let left_val = left
        .as_any()
        .downcast_ref::<object::Integer>()
        .expect("左操作数不是整数");

    let right_val = right
        .as_any()
        .downcast_ref::<object::Integer>()
        .expect("右操作数不是整数");

    match operator {
        "+" => Box::new(Integer::new(left_val.value + right_val.value)),
        "-" => Box::new(Integer::new(left_val.value - right_val.value)),
        "*" => Box::new(Integer::new(left_val.value * right_val.value)),
        "/" => Box::new(Integer::new(left_val.value / right_val.value)),
        "<" => native_bool_to_boolean_object(left_val.value < right_val.value),
        ">" => native_bool_to_boolean_object(left_val.value > right_val.value),
        "==" => native_bool_to_boolean_object(left_val.value == right_val.value),
        "!=" => native_bool_to_boolean_object(left_val.value != right_val.value),
        // _ => get_null_object(),
        _ => new_error(format!(
            "unknown operator:{} {} {}",
            left.type_obj(),
            operator,
            right.type_obj()
        )),
    }
}

fn eval_if_expression(ie: &IfExpression, env: &mut Environment) -> Box<dyn Object> {
    let condition = eval(ie.condition.as_ref(), env);
    if is_error(&condition) {
        return condition;
    }
    println!("got condition:{:?}", condition.inspect());

    if is_truthy(&condition) {
        println!("condition is true");
        if let Some(block) = ie.consequence.as_any().downcast_ref::<BlockStatement>() {
            return eval_block_statement(&block, env);
        }
        return eval(ie.consequence.as_ref(), env);
    } else if let Some(alt) = &ie.alternative {
        println!("condition is false");
        if let Some(block) = alt.as_any().downcast_ref::<BlockStatement>() {
            return eval_block_statement(&block, env);
        }
        return eval(alt.as_ref(), env);
    }
    println!("if expression return none");
    get_null_object()
}
fn is_truthy(obj: &Box<dyn Object>) -> bool {
    match obj.type_obj().as_str() {
        "NULL" => false,
        "BOOLEAN" => {
            // 对于布尔对象，获取其值
            if let Some(bool_obj) = obj.as_any().downcast_ref::<object::Boolean>() {
                bool_obj.value
            } else {
                false
            }
        }
        _ => {
            println!("other all return true");
            true
        }
    }
}

fn new_error(message: String) -> Box<dyn Object> {
    Box::new(object::error::Error { message })
}

fn is_error(obj: &Box<dyn Object>) -> bool {
    obj.type_obj() == "ERROR"
}
