use crate::lexer::Token;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum AST {
    BinaryOperation(BinaryOperationAST),
    NumberLiteral(i64),
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
}

pub struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, cursor: 0 }
    }
    pub fn program(&mut self) -> AST {
        self.expr()
    }
    fn expr(&mut self) -> AST {
        self.equality()
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
        AST::NumberLiteral(self.expect_number())
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
    fn expect_number(&mut self) -> i64 {
        if let Token::Number(v) = self.tokens[self.cursor] {
            self.cursor += 1;
            v
        } else {
            panic!("unexpected token: {:?}", self.tokens[self.cursor]);
        }
    }
}
