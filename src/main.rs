use std::{env, io};

use crate::{
    ast::printer::print_expression,
    lexer::{Lexer, scan_tokens},
    parser::Parser,
};

mod ast;
mod lexer;
mod parser;
mod token;

struct Ornn {
    had_error: bool,
}

impl Ornn {
    fn new() -> Ornn {
        Ornn { had_error: false }
    }

    fn start(&mut self) {
        let args: Vec<String> = env::args().collect();
        if args.len() > 2 {
            println!("Interpreter expects 1 single argument.\nUsage: ornn [script]");
        } else if args.len() == 2 {
            self.run_file(&args[1]);
        } else {
            self.run_prompt();
        }
    }

    fn run_file(&mut self, filename: &str) {
        println!("Interpreting file at {}...", filename);
    }

    fn run_prompt(&mut self) {
        println!("Initialising interactive prompt...");
        let mut command = String::new();

        loop {
            command.clear();
            match io::stdin().read_line(&mut command) {
                Ok(_n) => {}
                Err(error) => println!("erorr: {error}"),
            }

            if command.as_str() == "" {
                break;
            }

            self.run(command.as_str());
        }
    }

    fn run(&mut self, command: &str) {
        let mut lexer = Lexer::new(command);
        lexer = scan_tokens(lexer);
        let mut parser = Parser::new(lexer.tokens);
        let expr = parser.parse();

        match expr {
            Ok(expr) => {
                let expr_as_string = print_expression(&expr);
                println!("{}", expr_as_string)
            }
            Err(parse_err) => {
                self.error(parse_err.token.line, parse_err.message.as_str());
            }
        }

        // Print scanned tokens
        // println!("Command: {}", command);
        // for tkn in lexer.tokens {
        //     print!("[Token: {}, type: {:?}], ", tkn.lexeme, tkn.token_type);
        // }
        // println!("Finished printing tokens");
    }

    fn error(&mut self, line: i32, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: i32, location: &str, message: &str) {
        eprintln!("[Line {}] Error {}: {}", line, location, message);
        self.had_error = true;
        return;
    }
}

fn main() {
    let mut interpreter = Ornn::new();
    interpreter.start();
}
