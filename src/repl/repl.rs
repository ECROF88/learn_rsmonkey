use std::io::{self, Write};

use crate::{
    ast::Node,
    evaluator::{self, evaluator::eval},
    lexer::lexer::Lexer,
    parser::parser::Parser,
    token::token::TokenType,
};

const PROMPT: &str = ">> ";
const MONKEY_FACE: &str = r#"            __,__
   .--.  .-"     "-.  .--.
  / .. \/  .-. .-.  \/ .. \
 | |  '|  /   Y   \  |'  | |
 | \   \  \ 0 | 0 /  /   / |
  \ '- ,\.-"""""""-./, -' /
   ''-' /_   ^ ^   _\ '-''
       |  \._   _./  |
       \   \ '~' /   /
        '._ '-=-' _.'
           '-----'
"#;
pub fn start() {
    let stdin = io::stdin();

    loop {
        print!("{}", PROMPT);
        io::stdout().flush().expect("fail");

        let mut input = String::new();
        match stdin.read_line(&mut input) {
            Ok(n) if n > 0 => {
                let l = Lexer::new(input.trim().to_string());
                let mut p = Parser::new(l);
                let program = p.parse_program();

                if !p.errors().is_empty() {
                    print_parser_errors(&p.errors());
                    continue;
                }

                // 评估程序
                let evaluated = eval(program.as_ref());

                // 打印评估结果
                if evaluated.type_obj() != "NULL" {
                    // 假设NULL_OBJ是"NULL"
                    println!("{}", evaluated.inspect());
                }

                // println!("{:?}", program.to_string());
            }
            _ => break,
        }
    }
}

fn print_parser_errors(errors: &[String]) {
    eprintln!("{}", MONKEY_FACE);
    eprintln!("parsing ERROR!");
    eprintln!("errors:");
    for msg in errors {
        eprintln!("\t{}", msg);
    }
}
