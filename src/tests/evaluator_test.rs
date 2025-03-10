use crate::{
    evaluator::evaluator::eval,
    lexer::lexer::Lexer,
    object::{
        Object,
        boolean::Boolean,
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
