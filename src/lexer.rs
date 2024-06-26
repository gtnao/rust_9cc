#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Token {
    Identifier(String),
    Number(i64),
    Keyword(Keyword),
    Plus,
    Minus,
    Asterisk,
    Slash,
    LeftParen,
    RightParen,
    Assign,
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    SemiColon,
    LeftBrace,
    RightBrace,
    EOF,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Keyword {
    Return,
    If,
    Else,
    While,
    For,
}

pub fn tokenize(s: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut iter = s.chars().peekable();
    loop {
        match iter.peek() {
            Some(&c) if c.is_whitespace() => {
                iter.next();
            }
            Some(&c) if '_' == c || c.is_alphabetic() => {
                let mut ret = String::new();
                loop {
                    match iter.peek() {
                        Some(&cc) if '_' == cc || cc.is_ascii_digit() || cc.is_alphabetic() => {
                            ret.push(cc);
                            iter.next();
                        }
                        _ => {
                            break;
                        }
                    }
                }
                match ret.as_str() {
                    "return" => {
                        tokens.push(Token::Keyword(Keyword::Return));
                    }
                    "if" => {
                        tokens.push(Token::Keyword(Keyword::If));
                    }
                    "else" => {
                        tokens.push(Token::Keyword(Keyword::Else));
                    }
                    "while" => {
                        tokens.push(Token::Keyword(Keyword::While));
                    }
                    "for" => {
                        tokens.push(Token::Keyword(Keyword::For));
                    }
                    _ => {
                        tokens.push(Token::Identifier(ret));
                    }
                }
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
                    tokens.push(Token::Number(v));
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
                tokens.push(Token::LeftParen);
                iter.next();
            }
            Some(&')') => {
                tokens.push(Token::RightParen);
                iter.next();
            }
            Some(&'=') => {
                iter.next();
                match iter.peek() {
                    Some(&'=') => {
                        tokens.push(Token::Equal);
                        iter.next();
                    }
                    _ => {
                        tokens.push(Token::Assign);
                    }
                }
            }
            Some(&'!') => {
                iter.next();
                match iter.peek() {
                    Some(&'=') => {
                        tokens.push(Token::NotEqual);
                        iter.next();
                    }
                    Some(&cc) => {
                        panic!("unexpected character: {}", cc);
                    }
                    None => {
                        panic!("unexpected EOF");
                    }
                }
            }
            Some(&'>') => {
                iter.next();
                match iter.peek() {
                    Some(&'=') => {
                        tokens.push(Token::GreaterThanOrEqual);
                        iter.next();
                    }
                    _ => {
                        tokens.push(Token::GreaterThan);
                    }
                }
            }
            Some(&'<') => {
                iter.next();
                match iter.peek() {
                    Some(&'=') => {
                        tokens.push(Token::LessThanOrEqual);
                        iter.next();
                    }
                    _ => {
                        tokens.push(Token::LessThan);
                    }
                }
            }
            Some(&';') => {
                tokens.push(Token::SemiColon);
                iter.next();
            }
            Some(&'{') => {
                tokens.push(Token::LeftBrace);
                iter.next();
            }
            Some(&'}') => {
                tokens.push(Token::RightBrace);
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
