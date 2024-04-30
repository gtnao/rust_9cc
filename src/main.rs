use std::{env, process};

use rust_9cc::{generator::Generator, lexer::tokenize, parser::Parser};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("Invalid number of arguments");
        process::exit(1);
    }
    let tokens = tokenize(&args[1]);
    let mut parser = Parser::new(tokens);
    let asts = parser.program();
    let mut generator = Generator::new();
    generator.prologue();
    generator.allocate_local_variables(parser.local_variable_count());
    for ast in asts {
        generator.gen(ast);
    }
    generator.epilogue();
}
