use std::io::{self, Write};
use std::env;

use interpreter::Interpreter;

mod symbol_table;
mod interpreter;
mod variable;
mod keyword;
mod context;
mod parser;
mod token;
mod lexer;
mod types;
mod node;
mod op;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        run_cli()
    }

    run_file(&args[1])
}

fn run_file(_: &str) {
}

fn run_cli() {
    let mut interpreter = Interpreter::new("").expect("error creating interpreter");
    loop {
        print!("kumi> ");
        io::stdout().flush().expect("flush error");
        
        let mut input = String::new();
        if let Err(err) = io::stdin().read_line(&mut input) {
            println!("{}", err);
            continue
        };
        let input = input.trim();

        if let Err(err) = interpreter.update(input) {
            println!("{}", err);
            continue
        };

        let result = match interpreter.run() {
            Ok(result) => result,
            Err(err) => {
                println!("{}", err);
                continue
            },
        };

        println!("{:?}", result)
    }
}