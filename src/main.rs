use std::{iter, env};

fn main() {
    let mut args = env::args();
    args.next();
    let question = match args.next() {
        Some(arg) => arg,
        None => panic!(),
    };
    let tokens = tokeniser(String::from(&question)).unwrap();

    let mut parser = Parser::new(tokens);
    let nodes = parser.parse_add();
    println!("{} = {}", question, nodes.eval())
}

fn tokeniser(input: String) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = vec![];
    let mut iter = input.chars().peekable();

    while let Some(ch) = iter.next() {
        match ch {
            ch if ch.is_whitespace() => continue,
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Dash),
            '*' => tokens.push(Token::Star),
            '/' => tokens.push(Token::Slash),
            '0'..='9' => {
                let n: i64 = iter::once(ch)
                    .chain(iter::from_fn(|| {
                        iter.by_ref().next_if(|s| s.is_ascii_digit())
                    }))
                    .collect::<String>()
                    .parse()
                    .unwrap();
                tokens.push(Token::Number(n));
            }
            _ => return Err(format!("unrecognised character {}", ch)),
        }
    }
    Ok(tokens)
}

#[derive(Debug)]
enum Token {
    Number(i64),
    Plus,
    Dash,
    Star,
    Slash,
}

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide
}

#[derive(Debug, Clone)]
enum AstNode {
    Number(i64),
    BinaryOp {
        op: Operator,
        left: Box<AstNode>,
        right: Box<AstNode>,
    }
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens,
            pos: 0,
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn next(&mut self) -> Option<&Token> {
        let tok = self.tokens.get(self.pos);
        if tok.is_some() {
            self.pos += 1;
        }
        tok
    }

    fn parse_add(&mut self) -> AstNode {
        let mut node = self.parse_mult();
        while let Some(token) = self.peek() {
            match token {
                Token::Plus | Token::Dash => {
                    let op = match self.next() {
                        Some(Token::Plus) => Operator::Add,
                        Some(Token::Dash) => Operator::Subtract,
                        _ => unreachable!(),
                    };
                    let right = self.parse_mult();

                    node = AstNode::BinaryOp {
                        op: op,
                        left: Box::new(node),
                        right: Box::new(right)
                    };
                },
                _ => break
            }
        }
        node
    }

    fn parse_mult(&mut self) -> AstNode {
        let mut node = self.parse_num();
        while let Some(token) = self.peek() {
            match token {
                Token::Star | Token::Slash => {
                    let op = match self.next() {
                        Some(Token::Star) => Operator::Multiply,
                        Some(Token::Slash) => Operator::Divide,
                        _ => unreachable!()
                    };
                    let right = self.parse_num();
                    
                    node = AstNode::BinaryOp {
                        op: op,
                        left: Box::new(node),
                        right: Box::new(right)
                    };
                },
                _ => break
            }
        }
        node
    }

    fn parse_num(&mut self) -> AstNode {
        match self.next() {
            Some(Token::Number(n)) => AstNode::Number(*n),
            Some(unexpected) => panic!("Expected a number, but got {:?}", unexpected),
            None => panic!("Expected a number, but found end of input"),
        }
    }
}

impl AstNode {
    fn eval(&self) -> i64 {
        match self {
            AstNode::Number(n) => *n,
            AstNode::BinaryOp { op, left, right } => {
                let l = left.eval();
                let r = right.eval();
                match op {
                    Operator::Add => l + r,
                    Operator::Subtract => l - r,
                    Operator::Multiply => l * r,
                    Operator::Divide => l / r,
                }
            } 
        }
    }
}