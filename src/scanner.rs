// todo: Lexical analysis.
// 1. Take raw source code
// 2. Group into tokens
//
use crate::tokenizer::{Token, TokenType};

struct Scanner {
    _source: String,
    tokens: Vec<Token>,

    _start: usize,
    _current: usize,
    _line: usize,
}

impl Scanner {
    fn new(source: String) -> Self {
        Self {
            _source: source,
            tokens: Vec::new(),

            _start: 0,
            _current: 0,
            _line: 1,
        }
    }

    fn scan_tokens(&mut self) {
        while !self.is_eof() {
            self._start = self._current;
            self.scan_single_token();
        }
        self.add_token(TokenType::Eof);
    }

    fn add_token(&mut self, type_: TokenType) {
        let token = Token::new(type_, String::new(), None, self._line);
        self.tokens.push(token);
    }

    fn is_eof(&self) -> bool {
        self._current >= self._source.len()
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_eof() {
            return false;
        }

        match self._source.chars().nth(self._current) {
            Some(c) => {
                self._current += 1;
                c == expected
            }
            None => false,
        }
    }

    fn scan_single_token(&mut self) {
        let c: char = self._source.remove(self._current);
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),

            '!' => self.add_token(if self.match_char('=') {
                TokenType::BangEqual
            } else {
                TokenType::Bang
            }),
            '=' => self.add_token(if self.match_char('=') {
                TokenType::EqualEqual
            } else {
                TokenType::Equal
            }),
            '<' => self.add_token(if self.match_char('=') {
                TokenType::LessEqual
            } else {
                TokenType::Less
            }),
            '>' => self.add_token(if self.match_char('=') {
                TokenType::GreaterEqual
            } else {
                TokenType::Greater
            }),

            _ => println!("Unexpected token: {}", c),
        };
        //todo: match character
    }
}
