use core::error;
use std::{env, io};

struct Ornn {
    had_error: bool,
}

impl Ornn {
    fn new() -> Ornn {
        Ornn { had_error: false }
    }

    fn start(&self) {
        let args: Vec<String> = env::args().collect();
        if args.len() < 2 {
            println!("Interpreter expects 1 single argument.\nUsage: ornn [script]");
        } else if args.len() == 2 {
            run_file(&args[1]);
        } else {
            run_prompt();
        }
    }
}

fn main() {
    let interpreter = Ornn::new();
    interpreter.start();
}

fn run_file() {
    println!("Interpreting file...");
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
    }

    run(command.as_str());
}

fn run(command: &str) {
    println!("Running ")
}

fn error(line: i32, message: &str) {
    report(line, "", message);
}

fn report(line: i32, location: &str, message: &str) {
    panic!("[Line {}] Error {}: {}", line, location, message);
}
