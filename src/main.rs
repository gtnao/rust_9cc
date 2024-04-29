use std::{env, process};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Token {
    Num(i64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    LParen,
    RParen,
    EOF,
}

pub fn tokenize(s: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut iter = s.chars().peekable();
    loop {
        match iter.peek() {
            Some(&c) if c.is_whitespace() => {
                iter.next();
            }
            Some(&c) if c.is_ascii_digit() => {
                let mut ret = String::new();
                loop {
                    match iter.peek() {
                        Some(&cc) if cc.is_ascii_digit() => {
                            ret.push(cc);
                            iter.next();
                        }
                        _ => {
                            break;
                        }
                    }
                }
                if let Ok(v) = ret.parse::<i64>() {
                    tokens.push(Token::Num(v));
                } else {
                    panic!("failed convert: {}", ret);
                }
            }
            Some(&'+') => {
                tokens.push(Token::Plus);
                iter.next();
            }
            Some(&'-') => {
                tokens.push(Token::Minus);
                iter.next();
            }
            Some(&'*') => {
                tokens.push(Token::Asterisk);
                iter.next();
            }
            Some(&'/') => {
                tokens.push(Token::Slash);
                iter.next();
            }
            Some(&'(') => {
                tokens.push(Token::LParen);
                iter.next();
            }
            Some(&')') => {
                tokens.push(Token::RParen);
                iter.next();
            }
            Some(&c) => {
                panic!("unexpected character: {}", c);
            }
            None => {
                tokens.push(Token::EOF);
                break;
            }
        }
    }
    tokens
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum NodeKind {
    Add,
    Sub,
    Mul,
    Div,
    Num(i64),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Node {
    pub kind: NodeKind,
    pub lhs: Option<Box<Node>>,
    pub rhs: Option<Box<Node>>,
}

struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, cursor: 0 }
    }
    pub fn program(&mut self) -> Node {
        self.expr()
    }
    fn expr(&mut self) -> Node {
        let mut node = self.mul();
        loop {
            if self.consume(Token::Plus) {
                let rhs = self.mul();
                node = Node {
                    kind: NodeKind::Add,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(rhs)),
                };
            } else if self.consume(Token::Minus) {
                let rhs = self.mul();
                node = Node {
                    kind: NodeKind::Sub,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(rhs)),
                };
            } else {
                break;
            }
        }
        node
    }
    fn mul(&mut self) -> Node {
        let mut node = self.unary();
        loop {
            if self.consume(Token::Asterisk) {
                let rhs = self.unary();
                node = Node {
                    kind: NodeKind::Mul,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(rhs)),
                };
            } else if self.consume(Token::Slash) {
                let rhs = self.unary();
                node = Node {
                    kind: NodeKind::Div,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(rhs)),
                };
            } else {
                break;
            }
        }
        node
    }
    fn unary(&mut self) -> Node {
        if self.consume(Token::Plus) {
            return self.primary();
        }
        if self.consume(Token::Minus) {
            let v = self.primary();
            return Node {
                kind: NodeKind::Sub,
                lhs: Some(Box::new(Node {
                    kind: NodeKind::Num(0),
                    lhs: None,
                    rhs: None,
                })),
                rhs: Some(Box::new(v)),
            };
        }
        self.primary()
    }
    fn primary(&mut self) -> Node {
        if self.consume(Token::LParen) {
            let v = self.expr();
            self.expect(Token::RParen);
            return v;
        }
        Node {
            kind: NodeKind::Num(self.expect_num()),
            lhs: None,
            rhs: None,
        }
    }

    fn consume(&mut self, expected: Token) -> bool {
        if self.tokens[self.cursor] != expected {
            return false;
        }
        self.cursor += 1;
        true
    }
    fn expect(&mut self, expected: Token) {
        if self.tokens[self.cursor] != expected {
            panic!("unexpected token: {:?}", self.tokens[self.cursor]);
        }
        self.cursor += 1;
    }
    fn expect_num(&mut self) -> i64 {
        if let Token::Num(v) = self.tokens[self.cursor] {
            self.cursor += 1;
            v
        } else {
            panic!("unexpected token: {:?}", self.tokens[self.cursor]);
        }
    }
}

fn gen(node: Node) {
    if let NodeKind::Num(v) = node.kind {
        println!("  push {}", v);
        return;
    }

    if let Some(lhs) = node.lhs {
        gen(*lhs);
    }
    if let Some(rhs) = node.rhs {
        gen(*rhs);
    }

    println!("  pop rdi");
    println!("  pop rax");

    match node.kind {
        NodeKind::Add => {
            println!("  add rax, rdi");
        }
        NodeKind::Sub => {
            println!("  sub rax, rdi");
        }
        NodeKind::Mul => {
            println!("  imul rax, rdi");
        }
        NodeKind::Div => {
            println!("  cqo");
            println!("  idiv rdi");
        }
        _ => {}
    }
    println!("  push rax");
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("Invalid number of arguments");
        process::exit(1);
    }

    let tokens = tokenize(&args[1]);
    let mut parser = Parser::new(tokens);
    let node = parser.program();

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    gen(node);

    println!("  pop rax");
    println!("  ret");
}
