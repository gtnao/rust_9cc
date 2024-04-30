use std::collections::HashMap;

use crate::lexer::{Keyword, Token};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum AST {
    BinaryOperation(BinaryOperationAST),
    Return(Box<AST>),
    If(IfAST),
    While(WhileAST),
    For(ForAST),
    NumberLiteral(i64),
    LocalVariable(LocalVariableAST),
}
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct BinaryOperationAST {
    pub op: BinaryOperator,
    pub lhs: Box<AST>,
    pub rhs: Box<AST>,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum BinaryOperator {
    Add,
    Sub,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    Assign,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct LocalVariableAST {
    pub name: String,
    pub offset: i64,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct IfAST {
    pub condition: Box<AST>,
    pub then: Box<AST>,
    pub else_: Option<Box<AST>>,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct WhileAST {
    pub condition: Box<AST>,
    pub body: Box<AST>,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ForAST {
    pub init: Option<Box<AST>>,
    pub condition: Option<Box<AST>>,
    pub update: Option<Box<AST>>,
    pub body: Box<AST>,
}

pub struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
    local_variable_map: HashMap<String, i64>,
    local_variable_current_offset: i64,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            cursor: 0,
            local_variable_map: HashMap::new(),
            local_variable_current_offset: 8,
        }
    }
    pub fn program(&mut self) -> Vec<AST> {
        let mut nodes = Vec::new();
        while self.tokens[self.cursor] != Token::EOF {
            nodes.push(self.stmt());
        }
        nodes
    }
    pub fn local_variable_count(&self) -> usize {
        self.local_variable_map.len()
    }
    fn stmt(&mut self) -> AST {
        if self.consume(Token::Keyword(Keyword::Return)) {
            let node = AST::Return(Box::new(self.expr()));
            self.expect(Token::SemiColon);
            return node;
        }
        if self.consume(Token::Keyword(Keyword::If)) {
            self.expect(Token::LeftParen);
            let condition = Box::new(self.expr());
            self.expect(Token::RightParen);
            let then = Box::new(self.stmt());
            let else_ = if self.consume(Token::Keyword(Keyword::Else)) {
                Some(Box::new(self.stmt()))
            } else {
                None
            };
            return AST::If(IfAST {
                condition,
                then,
                else_,
            });
        }
        if self.consume(Token::Keyword(Keyword::While)) {
            self.expect(Token::LeftParen);
            let condition = Box::new(self.expr());
            self.expect(Token::RightParen);
            let body = Box::new(self.stmt());
            return AST::While(WhileAST { condition, body });
        }
        if self.consume(Token::Keyword(Keyword::For)) {
            self.expect(Token::LeftParen);
            let init = if self.consume(Token::SemiColon) {
                None
            } else {
                let node = Some(Box::new(self.expr()));
                self.expect(Token::SemiColon);
                node
            };
            let condition = if self.consume(Token::SemiColon) {
                None
            } else {
                let node = Some(Box::new(self.expr()));
                self.expect(Token::SemiColon);
                node
            };
            let update = if self.consume(Token::RightParen) {
                None
            } else {
                let node = Some(Box::new(self.expr()));
                self.expect(Token::RightParen);
                node
            };
            let body = Box::new(self.stmt());
            return AST::For(ForAST {
                init,
                condition,
                update,
                body,
            });
        }
        let node = self.expr();
        self.expect(Token::SemiColon);
        node
    }
    fn expr(&mut self) -> AST {
        self.assign()
    }
    fn assign(&mut self) -> AST {
        let mut node = self.equality();
        if self.consume(Token::Assign) {
            let rhs = self.assign();
            node = AST::BinaryOperation(BinaryOperationAST {
                op: BinaryOperator::Assign,
                lhs: Box::new(node),
                rhs: Box::new(rhs),
            });
        }
        node
    }
    fn equality(&mut self) -> AST {
        let mut node = self.relational();
        loop {
            if self.consume(Token::Equal) {
                let rhs = self.relational();
                node = AST::BinaryOperation(BinaryOperationAST {
                    op: BinaryOperator::Equal,
                    lhs: Box::new(node),
                    rhs: Box::new(rhs),
                });
            } else if self.consume(Token::NotEqual) {
                let rhs = self.relational();
                node = AST::BinaryOperation(BinaryOperationAST {
                    op: BinaryOperator::NotEqual,
                    lhs: Box::new(node),
                    rhs: Box::new(rhs),
                });
            } else {
                break;
            }
        }
        node
    }
    fn relational(&mut self) -> AST {
        let mut node = self.add();
        loop {
            if self.consume(Token::GreaterThan) {
                let rhs = self.add();
                node = AST::BinaryOperation(BinaryOperationAST {
                    op: BinaryOperator::LessThan,
                    lhs: Box::new(rhs),
                    rhs: Box::new(node),
                });
            } else if self.consume(Token::GreaterThanOrEqual) {
                let rhs = self.add();
                node = AST::BinaryOperation(BinaryOperationAST {
                    op: BinaryOperator::LessThanOrEqual,
                    lhs: Box::new(rhs),
                    rhs: Box::new(node),
                });
            } else if self.consume(Token::LessThan) {
                let rhs = self.add();
                node = AST::BinaryOperation(BinaryOperationAST {
                    op: BinaryOperator::LessThan,
                    lhs: Box::new(node),
                    rhs: Box::new(rhs),
                });
            } else if self.consume(Token::LessThanOrEqual) {
                let rhs = self.add();
                node = AST::BinaryOperation(BinaryOperationAST {
                    op: BinaryOperator::LessThanOrEqual,
                    lhs: Box::new(node),
                    rhs: Box::new(rhs),
                });
            } else {
                break;
            }
        }
        node
    }
    fn add(&mut self) -> AST {
        let mut node = self.mul();
        loop {
            if self.consume(Token::Plus) {
                let rhs = self.mul();
                node = AST::BinaryOperation(BinaryOperationAST {
                    op: BinaryOperator::Add,
                    lhs: Box::new(node),
                    rhs: Box::new(rhs),
                });
            } else if self.consume(Token::Minus) {
                let rhs = self.mul();
                node = AST::BinaryOperation(BinaryOperationAST {
                    op: BinaryOperator::Sub,
                    lhs: Box::new(node),
                    rhs: Box::new(rhs),
                });
            } else {
                break;
            }
        }
        node
    }
    fn mul(&mut self) -> AST {
        let mut node = self.unary();
        loop {
            if self.consume(Token::Asterisk) {
                let rhs = self.unary();
                node = AST::BinaryOperation(BinaryOperationAST {
                    op: BinaryOperator::Multiply,
                    lhs: Box::new(node),
                    rhs: Box::new(rhs),
                });
            } else if self.consume(Token::Slash) {
                let rhs = self.unary();
                node = AST::BinaryOperation(BinaryOperationAST {
                    op: BinaryOperator::Divide,
                    lhs: Box::new(node),
                    rhs: Box::new(rhs),
                });
            } else {
                break;
            }
        }
        node
    }
    fn unary(&mut self) -> AST {
        if self.consume(Token::Plus) {
            return self.primary();
        }
        if self.consume(Token::Minus) {
            let v = self.primary();
            return AST::BinaryOperation(BinaryOperationAST {
                op: BinaryOperator::Sub,
                lhs: Box::new(AST::NumberLiteral(0)),
                rhs: Box::new(v),
            });
        }
        self.primary()
    }
    fn primary(&mut self) -> AST {
        if self.consume(Token::LeftParen) {
            let v = self.expr();
            self.expect(Token::RightParen);
            return v;
        }
        if let Token::Number(_) = self.tokens[self.cursor] {
            return self.expect_number();
        }
        self.expect_local_variable()
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
    fn expect_number(&mut self) -> AST {
        if let Token::Number(v) = self.tokens[self.cursor] {
            self.cursor += 1;
            AST::NumberLiteral(v)
        } else {
            panic!("unexpected token: {:?}", self.tokens[self.cursor]);
        }
    }
    fn expect_local_variable(&mut self) -> AST {
        if let Token::Identifier(v) = &self.tokens[self.cursor] {
            self.cursor += 1;
            if let Some(offset) = self.local_variable_map.get(v) {
                AST::LocalVariable(LocalVariableAST {
                    name: v.clone(),
                    offset: *offset,
                })
            } else {
                let offset = self.local_variable_current_offset;
                self.local_variable_map.insert(v.clone(), offset);
                self.local_variable_current_offset += 8;
                AST::LocalVariable(LocalVariableAST {
                    name: v.clone(),
                    offset,
                })
            }
        } else {
            panic!("unexpected token: {:?}", self.tokens[self.cursor]);
        }
    }
}
