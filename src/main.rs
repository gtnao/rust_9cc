use std::{env, process};

use rust_9cc::{
    generator::{allocate_local_variables, epilogue, gen, prologue},
    lexer::tokenize,
    parser::Parser,
};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("Invalid number of arguments");
        process::exit(1);
    }
    let tokens = tokenize(&args[1]);
    let mut parser = Parser::new(tokens);
    let asts = parser.program();
    prologue();
    allocate_local_variables();
    for ast in asts {
        gen(ast);
        println!("  pop rax");
    }
    epilogue();
}
