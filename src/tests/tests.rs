#[cfg(test)]
mod tests {
    use crate::ast::{
        BlockStatement, Boolean, CallExpression, Expression, FunctionLiteral, Identifier,
        IfExpression, InfixExpression, IntegerLiteral, LetStatement, NodeType, PrefixExpression,
        ReturnStatement,
    };
    use crate::ast::{ExpressionStatement, Node};
    use crate::lexer::lexer::Lexer;
    use crate::parser::parser::Parser;
    use crate::token::token::TokenType;
    use core::panic;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";

        let tests = vec![
            (TokenType::ASSIGN, "="),
            (TokenType::PLUS, "+"),
            (TokenType::LPAREN, "("),
            (TokenType::RPAREN, ")"),
            (TokenType::LBRACE, "{"),
            (TokenType::RBRACE, "}"),
            (TokenType::COMMA, ","),
            (TokenType::SEMICOLON, ";"),
            (TokenType::EOF, ""),
        ];

        let mut l = Lexer::new(input.to_string());
        for (expected_type, expected_literal) in tests {
            let tok = l.next_token();

            assert_eq!(tok.token_type, expected_type);
            assert_eq!(tok.literal, expected_literal);
        }
    }

    #[test]
    fn test_next_token_2() {
        let input = r#"
    let five = 5;
    let ten = 10;

    let add = fn(x, y) {
      x + y;
    };

    let result = add(five, ten);
    !-/*5;
    5 < 10 > 5;

    if (5 < 10) {
    return true;
    } else {
    return false;
    }

    10 == 10;
    10 != 9;
    "foobar"
    "foo bar"
    [1, 2];
    {"foo": "bar"}
    "#;

        let tests = vec![
            (TokenType::LET, "let"),
            (TokenType::IDENT, "five"),
            (TokenType::ASSIGN, "="),
            (TokenType::INT, "5"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::LET, "let"),
            (TokenType::IDENT, "ten"),
            (TokenType::ASSIGN, "="),
            (TokenType::INT, "10"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::LET, "let"),
            (TokenType::IDENT, "add"),
            (TokenType::ASSIGN, "="),
            (TokenType::FN, "fn"),
            (TokenType::LPAREN, "("),
            (TokenType::IDENT, "x"),
            (TokenType::COMMA, ","),
            (TokenType::IDENT, "y"),
            (TokenType::RPAREN, ")"),
            (TokenType::LBRACE, "{"),
            (TokenType::IDENT, "x"),
            (TokenType::PLUS, "+"),
            (TokenType::IDENT, "y"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::RBRACE, "}"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::LET, "let"),
            (TokenType::IDENT, "result"),
            (TokenType::ASSIGN, "="),
            (TokenType::IDENT, "add"),
            (TokenType::LPAREN, "("),
            (TokenType::IDENT, "five"),
            (TokenType::COMMA, ","),
            (TokenType::IDENT, "ten"),
            (TokenType::RPAREN, ")"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::BANG, "!"),
            (TokenType::MINUS, "-"),
            (TokenType::SLASH, "/"),
            (TokenType::ASTERISK, "*"),
            (TokenType::INT, "5"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::INT, "5"),
            (TokenType::LT, "<"),
            (TokenType::INT, "10"),
            (TokenType::GT, ">"),
            (TokenType::INT, "5"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::IF, "if"),
            (TokenType::LPAREN, "("),
            (TokenType::INT, "5"),
            (TokenType::LT, "<"),
            (TokenType::INT, "10"),
            (TokenType::RPAREN, ")"),
            (TokenType::LBRACE, "{"),
            (TokenType::RETURN, "return"),
            (TokenType::TRUE, "true"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::RBRACE, "}"),
            (TokenType::ELSE, "else"),
            (TokenType::LBRACE, "{"),
            (TokenType::RETURN, "return"),
            (TokenType::FALSE, "false"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::RBRACE, "}"),
            (TokenType::INT, "10"),
            (TokenType::EQ, "=="),
            (TokenType::INT, "10"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::INT, "10"),
            (TokenType::NOTEQ, "!="),
            (TokenType::INT, "9"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::STRING, "foobar"),
            (TokenType::STRING, "foo bar"),
            (TokenType::LBRACKET, "["),
            (TokenType::INT, "1"),
            (TokenType::COMMA, ","),
            (TokenType::INT, "2"),
            (TokenType::RBRACKET, "]"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::LBRACE, "{"),
            (TokenType::STRING, "foo"),
            (TokenType::COLON, ":"),
            (TokenType::STRING, "bar"),
            (TokenType::RBRACE, "}"),
            (TokenType::EOF, ""),
        ];

        let mut l = Lexer::new(input.to_string());

        for (expected_type, expected_literal) in tests {
            let tok = l.next_token();

            assert_eq!(tok.token_type, expected_type);
            assert_eq!(tok.literal, expected_literal);
        }
    }

    #[test]
    fn test_let_statements() {
        struct Test {
            input: String,
            expected_identifier: String,
            expected_value: ExpectedValue<'static>,
        }

        let tests = vec![
            Test {
                input: "let x = 5;".to_string(),
                expected_identifier: "x".to_string(),
                expected_value: ExpectedValue::Integer(5),
            },
            Test {
                input: "let y = true;".to_string(),
                expected_identifier: "y".to_string(),
                expected_value: ExpectedValue::Boolean(true),
            },
            Test {
                input: "let foobar = y;".to_string(),
                expected_identifier: "foobar".to_string(),
                expected_value: ExpectedValue::String("y"),
            },
        ];

        for tt in tests {
            let l = Lexer::new(tt.input);
            let mut p = Parser::new(l);
            let program = p.parse_program();
            check_parser_errors(&p);

            if program.statements.len() != 1 {
                panic!(
                    "program.statements does not contain 1 statements. got={}",
                    program.statements.len()
                );
            }

            let stmt = &program.statements[0];
            if !test_let_statement(stmt, &tt.expected_identifier) {
                return;
            }

            match stmt {
                NodeType::Statement(statement) => {
                    let let_stmt = statement
                        .as_any()
                        .downcast_ref::<LetStatement>()
                        .expect("statement not LetStatement");

                    if !test_literal_expression(&let_stmt.value, tt.expected_value) {
                        panic!("Value test failed");
                    }
                }
                NodeType::Expression(_) => panic!("stmt is not Statement"),
            }
        }
    }

    fn test_let_statement(stmt: &NodeType, name: &str) -> bool {
        if stmt.token_literal() != "let" {
            println!(
                "stmt.token_literal not 'let'. got={:?}",
                stmt.token_literal()
            );
            return false;
        }

        // 尝试将 NodeType::Statement 转换为 LetStatement
        match stmt {
            NodeType::Statement(statement) => {
                // 使用as_any()来进行向下转型
                let let_stmt = match statement.as_any().downcast_ref::<LetStatement>() {
                    Some(ls) => ls,
                    None => {
                        println!("statement not LetStatement. got={:?}", statement);
                        return false;
                    }
                };

                if &let_stmt.name.value != name {
                    println!(
                        "let_stmt.name.value not '{}'. got={}",
                        name, let_stmt.name.value
                    );
                    return false;
                }

                if let_stmt.name.token_literal().as_str() != name {
                    println!(
                        "let_stmt.name.token_literal() not '{}'. got={}",
                        name,
                        let_stmt.name.token_literal()
                    );
                    return false;
                }

                true
            }
            NodeType::Expression(_) => {
                println!("stmt not Statement. got=Expression");
                false
            }
        }
    }

    fn check_parser_errors(p: &Parser) {
        let errors = p.errors();
        if errors.is_empty() {
            return;
        }
        eprintln!("parser has {} errors", errors.len());
        for err in errors {
            eprintln!("parser error: {}", err);
        }
    }

    #[test]
    fn test_return_statements() {
        struct Test {
            input: String,
            expected_value: ExpectedValue<'static>,
        }
        let tests = vec![
            Test {
                input: "return 5;".to_string(),
                expected_value: ExpectedValue::Integer(5),
            },
            Test {
                input: "return true;".to_string(),
                expected_value: ExpectedValue::Boolean(true),
            },
            Test {
                input: "return y;".to_string(),
                expected_value: ExpectedValue::String("y"),
            },
        ];
        for tt in tests {
            let l = Lexer::new(tt.input);
            let mut p = Parser::new(l);
            let program = p.parse_program();

            check_parser_errors(&p);

            if program.statements.len() != 1 {
                panic!(
                    "program.statements does not contain 3 statements. got {} ",
                    program.statements.len()
                );
            }

            match &program.statements[0] {
                NodeType::Statement(stmt) => {
                    let return_stmt = stmt
                        .as_any()
                        .downcast_ref::<ReturnStatement>()
                        .expect("not RetrunStatement!");

                    assert_eq!(
                        return_stmt.token_literal(),
                        "return",
                        "returnStmt.token_literal not 'return', got {}",
                        return_stmt.token_literal()
                    );
                    if !test_literal_expression(&return_stmt.return_value, tt.expected_value) {
                        panic!("return_stmt.return_value not matching expected value");
                    }
                }
                _ => panic!("stmt not Statement. got Expression"),
            }
        }

        // for stmt in &program.statements {
        //     match stmt {
        //         NodeType::Statement(statement) => {
        //             let return_stmt = match statement.as_any().downcast_ref::<ReturnStatement>() {
        //                 Some(rs) => rs,
        //                 None => {
        //                     panic!("statement not ReturnStatement. got={:?}", statement);
        //                 }
        //             };
        //             assert_eq!(
        //                 return_stmt.token_literal(),
        //                 "return",
        //                 "returnStmt.token_literal not 'return',got {}",
        //                 return_stmt.token_literal()
        //             );
        //         }
        //         NodeType::Expression(_) => {
        //             panic!("stmt not Statement. got Expression");
        //         }
        //     }
        // }
    }

    #[test]
    fn test_to_string() {
        use crate::ast::{
            ExpressionStatement, Identifier, LetStatement, NodeType, Program, ReturnStatement,
        };
        use crate::token::token::{Token, TokenType};

        // 测试简单标识符
        let ident = Identifier {
            token: Token {
                token_type: TokenType::IDENT,
                literal: "testVar".to_string(),
            },
            value: "testVar".to_string(),
        };
        assert_eq!(ident.to_string(), "testVar", "Identifier to_string failed");

        // 测试let语句
        let let_stmt = LetStatement {
            token: Token {
                token_type: TokenType::LET,
                literal: "let".to_string(),
            },
            name: Box::new(Identifier {
                token: Token {
                    token_type: TokenType::IDENT,
                    literal: "x".to_string(),
                },
                value: "x".to_string(),
            }),
            value: Box::new(NodeType::Expression(Box::new(Identifier {
                token: Token {
                    token_type: TokenType::IDENT,
                    literal: "y".to_string(),
                },
                value: "y".to_string(),
            }))),
        };
        assert_eq!(
            let_stmt.to_string(),
            "let x = y;",
            "LetStatement to_string failed"
        );

        // 测试return语句
        let return_stmt = ReturnStatement {
            token: Token {
                token_type: TokenType::RETURN,
                literal: "return".to_string(),
            },
            return_value: Box::new(NodeType::Expression(Box::new(Identifier {
                token: Token {
                    token_type: TokenType::IDENT,
                    literal: "result".to_string(),
                },
                value: "result".to_string(),
            }))),
        };
        assert_eq!(
            return_stmt.to_string(),
            "return result;",
            "ReturnStatement to_string failed"
        );

        // 测试表达式语句
        let expr_stmt = ExpressionStatement {
            token: Token {
                token_type: TokenType::IDENT,
                literal: "x".to_string(),
            },
            expression: Box::new(NodeType::Expression(Box::new(Identifier {
                token: Token {
                    token_type: TokenType::IDENT,
                    literal: "x".to_string(),
                },
                value: "x".to_string(),
            }))),
        };
        assert_eq!(
            expr_stmt.to_string(),
            "x",
            "ExpressionStatement to_string failed"
        );

        // 测试完整程序
        let program = Program {
            statements: vec![
                NodeType::Statement(Box::new(let_stmt)),
                NodeType::Statement(Box::new(return_stmt)),
                NodeType::Statement(Box::new(expr_stmt)),
            ],
        };

        let expected = "let x = y;return result;x";
        assert_eq!(
            program.to_string(),
            expected,
            "Program to_string wrong. got={}",
            program.to_string()
        );
    }

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;";

        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program();

        check_parser_errors(&p);

        if program.statements.len() != 1 {
            panic!(
                "program has not enough statements. got={}",
                program.statements.len()
            );
        }

        match &program.statements[0] {
            NodeType::Statement(stmt) => {
                let expr_stmt = stmt
                    .as_any()
                    .downcast_ref::<ExpressionStatement>()
                    .expect("stmt not ExpressionStatement");

                let expr = &*expr_stmt.expression;
                if let NodeType::Expression(e) = expr {
                    let ident = e
                        .as_any()
                        .downcast_ref::<Identifier>()
                        .expect("not Identifier");
                    assert_eq!(ident.value, "foobar");
                    assert_eq!(ident.token_literal(), "foobar");
                }
            }
            NodeType::Expression(_) => panic!("program.statements[0] not Statement"),
        }
    }

    #[test]
    fn test_boolean_expression() {
        let input = "true";
        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program();

        check_parser_errors(&p);
        if program.statements.len() != 1 {
            panic!(
                "program has not enough statements. got={}",
                program.statements.len()
            );
        }

        match &program.statements[0] {
            NodeType::Statement(stmt) => {
                let expr_stmt = stmt
                    .as_any()
                    .downcast_ref::<ExpressionStatement>()
                    .expect("stmt not ExpressionStatement");

                let expr = &*expr_stmt.expression;
                if let NodeType::Expression(e) = expr {
                    let ident = e.as_any().downcast_ref::<Boolean>().expect("not Boolean");
                    assert_eq!(ident.value, true);
                    assert_eq!(ident.token_literal(), "true");
                }
            }
            NodeType::Expression(_) => panic!("program.statements[0] not Statement"),
        }
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = "5;";
        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_parser_errors(&p);

        if program.statements.len() != 1 {
            panic!(
                "program has not enough statement.got {}",
                program.statements.len()
            );
        }
        match &program.statements[0] {
            NodeType::Statement(stmt) => {
                let expr_stmt = stmt
                    .as_any()
                    .downcast_ref::<ExpressionStatement>()
                    .expect("stmt not ExpressionStatement");
                if let NodeType::Expression(expr) = &*expr_stmt.expression {
                    let literal = expr
                        .as_any()
                        .downcast_ref::<IntegerLiteral>()
                        .expect("expr not IntegerLiteral");

                    assert_eq!(
                        literal.value, 5,
                        "literal.value not {}. got={}",
                        5, literal.value
                    );
                    assert_eq!(
                        literal.token_literal(),
                        "5",
                        "literal.token_literal not {}. got={}",
                        "5",
                        literal.token_literal()
                    );
                }
            }
            NodeType::Expression(_) => {
                panic!("program.Statements[0] is not ast.ExpressionStatement.")
            }
        }
    }

    #[test]
    fn test_parsing_prefix_expressions() {
        #[derive(Debug)]
        enum TestValue {
            Int(i64),
            Boolean(bool),
        }
        struct PrefixTest {
            input: &'static str,
            operator: &'static str,
            // integer_value: i64,
            value: TestValue,
        }

        let prefix_tests = vec![
            PrefixTest {
                input: "!5;",
                operator: "!",
                value: TestValue::Int(5),
            },
            PrefixTest {
                input: "-15;",
                operator: "-",
                value: TestValue::Int(15),
            },
            PrefixTest {
                input: "!true",
                operator: "!",
                value: TestValue::Boolean(true),
            },
            PrefixTest {
                input: "!false",
                operator: "!",
                value: TestValue::Boolean(false),
            },
        ];

        for tt in prefix_tests {
            let l = Lexer::new(tt.input.to_string());
            let mut p = Parser::new(l);
            let program = p.parse_program();
            check_parser_errors(&p);

            if program.statements.len() != 1 {
                panic!(
                    "program.statements does not contain {} statements. got={}\n",
                    1,
                    program.statements.len()
                );
            }

            match &program.statements[0] {
                NodeType::Statement(stmt) => {
                    let expr_stmt = stmt
                        .as_any()
                        .downcast_ref::<ExpressionStatement>()
                        .expect("program.statements[0] is not ExpressionStatement");

                    let prefix_expr = match &*expr_stmt.expression {
                        NodeType::Expression(expr) => expr
                            .as_any()
                            .downcast_ref::<PrefixExpression>()
                            .expect("expression is not PrefixExpression"),
                        _ => panic!("not an Expression type"),
                    };

                    assert_eq!(
                        prefix_expr.operator, tt.operator,
                        "exp.operator is not '{}'. got={}",
                        tt.operator, prefix_expr.operator
                    );

                    match tt.value {
                        TestValue::Boolean(bo) => test_boolean_literal(&prefix_expr.right, bo),
                        TestValue::Int(value) => test_integer_literal(&prefix_expr.right, value),
                    };
                }
                NodeType::Expression(_) => panic!("program.statements[0] is not Statement"),
            }
        }
    }
    fn test_integer_literal(node: &NodeType, value: i64) -> bool {
        if let NodeType::Expression(expr) = node {
            let literal = match expr.as_any().downcast_ref::<IntegerLiteral>() {
                Some(il) => il,
                None => {
                    panic!("expr is not IntegerLiteral. got={:?}", expr);
                    return false;
                }
            };

            if literal.value != value {
                panic!("literal.value not {}. got={}", value, literal.value);
                return false;
            }

            if literal.token_literal() != value.to_string() {
                panic!(
                    "literal.token_literal not {}. got={}",
                    value,
                    literal.token_literal()
                );
                return false;
            }

            true
        } else {
            panic!("node is not Expression");
            false
        }
    }

    // 中缀表达式
    #[test]
    fn test_parsing_infix_expressions() {
        #[derive(Debug)]
        enum TestValue {
            Int(i64),
            Boolean(bool),
        }
        struct InfixTest {
            input: String,
            left_value: TestValue,
            operator: String,
            right_value: TestValue,
        }
        let infix_tests = vec![
            InfixTest {
                input: "5 + 5 ;".to_string(),
                left_value: TestValue::Int(5),
                operator: "+".to_string(),
                right_value: TestValue::Int(5),
            },
            InfixTest {
                input: "5 - 5;".to_string(),
                left_value: TestValue::Int(5),
                operator: "-".to_string(),
                right_value: TestValue::Int(5),
            },
            InfixTest {
                input: "5 * 5;".to_string(),
                left_value: TestValue::Int(5),
                operator: "*".to_string(),
                right_value: TestValue::Int(5),
            },
            InfixTest {
                input: "5 / 5;".to_string(),
                left_value: TestValue::Int(5),
                operator: "/".to_string(),
                right_value: TestValue::Int(5),
            },
            InfixTest {
                input: "5 > 5;".to_string(),
                left_value: TestValue::Int(5),
                operator: ">".to_string(),
                right_value: TestValue::Int(5),
            },
            InfixTest {
                input: "5 < 5;".to_string(),
                left_value: TestValue::Int(5),
                operator: "<".to_string(),
                right_value: TestValue::Int(5),
            },
            InfixTest {
                input: "5 == 5;".to_string(),
                left_value: TestValue::Int(5),
                operator: "==".to_string(),
                right_value: TestValue::Int(5),
            },
            InfixTest {
                input: "5 != 5;".to_string(),
                left_value: TestValue::Int(5),
                operator: "!=".to_string(),
                right_value: TestValue::Int(5),
            },
            // 布尔测试用例
            InfixTest {
                input: "true == true".to_string(),
                left_value: TestValue::Boolean(true),
                operator: "==".to_string(),
                right_value: TestValue::Boolean(true),
            },
            InfixTest {
                input: "true != false".to_string(),
                left_value: TestValue::Boolean(true),
                operator: "!=".to_string(),
                right_value: TestValue::Boolean(false),
            },
            InfixTest {
                input: "false == false".to_string(),
                left_value: TestValue::Boolean(false),
                operator: "==".to_string(),
                right_value: TestValue::Boolean(false),
            },
        ];

        for tt in infix_tests {
            let l = Lexer::new(tt.input.to_string());
            let mut p = Parser::new(l);
            let program = p.parse_program();
            check_parser_errors(&p);

            if program.statements.len() != 1 {
                panic!(
                    "program.Statements does not contain 1 statements. got={}",
                    program.statements.len()
                );
            }

            // match &program.statements[0] {
            //     // NodeType::Statement(stmt) => {
            //     //     let expr_stmt = stmt
            //     //         .as_any()
            //     //         .downcast_ref::<ExpressionStatement>()
            //     //         .expect("not expression stetement");
            //     //     let expr = match &*expr_stmt.expression {
            //     //         NodeType::Expression(expr) => expr
            //     //             .as_any()
            //     //             .downcast_ref::<InfixExpression>()
            //     //             .expect("not InfixExpression"),
            //     //         _ => panic!("not an Expression type"),
            //     //     };
            //     //     test_integer_literal(&expr.left, tt.left_value);
            //     //     assert_eq!(expr.operator, tt.operator);
            //     //     test_integer_literal(&expr.right, tt.right_value);
            //     }
            //     NodeType::Expression(_) => panic!("program.statements[0] is not Statement"),

            match &program.statements[0] {
                NodeType::Statement(stmt) => {
                    let expr_stmt = stmt
                        .as_any()
                        .downcast_ref::<ExpressionStatement>()
                        .expect("not expression statement");

                    match &tt.left_value {
                        TestValue::Int(left_int) => match &tt.right_value {
                            TestValue::Int(right_int) => {
                                test_infix_expression(
                                    &expr_stmt.expression,
                                    ExpectedValue::Integer(*left_int),
                                    &tt.operator,
                                    ExpectedValue::Integer(*right_int),
                                );
                            }
                            // TestValue::Boolean(right_bool) => {
                            //     test_infix_expression(
                            //         &expr_stmt.expression,
                            //         ExpectedValue::Integer(*left_int),
                            //         &tt.operator,
                            //         ExpectedValue::Boolean(*right_bool),
                            //     );
                            // }
                            TestValue::Boolean(_) => panic!("left is i64,but right is bool"),
                        },
                        TestValue::Boolean(left_bool) => match &tt.right_value {
                            // TestValue::Int(right_int) => {
                            //     test_infix_expression(
                            //         &expr_stmt.expression,
                            //         ExpectedValue::Boolean(*left_bool),
                            //         &tt.operator,
                            //         ExpectedValue::Integer(*right_int),
                            //     );
                            // }
                            TestValue::Boolean(right_bool) => {
                                test_infix_expression(
                                    &expr_stmt.expression,
                                    ExpectedValue::Boolean(*left_bool),
                                    &tt.operator,
                                    ExpectedValue::Boolean(*right_bool),
                                );
                            }
                            TestValue::Int(_) => panic!("left is bool,but right is i64"),
                        },
                    }
                }
                NodeType::Expression(_) => panic!("program.statements[0] is not Statement"),
            }
        }
    }

    #[test]
    fn test_operator_precedence_parsing() {
        struct OperatorPrecedenceTest {
            input: String,
            expected: String,
        }

        let tests = vec![
            OperatorPrecedenceTest {
                input: "-a * b".to_string(),
                expected: "((-a) * b)".to_string(),
            },
            OperatorPrecedenceTest {
                input: "!-a".to_string(),
                expected: "(!(-a))".to_string(),
            },
            OperatorPrecedenceTest {
                input: "a + b + c".to_string(),
                expected: "((a + b) + c)".to_string(),
            },
            OperatorPrecedenceTest {
                input: "a + b - c".to_string(),
                expected: "((a + b) - c)".to_string(),
            },
            OperatorPrecedenceTest {
                input: "a * b * c".to_string(),
                expected: "((a * b) * c)".to_string(),
            },
            OperatorPrecedenceTest {
                input: "a * b / c".to_string(),
                expected: "((a * b) / c)".to_string(),
            },
            OperatorPrecedenceTest {
                input: "a + b / c".to_string(),
                expected: "(a + (b / c))".to_string(),
            },
            OperatorPrecedenceTest {
                input: "a + b * c + d / e - f".to_string(),
                expected: "(((a + (b * c)) + (d / e)) - f)".to_string(),
            },
            OperatorPrecedenceTest {
                input: "3 + 4; -5 * 5".to_string(),
                expected: "(3 + 4)((-5) * 5)".to_string(),
            },
            OperatorPrecedenceTest {
                input: "5 > 4 == 3 < 4".to_string(),
                expected: "((5 > 4) == (3 < 4))".to_string(),
            },
            OperatorPrecedenceTest {
                input: "5 < 4 != 3 > 4".to_string(),
                expected: "((5 < 4) != (3 > 4))".to_string(),
            },
            OperatorPrecedenceTest {
                input: "3 + 4 * 5 == 3 * 1 + 4 * 5".to_string(),
                expected: "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))".to_string(),
            },
            // boolean test
            OperatorPrecedenceTest {
                input: "true".to_string(),
                expected: "true".to_string(),
            },
            OperatorPrecedenceTest {
                input: "false".to_string(),
                expected: "false".to_string(),
            },
            OperatorPrecedenceTest {
                input: "3 > 5 == false".to_string(),
                expected: "((3 > 5) == false)".to_string(),
            },
            OperatorPrecedenceTest {
                input: "3 < 5 == true".to_string(),
                expected: "((3 < 5) == true)".to_string(),
            },
            // 添加带括号的表达式
            OperatorPrecedenceTest {
                input: "1 + (2 + 3) + 4".to_string(),
                expected: "((1 + (2 + 3)) + 4)".to_string(),
            },
            OperatorPrecedenceTest {
                input: "(5 + 5) * 2".to_string(),
                expected: "((5 + 5) * 2)".to_string(),
            },
            OperatorPrecedenceTest {
                input: "2 / (5 + 5)".to_string(),
                expected: "(2 / (5 + 5))".to_string(),
            },
            OperatorPrecedenceTest {
                input: "-(5 + 5)".to_string(),
                expected: "(-(5 + 5))".to_string(),
            },
            OperatorPrecedenceTest {
                input: "!(true == true)".to_string(),
                expected: "(!(true == true))".to_string(),
            },
            OperatorPrecedenceTest {
                input: "a + add(b * c) +d".to_string(),
                expected: "((a + add((b * c))) + d)".to_string(),
            },
            OperatorPrecedenceTest {
                input: "add(a + b + c * d / f + g)".to_string(),
                expected: "add((((a + b) + ((c * d) / f)) + g))".to_string(),
            },
        ];

        for tt in tests {
            let l = Lexer::new(tt.input);
            let mut p = Parser::new(l);
            let program = p.parse_program();
            check_parser_errors(&p);

            let actual = program.to_string();
            assert_eq!(
                actual, tt.expected,
                "expected={}, got={}",
                tt.expected, actual
            );
        }
    }

    fn test_identifier(node: &NodeType, value: &str) -> bool {
        if let NodeType::Expression(expr) = node {
            let ident = match expr.as_any().downcast_ref::<Identifier>() {
                Some(id) => id,
                None => {
                    panic!("expr is not Identifier. got={:?}", expr);
                    return false;
                }
            };

            if ident.value != value {
                panic!("ident.value not {}. got={}", value, ident.value);
                return false;
            }

            if ident.token_literal() != value {
                panic!(
                    "ident.token_literal not {}. got={}",
                    value,
                    ident.token_literal()
                );
                return false;
            }

            true
        } else {
            panic!("node is not Expression");
            false
        }
    }
    enum ExpectedValue<'a> {
        Integer(i64),
        String(&'a str),
        Boolean(bool),
    }
    fn test_literal_expression(exp: &NodeType, expected: ExpectedValue) -> bool {
        match expected {
            ExpectedValue::Integer(value) => test_integer_literal(exp, value),
            ExpectedValue::String(value) => test_identifier(exp, value),
            ExpectedValue::Boolean(value) => test_boolean_literal(exp, value),
        }
    }

    fn test_boolean_literal(exp: &NodeType, value: bool) -> bool {
        if let NodeType::Expression(expr) = exp {
            let bo = match expr.as_any().downcast_ref::<Boolean>() {
                Some(bo) => bo,
                None => {
                    panic!("exp is not Boolean.got {:?}", expr);
                    return false;
                }
            };

            if bo.value != value {
                panic!("bo.value not {},got {}", value, bo.value);
                return false;
            }

            let expected_literal = if value { "true" } else { "false" };
            if bo.token_literal() != expected_literal {
                panic!(
                    "bo.token_literal not {}. got={}",
                    expected_literal,
                    bo.token_literal()
                );
                return false;
            }
            return true;
        } else {
            panic!("node is not Expression");
            return false;
        }
    }

    fn test_infix_expression(
        exp: &NodeType,
        left: ExpectedValue,
        operator: &str,
        right: ExpectedValue,
    ) -> bool {
        if let NodeType::Expression(expr) = exp {
            // let op_exp = match expr.as_any().downcast_ref::<InfixExpression>() {
            //     Some(op) => op,
            //     None => {
            //         eprintln!("exp is not InfixExpression. got={:?}", expr);
            //         return false;
            //     }
            // };
            if let Some(op) = expr.as_any().downcast_ref::<InfixExpression>() {
                let op_exp = op;

                if !test_literal_expression(&op_exp.left, left) {
                    return false;
                }

                // 检查操作符
                if op_exp.operator != operator {
                    panic!(
                        "exp.operator is not '{}'. got='{}'",
                        operator, op_exp.operator
                    );
                    return false;
                }

                // 测试右操作数
                if !test_literal_expression(&op_exp.right, right) {
                    return false;
                }

                true
            } else {
                panic!("exp is not InfixExpression. got={:?}", expr);
                false
            }
        } else {
            panic!("exp is not Expression");
            false
        }
    }

    #[test]
    fn test_if_expression() {
        let input = "if (x>y){x}";

        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_parser_errors(&p);

        if program.statements.len() != 1 {
            panic!(
                "program.Statements does not contain 1 statements.got {}",
                program.statements.len()
            );
        }

        match &program.statements[0] {
            NodeType::Statement(stmt) => {
                if let Some(expr_stmt) = stmt.as_any().downcast_ref::<ExpressionStatement>() {
                    let node = &*expr_stmt.expression;
                    let if_expr = match node {
                        NodeType::Expression(expr) => expr
                            .as_any()
                            .downcast_ref::<IfExpression>()
                            .expect("not IfExpressino"),
                        NodeType::Statement(_) => panic!("not ExpressionStatement"),
                    };

                    test_infix_expression(
                        &if_expr.condition,
                        ExpectedValue::String("x"),
                        ">",
                        ExpectedValue::String("y"),
                    );
                    let block_stmt = match &*if_expr.consequence {
                        NodeType::Statement(stmt) => stmt
                            .as_any()
                            .downcast_ref::<BlockStatement>()
                            .expect("not BlockStatement"),
                        _ => panic!("if_expr.consequence is not a Statement"),
                    };
                    if block_stmt.statements.len() != 1 {
                        panic!(
                            "consequence is not 1 statements. got={}",
                            block_stmt.statements.len()
                        );
                    }

                    match &block_stmt.statements[0] {
                        NodeType::Statement(stmt) => {
                            if let Some(consequence) =
                                stmt.as_any().downcast_ref::<ExpressionStatement>()
                            {
                                test_identifier(&consequence.expression, "x");
                            } else {
                                panic!("Consequence.Statements[0] is not ExpressionStatement");
                            }
                        }
                        _ => panic!("Not a Statement type"),
                    }

                    // 这个测试用例没有 else 语句
                    assert!(
                        if_expr.alternative.is_none(),
                        "if_expr.alternative was not None. got={:?}",
                        if_expr.alternative
                    );
                } else {
                    panic!("not ExpressionStatement");
                }
            }
            NodeType::Expression(_) => panic!("program.statements[0] is not Statement"),
        }
    }

    #[test]
    fn test_if_else_expression() {
        let input = "if (x>y) {x}else{y}";
        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_parser_errors(&p);
        if program.statements.len() != 1 {
            panic!(
                "program.Statements does not contain 1 statements.got {}",
                program.statements.len()
            );
        }

        match &program.statements[0] {
            NodeType::Statement(stmt) => {
                if let Some(expr_stmt) = stmt.as_any().downcast_ref::<ExpressionStatement>() {
                    let node = &*expr_stmt.expression;
                    let if_expr = match node {
                        NodeType::Expression(expr) => expr
                            .as_any()
                            .downcast_ref::<IfExpression>()
                            .expect("not IfExpressino"),
                        NodeType::Statement(_) => panic!("not ExpressionStatement"),
                    };

                    test_infix_expression(
                        &if_expr.condition,
                        ExpectedValue::String("x"),
                        ">",
                        ExpectedValue::String("y"),
                    );
                    let block_stmt = match &*if_expr.consequence {
                        NodeType::Statement(stmt) => stmt
                            .as_any()
                            .downcast_ref::<BlockStatement>()
                            .expect("not BlockStatement"),
                        _ => panic!("if_expr.consequence is not a Statement"),
                    };
                    if block_stmt.statements.len() != 1 {
                        panic!(
                            "consequence is not 1 statements. got={}",
                            block_stmt.statements.len()
                        );
                    }

                    match &block_stmt.statements[0] {
                        NodeType::Statement(stmt) => {
                            if let Some(consequence) =
                                stmt.as_any().downcast_ref::<ExpressionStatement>()
                            {
                                test_identifier(&consequence.expression, "x");
                            } else {
                                panic!("Consequence.Statements[0] is not ExpressionStatement");
                            }
                        }
                        _ => panic!("Not a Statement type"),
                    }

                    if let Some(alt) = &if_expr.alternative {
                        // let a = alt;
                        let block_stmt = match &**alt {
                            NodeType::Statement(stmt) => stmt
                                .as_any()
                                .downcast_ref::<BlockStatement>()
                                .expect("not BlockStatement"),
                            _ => panic!("if_expr.alternative is not a Statement"),
                        };

                        // 检查 else 块中的语句
                        if block_stmt.statements.len() != 1 {
                            panic!(
                                "alternative is not 1 statements. got={}",
                                block_stmt.statements.len()
                            );
                        }

                        // 检查 else 块中的表达式
                        match &block_stmt.statements[0] {
                            NodeType::Statement(stmt) => {
                                if let Some(alternative) =
                                    stmt.as_any().downcast_ref::<ExpressionStatement>()
                                {
                                    test_identifier(&alternative.expression, "y");
                                } else {
                                    panic!("Alternative.Statements[0] is not ExpressionStatement");
                                }
                            }
                            _ => panic!("Not a Statement type"),
                        }
                    } else {
                        panic!("if_expr.alternative was None");
                    }
                } else {
                    panic!("not ExpressionStatement");
                }
            }
            NodeType::Expression(_) => panic!("program.statements[0] is not Statement"),
        }
    }

    #[test]
    fn test_function_literal_parsing() {
        let input = "fn(x,y){x+y;}";

        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_parser_errors(&p);

        if program.statements.len() != 1 {
            panic!(
                "program.Statements does not contain 1 statements.got {}",
                program.statements.len()
            )
        }
        match &program.statements[0] {
            NodeType::Statement(stmt) => {
                let expr_stmt = stmt
                    .as_any()
                    .downcast_ref::<ExpressionStatement>()
                    .expect("not ExpressionStatement!");

                let function = match &*expr_stmt.expression {
                    NodeType::Expression(expr) => expr
                        .as_any()
                        .downcast_ref::<FunctionLiteral>()
                        .expect("not function literal"),
                    _ => panic!("expr_stmt.expression is not an Expression"),
                };

                if function.parameters.len() != 2 {
                    panic!(
                        "function literal params wrong.want 2,got {}",
                        function.parameters.len()
                    );
                }

                test_literal_expression(&function.parameters[0], ExpectedValue::String("x"));
                test_literal_expression(&function.parameters[1], ExpectedValue::String("y"));

                let body = match &*function.body {
                    NodeType::Statement(stmt) => stmt
                        .as_any()
                        .downcast_ref::<BlockStatement>()
                        .expect("function body stmt is not BlockStatement"),
                    _ => {
                        panic!("function.body is not a Statement type")
                    }
                };
                if body.statements.len() != 1 {
                    panic!(
                        "function.Body.Statements has not 1 statements. got={}",
                        body.statements.len()
                    );
                }

                match &body.statements[0] {
                    NodeType::Statement(body_stmt) => {
                        let body_expr_stmt = body_stmt
                            .as_any()
                            .downcast_ref::<ExpressionStatement>()
                            .expect("function body stmt is not ExpressionStatement");
                        test_infix_expression(
                            &body_expr_stmt.expression,
                            ExpectedValue::String("x"),
                            "+",
                            ExpectedValue::String("y"),
                        );
                    }
                    _ => panic!("function.Body.Statements[0] is not Statement"),
                }
            }

            NodeType::Expression(_) => panic!("program.statements[0] is not Statement"),
        }
    }

    #[test]
    fn test_function_parameter_parsing() {
        struct Test {
            input: String,
            expected_params: Vec<String>,
        }
        let tests = vec![
            Test {
                input: "fn() {};".to_string(),
                expected_params: vec![],
            },
            Test {
                input: "fn(x) {};".to_string(),
                expected_params: vec!["x".to_string()],
            },
            Test {
                input: "fn(x, y, z) {};".to_string(),
                expected_params: vec!["x".to_string(), "y".to_string(), "z".to_string()],
            },
        ];

        for tt in tests {
            let l = Lexer::new(tt.input);
            let mut p = Parser::new(l);
            let program = p.parse_program();
            check_parser_errors(&p);

            if program.statements.len() != 1 {
                panic!(
                    "program.statements should contain 1 statement, got {}",
                    program.statements.len()
                );
            }

            match &program.statements[0] {
                NodeType::Statement(stmt) => {
                    let expr_stmt = stmt
                        .as_any()
                        .downcast_ref::<ExpressionStatement>()
                        .expect("not ExpressionStatement!");

                    let function = match &*expr_stmt.expression {
                        NodeType::Expression(expr) => expr
                            .as_any()
                            .downcast_ref::<FunctionLiteral>()
                            .expect("not function literal"),
                        _ => panic!("not an Expression type"),
                    };
                    assert_eq!(
                        function.parameters.len(),
                        tt.expected_params.len(),
                        "length parameters wrong. want {}, got={}",
                        tt.expected_params.len(),
                        function.parameters.len()
                    );

                    for (index, ident) in tt.expected_params.iter().enumerate() {
                        test_literal_expression(
                            &function.parameters[index],
                            ExpectedValue::String(ident),
                        );
                    }
                }
                _ => panic!("program.statements[0] is not Statement"),
            }
        }
    }

    #[test]
    fn test_call_expression_parsing() {
        let input = "add(1, 2 * 3, 4 + 5);";

        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_parser_errors(&p);

        if program.statements.len() != 1 {
            panic!(
                "program.statements should contain 1 statement, got {}",
                program.statements.len()
            );
        }

        match &program.statements[0] {
            NodeType::Statement(stmt) => {
                let expr_stmt = stmt
                    .as_any()
                    .downcast_ref::<ExpressionStatement>()
                    .expect("stmt is not expressionStatement");

                let exp = match &*expr_stmt.expression {
                    NodeType::Expression(expr) => expr
                        .as_any()
                        .downcast_ref::<CallExpression>()
                        .expect(&format!(
                            "not CallExpression. got {:?}",
                            expr_stmt.expression
                        )),
                    _ => panic!("expr_stmt.expression is not an Expression"),
                };

                test_identifier(&exp.function, "add");

                if exp.arguments.len() != 3 {
                    panic!("wrong length of arguments. got {}", exp.arguments.len());
                }

                test_literal_expression(&exp.arguments[0], ExpectedValue::Integer(1));
                test_infix_expression(
                    &exp.arguments[1],
                    ExpectedValue::Integer(2),
                    "*",
                    ExpectedValue::Integer(3),
                );
                test_infix_expression(
                    &exp.arguments[2],
                    ExpectedValue::Integer(4),
                    "+",
                    ExpectedValue::Integer(5),
                );
            }
            _ => panic!("is not Statement"),
        }
    }
}
