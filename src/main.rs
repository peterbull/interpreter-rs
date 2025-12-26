#![allow(unused_variables, dead_code)]
use std::env;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // one or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // literals.
    Identifier,
    String,
    Number,

    // keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

fn lox_error(line: usize, msg: String) {
    println!("ERROR: Line {}, {}", line, msg);
}

#[derive(Debug)]
enum Literal {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    lexeme: String,
    object: Option<Literal>,
    line: usize,
}

impl Token {
    fn new(token_type: TokenType, lexeme: String, object: Option<Literal>, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            object,
            line,
        }
    }
}

struct Scanner {
    source: String,
    tokens: Vec<Token>,
    line: usize,
    start: usize,
    current: usize,
}

impl Scanner {
    fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            line: 1,
            start: 0,
            current: 0,
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        let lexeme = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(token_type, lexeme, None, self.line));
    }
    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Literal) {
        let lexeme = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(token_type, lexeme, Some(literal), self.line));
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }
    fn handle_token(&mut self, c: &char) {
        match c {
            '+' => self.add_token(TokenType::Plus),
            '-' => self.add_token(TokenType::Minus),
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.match_next_char('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }
            '=' => {
                if self.match_next_char('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            }
            '<' => {
                if self.match_next_char('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            }
            '>' => {
                if self.match_next_char('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }
            '/' => {
                if self.match_next_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => {
                self.line += 1;
            }
            '"' => {}
            _ => {}
        }
    }
    fn handle_string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
            let str_val = self.source[self.start + 1..self.current - 1].to_string();
        }
        // closing "
        self.advance();
    }

    fn match_next_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        let c = self.source.chars().nth(self.current).unwrap();
        if c != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap()
        }
    }

    fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            let c = self.advance();
            self.handle_token(&c);
        }
    }

    fn print_info(&self) {
        println!("printing tokens:");
        for tok in &self.tokens {
            println!("{:?}", tok);
        }
        println!("end lines: {}", self.line);
        println!("finish value for current: {}", self.current);
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

struct Lox {}
impl Lox {
    fn run(text: &str) {
        let mut scanner = Scanner::new(text.to_string());
        scanner.scan_tokens();
        scanner.print_info();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            eprintln!("Logs from your program will appear here!");

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            if !file_contents.is_empty() {
                Lox::run(&file_contents);
                // panic!("Scanner not implemented");
            } else {
                println!("EOF  null");
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_output() {
        assert_eq!(1, 1)
    }
}
