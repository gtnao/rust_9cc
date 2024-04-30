use crate::parser::{BinaryOperator, AST};

pub fn prologue() {
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");
}

pub fn epilogue() {
    println!("  pop rax");
    println!("  ret");
}

pub fn gen(ast: AST) {
    if let AST::NumberLiteral(v) = ast {
        println!("  push {}", v);
        return;
    }

    if let AST::BinaryOperation(node) = ast {
        gen(*node.lhs);
        gen(*node.rhs);

        println!("  pop rdi");
        println!("  pop rax");

        match node.op {
            BinaryOperator::Add => {
                println!("  add rax, rdi");
            }
            BinaryOperator::Sub => {
                println!("  sub rax, rdi");
            }
            BinaryOperator::Multiply => {
                println!("  imul rax, rdi");
            }
            BinaryOperator::Divide => {
                println!("  cqo");
                println!("  idiv rdi");
            }
            BinaryOperator::Equal => {
                println!("  cmp rax, rdi");
                println!("  sete al");
                println!("  movzb rax, al");
            }
            BinaryOperator::NotEqual => {
                println!("  cmp rax, rdi");
                println!("  setne al");
                println!("  movzb rax, al");
            }
            BinaryOperator::LessThan => {
                println!("  cmp rax, rdi");
                println!("  setl al");
                println!("  movzb rax, al");
            }
            BinaryOperator::LessThanOrEqual => {
                println!("  cmp rax, rdi");
                println!("  setle al");
                println!("  movzb rax, al");
            }
        }
        println!("  push rax");
    }
}
