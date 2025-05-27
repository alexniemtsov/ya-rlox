// Scanner reads provided string and returns tokens instead.

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    // Single-character tokens
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

    // One or two character tokkens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
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

// Owns everything. Looks like something could be wrong with `lexeme` and heap allocations
#[derive(Debug, Clone)]
pub struct Token {
    pub type_: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Boolean(bool),
    Nil,
    String(String),
    Integer(i64),
    Float(f64),
}

impl Token {
    pub fn new(type_: TokenType, lexeme: String, literal: Option<Literal>, line: usize) -> Self {
        Self {
            type_,
            lexeme,
            literal,
            line,
        }
    }
}

pub struct Scanner {
    pub tokens: Vec<Token>,

    _source: String,
    _start: usize,
    _current: usize,
    _line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            _source: source,
            tokens: Vec::new(),

            _start: 0,
            _current: 0,
            _line: 1,
        }
    }

    // Consumes self and return tokens
    // Should return Result
    pub fn scan_tokens(mut self) -> Vec<Token> {
        while !self.is_eof() {
            self._start = self._current;
            self.scan_single_token();
        }
        self.add_token(TokenType::Eof);
        self.tokens
    }

    fn add_token(&mut self, type_: TokenType) {
        let token = Token::new(type_, self.get_lexeme(), None, self._line);
        self.tokens.push(token);
    }

    fn add_token_with_literal(&mut self, type_: TokenType, literal: Literal) {
        let token = Token::new(type_, self.get_lexeme(), Some(literal), self._line);
        self.tokens.push(token);
    }

    fn get_lexeme(&self) -> String {
        let text = &self._source[self._start..self._current];
        text.to_string()
    }

    fn is_eof(&self) -> bool {
        self._current >= self._source.len()
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_eof() {
            return false;
        }

        let next = self._source[self._current..].chars().next().unwrap();
        if next != expected {
            return false;
        }
        self._current += expected.len_utf8();
        true
    }

    fn advance(&mut self) -> char {
        let ch = self._source[self._current..].chars().next().unwrap();
        self._current += ch.len_utf8();
        ch
    }

    fn peek(&self) -> char {
        if self.is_eof() {
            return '\0';
        }
        self._source[self._current..].chars().next().unwrap()
    }

    fn peek_next(&self) -> char {
        if self._current + 1 >= self._source.len() {
            return '\0';
        }
        self._source[self._current + 1..].chars().next().unwrap()
    }

    fn scan_single_token(&mut self) {
        let ch: char = self.advance();
        match ch {
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

            '!' => self.add_conditional_token('=', TokenType::BangEqual, TokenType::Bang),
            '=' => self.add_conditional_token('=', TokenType::EqualEqual, TokenType::Equal),
            '<' => self.add_conditional_token('=', TokenType::LessEqual, TokenType::Less),
            '>' => self.add_conditional_token('=', TokenType::GreaterEqual, TokenType::Greater),

            // Long lexemes
            '/' => self.lookahead('/', TokenType::Slash, None),
            // Ignored
            ' ' | '\r' | '\t' => {}

            '\n' => {
                self._line += 1;
            }

            '"' => self.string(),

            _ => {
                if ch.is_ascii_digit() {
                    return self.number();
                }
                if ch.is_ascii_alphabetic() || ch == '_' {
                    return self.identifier();
                }
                println!("Unexpected token: {}", ch)
            }
        };
        //todo: match character
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_eof() {
            if self.peek() == '\n' {
                self._line += 1;
            }
            self.advance();
        }

        if self.is_eof() {
            // todo: throw error
            return;
        }

        let val = &self._source[self._start + 1..self._current];
        let lit = Literal::String(val.to_string());

        self.advance();
        self.add_token_with_literal(TokenType::String, lit);
    }

    fn number(&mut self) {
        // Integer part
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // Look for fractional part
        let mut is_float = false;
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            is_float = true;
            self.advance(); // consume "."
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }
        let val = &self._source[self._start..self._current];
        let lit = match is_float {
            true => Literal::Float(val.parse().unwrap()),
            false => Literal::Integer(val.parse().unwrap()),
        };

        self.add_token_with_literal(TokenType::Number, lit);
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }
        let token = self.keyword_to_token(&self._source[self._start..self._current]);

        match token {
            Some(t) => self.add_token(t),
            None => self.add_token(TokenType::Identifier),
        };
    }

    fn add_conditional_token(&mut self, expect: char, if_match: TokenType, if_not: TokenType) {
        let t = if self.match_char(expect) {
            if_match
        } else {
            if_not
        };
        self.add_token(t);
    }

    fn lookahead(&mut self, expect: char, on_found: TokenType, _escape: Option<char>) {
        // todo: handle multiline comments /* ... */
        let esc = _escape.unwrap_or('\n');
        let next = self.match_char(expect);

        if next {
            while self.peek() != esc && !self.is_eof() {
                self.advance();
            }
        } else {
            self.add_token(on_found);
        }
    }

    // Todo: investigate about crate `phf`. Generates static hash maps at compile time.
    fn keyword_to_token(&self, keyword: &str) -> Option<TokenType> {
        match keyword {
            "and" => Some(TokenType::And),
            "class" => Some(TokenType::Class),
            "else" => Some(TokenType::Else),
            "false" => Some(TokenType::False),
            "for" => Some(TokenType::For),
            "fun" => Some(TokenType::Fun),
            "if" => Some(TokenType::If),
            "nil" => Some(TokenType::Nil),
            "while" => Some(TokenType::While),
            "or" => Some(TokenType::Or),
            "print" => Some(TokenType::Print),
            "return" => Some(TokenType::Return),
            "super" => Some(TokenType::Super),
            "this" => Some(TokenType::This),
            "true" => Some(TokenType::True),
            "var" => Some(TokenType::Var),
            _ => None,
        }
    }
}
