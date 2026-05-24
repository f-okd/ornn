use std::{env, io};

use crate::{
    ast::printer::print_expression,
    lexer::Lexer,
    parser::Parser,
    token::{Token, TokenType},
};

mod ast;
mod interpreter;
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

        println!("Scanning:");
        lexer.scan_tokens();

        if lexer.errors.len() > 0 {
            for err in lexer.errors.iter() {
                self.scan_error(err.line, err.message.as_str());
            }
            return;
        }

        println!("Parsing:");
        let mut parser = Parser::new(lexer.tokens);
        let expr = parser.parse();

        match expr {
            Ok(expr) => {
                let expr_as_string = print_expression(&expr);
                println!("{}", expr_as_string)
            }
            Err(parse_err) => {
                self.parse_error(parse_err.token, parse_err.message.as_str());
            }
        }

        // Print scanned tokens
        // println!("Command: {}", command);
        // for tkn in lexer.tokens {
        //     print!("[Token: {}, type: {:?}], ", tkn.lexeme, tkn.token_type);
        // }
        // println!("Finished printing tokens");
    }

    fn scan_error(&mut self, line: i32, message: &str) {
        self.report(line, "", message);
    }

    fn parse_error(&mut self, token: Token, message: &str) {
        if token.token_type == TokenType::EOF {
            self.report(token.line, "at end", message);
        } else {
            self.report(
                token.line,
                format!("at '{}'", token.lexeme).as_str(),
                message,
            );
        }
    }

    fn report(&mut self, line: i32, location: &str, message: &str) {
        if location == "" {
            eprintln!("[Line {}] Error: {}", line, message);
        } else {
            eprintln!("[Line {}] Error {}: {}", line, location, message);
        }
        self.had_error = true;
        return;
    }
}

fn main() {
    let mut interpreter = Ornn::new();
    interpreter.start();
}
