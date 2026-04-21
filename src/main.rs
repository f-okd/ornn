use std::{env, io};

use crate::{
    ast::{expressions::Expr, printer::print_expression},
    lexer::{Lexer, scan_tokens},
    token::{Literal, Token, TokenType},
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

    fn start(&self) {
        let args: Vec<String> = env::args().collect();
        if args.len() > 2 {
            println!("Interpreter expects 1 single argument.\nUsage: ornn [script]");
        } else if args.len() == 2 {
            run_file(&args[1]);
        } else {
            run_prompt();
        }
    }
}

fn main() {
    // let interpreter = Ornn::new();
    // interpreter.start();
    let expr = Expr::FunctionCall {
        callee: Box::new(Expr::Variable {
            name: Token::new(TokenType::IDENTIFIER, "myFunc", Literal::Nil, 1),
        }),
        paren: Token::new(TokenType::LEFT_PAREN, "(", Literal::Nil, 1),
        arguments: vec![
            Expr::Literal {
                value: Literal::Number(1.0),
            },
            Expr::Literal {
                value: Literal::Number(2.0),
            },
        ],
    };
    println!("{}", print_expression(&expr));
}

fn run_file(filename: &str) {
    println!("Interpreting file at {}...", filename);
}

fn run_prompt() {
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

        run(command.as_str());
    }
}

fn run(command: &str) {
    let mut lexer = Lexer::new(command);
    lexer = scan_tokens(lexer);

    println!("Command: {}", command);
    for tkn in lexer.tokens {
        print!("[Token: {}, type: {:?}], ", tkn.lexeme, tkn.token_type);
    }
    println!("Finished printing tokens");
}

// fn error(interpreter: &mut Ornn, line: i32, message: &str) {
//     report(interpreter, line, "", message);
// }

// fn report(interpreter: &mut Ornn, line: i32, location: &str, message: &str) {
//     eprintln!("[Line {}] Error {}: {}", line, location, message);

//     interpreter.had_error = true;
//     return;
// }

fn error(line: i32, message: &str) {
    report(line, "", message);
}

fn report(line: i32, location: &str, message: &str) {
    eprintln!("[Line {}] Error {}: {}", line, location, message);

    return;
}
