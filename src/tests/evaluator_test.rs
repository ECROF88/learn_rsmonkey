use crate::{
    evaluator::evaluator::eval,
    lexer::lexer::Lexer,
    object::{
        Object,
        boolean::Boolean,
        error::Error,
        integer::Integer,
        object::{BOOLEAN_OBJ, INTEGER_OBJ},
    },
    parser::parser::Parser,
};

#[test]
fn test_eval_integer_expression() {
    let tests = vec![
        ("5", 5_i64),
        ("10", 10_i64),
        ("-5", -5),
        ("5+5+5-10", 5),
        ("20+2* -10", 0),
        ("2*(5+10)", 30),
        ("(5+10*2+15/3)*2+ -10", 50),
        ("2*2*2*2", 16),
        ("3*(3*3)+1", 28),
    ];
    for (input, expected) in tests {
        let evaluated = test_eval(input);
        test_integer_object(&evaluated, expected);
    }
}

fn test_eval(input: &str) -> Box<dyn Object> {
    let l = Lexer::new(input.to_string());
    let mut p = Parser::new(l);
    let program = p.parse_program();
    // println!("AST: {:#?}", program); // 打印AST
    eval(program.as_ref())
}

fn test_integer_object(obj: &Box<dyn Object>, expected: i64) {
    if obj.type_obj() != INTEGER_OBJ {
        panic!("对象不是整数。得到={:?}", obj.inspect());
    }

    // 由于Rust的类型系统，我们需要确保对象是Integer类型
    // 并且安全地访问其值
    let int_obj = match obj.as_any().downcast_ref::<Integer>() {
        Some(i) => i,
        None => panic!("对象不能转换为Integer类型"),
    };

    assert_eq!(
        int_obj.value, expected,
        "对象的值错误。得到={}，期望={}",
        int_obj.value, expected
    );
}

#[test]
fn test_eval_boolean_expression() {
    let tests = vec![
        ("true", true),
        ("false", false),
        ("1>1", false),
        ("1!=1", false),
        ("1!=2", true),
        ("1<2", true),
    ];

    for (input, expected) in tests {
        let evaluated = test_eval(input);
        test_boolean_object(&evaluated, expected);
    }
}

fn test_boolean_object(obj: &Box<dyn Object>, expected: bool) {
    if obj.type_obj() != BOOLEAN_OBJ {
        panic!("object is not Boolean. got{}", obj.inspect());
    }
    // let bool_obj = match obj.as_any().downcast_ref::<Boolean>() {
    //     Some(b) => b,
    //     None => panic!(""),
    // };

    if let Some(b) = obj.as_any().downcast_ref::<Boolean>() {
        assert_eq!(
            b.value, expected,
            "对象的值错误。得到={}，期望={}",
            b.value, expected
        );
    } else {
        panic!("对象不能转换为Boolean类型");
    }
}

#[test]
fn test_bang_operator() {
    let tests = vec![
        ("!true", false),
        ("!false", true),
        ("!5", false),
        ("!!true", true),
        ("!!false", false),
    ];

    for (input, expected) in tests {
        let evaluated = test_eval(input);
        test_boolean_object(&evaluated, expected);
    }
}

#[test]
fn test_eval_if_else_expression() {
    let tests = vec![
        ("if (true) { 10 }", Some(10)),
        ("if (false) { 10 }", None), // 新增：条件为false，无else分支
        ("if (1) { 10 }", Some(10)),
        ("if (1 < 2) { 10 }", Some(10)),
        ("if (1 > 2) { 10 }", None), // 新增：条件为false，无else分支
        ("if (1 > 2) { 10 } else { 20 }", Some(20)),
        ("if (1 < 2) { 10 } else { 20 }", Some(10)),
    ];

    for (input, expected) in tests {
        let evaluated = test_eval(input);
        eprintln!("evaluated = {:?}", evaluated.inspect());
        match expected {
            Some(value) => {
                // if let Some(_) = evaluated.as_any().downcast_ref::<Integer>() {
                //     test_integer_object(&evaluated, value);
                // } else {
                //     panic!("not integer,get {}", evaluated.inspect())
                // }
                test_integer_object(&evaluated, value);
            }
            None => {
                // test_null_object(&evaluated);
                assert_eq!(
                    evaluated.type_obj(),
                    "NULL",
                    "对象不是NULL。得到={:?}",
                    evaluated.inspect()
                );
            }
        }
    }
}

#[test]
fn test_return_statements() {
    let tests = vec![
        ("return 10;", 10),
        ("return 10; 9", 10),
        ("return 2 * 5; 9;", 10),
        ("9; return 2 * 5; 9;", 10),
    ];

    for (input, expected) in tests {
        let evaluated = test_eval(input);
        test_integer_object(&evaluated, expected);
    }
}

#[test]
fn test_error_handling() {
    let tests = vec![
        ("5+true", "type mismatch: INTEGER + BOOLEAN"),
        ("5 + true; 5;", "type mismatch: INTEGER + BOOLEAN"),
        ("-true", "unknown operator: -BOOLEAN"),
        ("true + false;", "unknown operator: BOOLEAN + BOOLEAN"),
        ("5; true + false; 5", "unknown operator: BOOLEAN + BOOLEAN"),
        (
            "if (10 > 1) { true + false; }",
            "unknown operator: BOOLEAN + BOOLEAN",
        ),
        (
            "if (10 > 1) {
              if (10 > 1) {
                return true + false;
              }
              return 1;
            } ",
            "unknown operator: BOOLEAN + BOOLEAN",
        ),
    ];

    for (input, expected_msg) in tests {
        let evaluated = test_eval(input);

        if evaluated.type_obj() != "ERROR" {
            panic!(
                "没有返回错误对象。得到={}({})",
                evaluated.type_obj(),
                evaluated.inspect()
            );
        }
        let error_obj = match evaluated.as_any().downcast_ref::<Error>() {
            Some(e) => e,
            None => panic!("对象不能转换为Error类型"),
        };
        assert_eq!(
            error_obj.message, expected_msg,
            "错误消息不匹配。期望=\"{}\", 得到=\"{}\"",
            expected_msg, error_obj.message
        );
    }
}
