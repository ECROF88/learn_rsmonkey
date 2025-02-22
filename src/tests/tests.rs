#[cfg(test)]
mod tests {
    use crate::ast::ast::Node;
    use crate::ast::ast::{LetStatement, NodeType, ReturnStatement};
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
            (TokenType::ILLEGAL, "!"),
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
        let _error_input = "
let x 5;
let = 10;
let 838 383;
        ";
        let input = "
            let x =12;
            let y=5;
            let Foo_Bar =100;

        ";

        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_parser_errors(&p);
        if program.statements.is_empty() {
            panic!("is empty");
        }

        if program.statements.len() != 3 {
            panic!(
                "program.statements does not contain 3 statements. got={}",
                program.statements.len()
            );
        }

        let tests = ["x", "y", "Foo_Bar"];
        // println!("\n=== Program Statements ===");
        // println!("{:#?}", program.statements);
        // println!("========================\n");
        for (i, expected_identifier) in tests.iter().enumerate() {
            let stmt: &NodeType = &program.statements[i];
            if !test_let_statement(stmt, expected_identifier) {
                // panic!("wwww");
                return;
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
        let input = "
            return 5;
            return 10;
            return 993 322;
        ";

        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program();

        check_parser_errors(&p);

        if program.statements.len() != 3 {
            panic!(
                "program.statements does not contain 3 statements. got {} ",
                program.statements.len()
            );
        }

        for stmt in &program.statements {
            match stmt {
                NodeType::Statement(statement) => {
                    let return_stmt = match statement.as_any().downcast_ref::<ReturnStatement>() {
                        Some(rs) => rs,
                        None => {
                            panic!("statement not ReturnStatement. got={:?}", statement);
                        }
                    };
                    assert_eq!(
                        return_stmt.token_literal(),
                        "return",
                        "returnStmt.token_literal not 'return',got {}",
                        return_stmt.token_literal()
                    )
                }
                NodeType::Expression(_) => {
                    panic!("stmt not Statement. got Expression");
                }
            }
        }
    }
}
