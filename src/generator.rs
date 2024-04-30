use crate::parser::{BinaryOperator, AST};

pub struct Generator {
    label_count: i64,
}
impl Generator {
    pub fn new() -> Self {
        Self { label_count: 0 }
    }
    pub fn prologue(&self) {
        println!(".intel_syntax noprefix");
        println!(".global main");
        println!("main:");
    }
    pub fn epilogue(&self) {
        println!("  mov rsp, rbp");
        println!("  pop rbp");
        println!("  ret");
    }
    pub fn allocate_local_variables(&self, local_variable_count: usize) {
        println!("  push rbp");
        println!("  mov rbp, rsp");
        println!("  sub rsp, {}", 8 * local_variable_count);
    }
    pub fn gen(&mut self, ast: AST) {
        if let AST::Block(nodes) = ast {
            for node in nodes {
                self.gen(node);
                // println!("  pop rax");
            }
            return;
        }
        if let AST::Return(node) = ast {
            self.gen(*node);
            println!("  pop rax");
            println!("  mov rsp, rbp");
            println!("  pop rbp");
            println!("  ret");
            return;
        }
        if let AST::If(node) = ast {
            if let Some(else_) = node.else_ {
                self.gen(*node.condition);
                println!("  pop rax");
                println!("  cmp rax, 0");
                let else_label = format!(".Lelse{}", self.label_count);
                self.label_count += 1;
                println!("  je {}", else_label);
                self.gen(*node.then);
                let end_label = format!(".Lend{}", self.label_count);
                println!("  jmp {}", end_label);
                println!("{}:", else_label);
                self.gen(*else_);
                println!("{}:", end_label);
                return;
            }
            self.gen(*node.condition);
            println!("  pop rax");
            println!("  cmp rax, 0");
            let end_label = format!(".Lend{}", self.label_count);
            self.label_count += 1;
            println!("  je {}", end_label);
            self.gen(*node.then);
            println!("{}:", end_label);
            return;
        }
        if let AST::While(node) = ast {
            let begin_label = format!(".Lbegin{}", self.label_count);
            self.label_count += 1;
            println!("{}:", begin_label);
            self.gen(*node.condition);
            println!("  pop rax");
            println!("  cmp rax, 0");
            let end_label = format!(".Lend{}", self.label_count);
            self.label_count += 1;
            println!("  je {}", end_label);
            self.gen(*node.body);
            println!("  jmp {}", begin_label);
            println!("{}:", end_label);
            return;
        }
        if let AST::For(node) = ast {
            if let Some(init) = node.init {
                self.gen(*init);
            }
            let begin_label = format!(".Lbegin{}", self.label_count);
            self.label_count += 1;
            println!("{}:", begin_label);
            let end_label = format!(".Lend{}", self.label_count);
            self.label_count += 1;
            if let Some(condition) = node.condition {
                self.gen(*condition);
                println!("  pop rax");
                println!("  cmp rax, 0");
                println!("  je {}", end_label);
            }
            self.gen(*node.body);
            if let Some(update) = node.update {
                self.gen(*update);
            }
            println!("  jmp {}", begin_label);
            println!("{}:", end_label);
            return;
        }
        if let AST::NumberLiteral(v) = ast {
            println!("  push {}", v);
            return;
        }
        if let AST::LocalVariable(_) = ast {
            self.gen_lval(ast);
            println!("  pop rax");
            println!("  mov rax, [rax]");
            println!("  push rax");
            return;
        }

        if let AST::BinaryOperation(node) = ast {
            if BinaryOperator::Assign == node.op {
                self.gen_lval(*node.lhs);
                self.gen(*node.rhs);

                println!("  pop rdi");
                println!("  pop rax");
                println!("  mov [rax], rdi");
                println!("  push rdi");
                return;
            }

            self.gen(*node.lhs);
            self.gen(*node.rhs);

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
    fn gen_lval(&self, ast: AST) {
        if let AST::LocalVariable(v) = ast {
            println!("  mov rax, rbp");
            println!("  sub rax, {}", v.offset);
            println!("  push rax");
            return;
        }
        panic!("invalid lval");
    }
}
