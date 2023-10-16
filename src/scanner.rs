use crate::{
    token::{Object, Token},
    token_type::TokenType,
    Run::Lox,
};

use substring::Substring;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u32,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let tokens: Vec<Token> = Vec::new();
        Self {
            source,
            tokens,
            line: 1,
            start: 0,
            current: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            crate::token_type::TokenType::EOF,
            "".to_string(),
            None,
            self.line,
        ));

        self.tokens
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LEFT_PAREN),
            ')' => self.add_token(TokenType::RIGHT_PAREN),
            '{' => self.add_token(TokenType::LEFT_BRACE),
            '}' => self.add_token(TokenType::RIGHT_BRACE),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BANG_EQUAL);
                } else {
                    self.add_token(TokenType::BANG);
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EQUAL_EQUAL);
                } else {
                    self.add_token(TokenType::EQUAL);
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LESS_EQUAL);
                } else {
                    self.add_token(TokenType::LESS)
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GREATER_EQUAL);
                } else {
                    self.add_token(TokenType::GREATER)
                }
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && self.is_at_end() {
                        self.advance();
                    }
                }
            }
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => {
                self.line += 1;
            }
            '"' => {
                self.string();
            }
            _ => {
                if self.is_digit(c) {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    let lox = Lox::new();
                    lox.error(self.line, "Unexpected character");
                }
            }
        }
    }

    fn identifier(&self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        self.add_token(TokenType::IDENTIFIER);
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        self._add_token(
            TokenType::NUMBER,
            Some(Object::Number(
                self.source
                    .substring(self.start, self.current)
                    .to_string()
                    .parse::<f64>()
                    .unwrap(),
            )),
        )
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        return self.source.chars().nth(self.current + 1).unwrap();
    }

    fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c <= 'A' && c <= 'Z') || c == '_'
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    fn string(&mut self) {
        while self.peek() != '"' && self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            let error = Lox::new();
            error.error(self.line, "Unterminated string");
            return;
        }

        self.advance();

        let value = self
            .source
            .substring(self.start + 1, self.current - 1)
            .to_string();
        self._add_token(TokenType::STRING, Some(Object::String(value)));
    }

    fn match_char(&self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            self.current += 1;
            return false;
        }
        return true;
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source.chars().nth(self.current).unwrap();
    }

    fn is_digit(&self, c: char) -> bool {
        return c >= '0' && c <= '9';
    }

    fn add_token(&self, token_type: TokenType) {
        self._add_token(token_type, None)
    }

    fn _add_token(&self, token_type: TokenType, literal: Option<Object>) {
        let text = self.source.substring(self.start, self.current);
        self.tokens
            .push(Token::new(token_type, text.to_owned(), literal, self.line))
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
