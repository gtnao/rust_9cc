use std::{env, process};

use rust_9cc::{
    generator::{epilogue, gen, prologue},
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
    let ast = parser.program();
    prologue();
    gen(ast);
    epilogue();
}
