use crate::parser::{BinaryOperator, AST};

pub fn prologue() {
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");
}

pub fn epilogue() {
    println!("  mov rsp, rbp");
    println!("  pop rbp");
    println!("  ret");
}

pub fn allocate_local_variables() {
    println!("  push rbp");
    println!("  mov rbp, rsp");
    println!("  sub rsp, {}", 8 * 26);
}

fn gen_lval(ast: AST) {
    if let AST::LocalVariable(v) = ast {
        println!("  mov rax, rbp");
        println!("  sub rax, {}", v.offset);
        println!("  push rax");
        return;
    }
    panic!("invalid lval");
}

pub fn gen(ast: AST) {
    if let AST::Return(node) = ast {
        gen(*node);
        println!("  pop rax");
        println!("  mov rsp, rbp");
        println!("  pop rbp");
        println!("  ret");
        return;
    }
    if let AST::NumberLiteral(v) = ast {
        println!("  push {}", v);
        return;
    }
    if let AST::LocalVariable(_) = ast {
        gen_lval(ast);
        println!("  pop rax");
        println!("  mov rax, [rax]");
        println!("  push rax");
        return;
    }

    if let AST::BinaryOperation(node) = ast {
        if BinaryOperator::Assign == node.op {
            gen_lval(*node.lhs);
            gen(*node.rhs);

            println!("  pop rdi");
            println!("  pop rax");
            println!("  mov [rax], rdi");
            println!("  push rdi");
            return;
        }

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
            _ => unreachable!(),
        }
        println!("  push rax");
    }
}
