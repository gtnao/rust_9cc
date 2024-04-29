use std::{env, process};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Token {
    Num(i64),
    Plus,
    Minus,
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

struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, cursor: 0 }
    }
    pub fn consume(&mut self, expected: Token) -> bool {
        if self.tokens[self.cursor] != expected {
            return false;
        }
        self.cursor += 1;
        true
    }
    pub fn expect(&mut self, expected: Token) {
        if self.tokens[self.cursor] != expected {
            panic!("unexpected token: {:?}", self.tokens[self.cursor]);
        }
        self.cursor += 1;
    }
    pub fn expect_num(&mut self) -> i64 {
        if let Token::Num(v) = self.tokens[self.cursor] {
            self.cursor += 1;
            v
        } else {
            panic!("unexpected token: {:?}", self.tokens[self.cursor]);
        }
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("Invalid number of arguments");
        process::exit(1);
    }
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    let tokens = tokenize(&args[1]);
    let mut parser = Parser::new(tokens);

    println!("  mov rax, {}", parser.expect_num());
    while parser.tokens[parser.cursor] != Token::EOF {
        if parser.consume(Token::Plus) {
            println!("  add rax, {}", parser.expect_num());
            continue;
        }
        parser.expect(Token::Minus);
        println!("  sub rax, {}", parser.expect_num());
    }
    println!("  ret");
}
