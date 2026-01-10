use std::io::{self, Write};

use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::scanner::Scanner;

pub struct Lox {}
impl Lox {
    pub fn new() -> Self {
        Lox {}
    }

    pub fn run(&self, text: &str) {
        let mut scanner = Scanner::new(text.to_string());
        let tokens = scanner.scan_tokens();
        scanner.print_info();
        let mut parser = Parser::new(tokens);

        let expressions = parser.parse();
        println!("expression: {:#?}", expressions);
        for expression in expressions {
            let expr_val = match expression {
                Ok(expr) => Interpreter::eval_expression(&expr),
                Err(e) => Err(e),
            };
            println!("###### expression value: {:?}", expr_val.unwrap());
        }
    }
    pub fn run_repl(&self) -> io::Result<()> {
        println!("Starting REPL...");
        loop {
            print!("> ");
            io::stdout().flush()?;
            let mut input_text = String::new();
            io::stdin().read_line(&mut input_text)?;
            if input_text.trim() == "exit" {
                break;
            }
            self.run(&input_text);
        }
        Ok(())
    }
}
