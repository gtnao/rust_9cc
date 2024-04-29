use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let num = args[1].parse::<u8>().unwrap();
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");
    println!("  mov rax, {}", num);
    println!("  ret");
}
