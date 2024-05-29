use std::env;
use std::fs;
use std::io;
use std::process::exit;

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash,
    LeftParen,
    RightParen,
}

#[derive(Debug)]
enum Expr {
    Number(f64),
    UnaryOp { op: Token, expr: Box<Expr> },
    BinaryOp { left: Box<Expr>, op: Token, right: Box<Expr> },
}

fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            '0'..='9' => {
                let mut num = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_digit(10) || c == '.' {
                        num.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(num.parse().unwrap()));
            }
            '+' => {
                chars.next();
                tokens.push(Token::Plus);
            }
            '-' => {
                chars.next();
                tokens.push(Token::Minus);
            }
            '*' => {
                chars.next();
                tokens.push(Token::Star);
            }
            '/' => {
                chars.next();
                tokens.push(Token::Slash);
            }
            '(' => {
                chars.next();
                tokens.push(Token::LeftParen);
            }
            ')' => {
                chars.next();
                tokens.push(Token::RightParen);
            }
            ' ' | '\t' | '\n' | '\r' => {
                chars.next();
            }
            _ => panic!("Unexpected character: {}", c),
        }
    }

    tokens
}

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn parse(&mut self) -> Expr {
        self.expression()
    }

    fn expression(&mut self) -> Expr {
        self.term()
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.match_tokens(&[Token::Plus, Token::Minus]) {
            let op = self.previous().clone();
            let right = self.factor();
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.match_tokens(&[Token::Star, Token::Slash]) {
            let op = self.previous().clone();
            let right = self.unary();
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_tokens(&[Token::Minus]) {
            let op = self.previous().clone();
            let right = self.unary();
            return Expr::UnaryOp {
                op,
                expr: Box::new(right),
            };
        }

        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self.match_tokens(&[Token::Number(0.0)]) {
            if let Token::Number(value) = self.previous() {
                return Expr::Number(*value);
            }
        }

        if self.match_tokens(&[Token::LeftParen]) {
            let expr = self.expression();
            self.consume(Token::RightParen, "Expect ')' after expression.");
            return expr;
        }

        panic!("Unexpected token");
    }

    fn match_tokens(&mut self, types: &[Token]) -> bool {
        for token in types {
            if self.check(token) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token: &Token) -> bool {
        if self.is_at_end() {
            return false;
        }
        std::mem::discriminant(self.peek()) == std::mem::discriminant(token)
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, token: Token, message: &str) {
        if self.check(&token) {
            self.advance();
        } else {
            panic!("{}", message);
        }
    }
}

fn eval(expr: &Expr) -> f64 {
    match expr {
        Expr::Number(value) => *value,
        Expr::UnaryOp { op, expr } => {
            let value = eval(expr);
            match op {
                Token::Minus => -value,
                _ => panic!("Unexpected unary operator"),
            }
        }
        Expr::BinaryOp { left, op, right } => {
            let left_val = eval(left);
            let right_val = eval(right);
            match op {
                Token::Plus => left_val + right_val,
                Token::Minus => left_val - right_val,
                Token::Star => left_val * right_val,
                Token::Slash => left_val / right_val,
                _ => panic!("Unexpected binary operator"),
            }
        }
    }
}

fn run_file(path: &str) -> Result<(), String> {
    match fs::read_to_string(path) {
        Err(msg) => Err(msg.to_string()),
        Ok(contents) => run(&contents),
    }
}

fn run(contents: &String) -> Result<(), String> {
    let tokens = lex(contents);
    let mut parser = Parser::new(tokens);
    let expr = parser.parse();
    let result = eval(&expr);
    println!("Result: {}", result);
    Ok(())
}

fn run_prompt() -> Result<(), String> {
    print!("> ");
    io::Write::flush(&mut io::stdout()).expect("flush failed!");
    let mut buffer = String::new();
    let stdin = io::stdin();
    match stdin.read_line(&mut buffer) {
        Ok(_) => (),
        Err(_) => return Err("Could not read line".to_string()),
    }
    let trimmed = buffer.trim();
    let tokens = lex(trimmed);
    let mut parser = Parser::new(tokens);
    let expr = parser.parse();
    let result = eval(&expr);
    println!("Result: {}", result);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: jlox [script]");
        exit(64);
    } else if args.len() == 2 {
        match run_file(&args[1]) {
            Ok(_) => exit(0),
            Err(msg) => {
                println!("Error: \n{}", msg); 
                exit(1);
            }
        }
    } else {
        match run_prompt() {
            Ok(_) => (),
            Err(msg) => {
                println!("Error: \n{}", msg); 
                exit(1);
            }
        }
    }
}
