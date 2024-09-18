use std::fmt::Display;

use crate::LoxErr;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(contents: &str) -> Self {
        Self {
            source: contents.to_string(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }
    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, LoxErr> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: "".to_string(),
            literal: None,
            line_number: self.line,
        });

        Ok(self.tokens.clone())
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) -> Result<(), LoxErr> {
        let c = self.advance();
        match c {
            '{' => self.add_token(TokenType::LEFTBRACE),
            '}' => self.add_token(TokenType::RIGHTBRACE),
            '(' => self.add_token(TokenType::LEFTPAREN),
            ')' => self.add_token(TokenType::RIGHTPAREN),
            ',' => self.add_token(TokenType::COMMA),
            '-' => self.add_token(TokenType::MINUS),
            '.' => self.add_token(TokenType::DOT),
            '*' => self.add_token(TokenType::STAR),
            '+' => self.add_token(TokenType::PLUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '!' => {
                let is_match = self.char_match('=');
                if is_match {
                    self.add_token(TokenType::BANGEQUAL);
                } else {
                    self.add_token(TokenType::BANG)
                }
            }
            '<' => {
                let is_match = self.char_match('=');
                if is_match {
                    self.add_token(TokenType::LESSEQUAL);
                } else {
                    self.add_token(TokenType::LESS)
                }
            }
            '>' => {
                let is_match = self.char_match('=');
                if is_match {
                    self.add_token(TokenType::GREATEREQUAL);
                } else {
                    self.add_token(TokenType::GREATER)
                }
            }
            '=' => {
                let is_match = self.char_match('=');
                if is_match {
                    self.add_token(TokenType::EQUALEQUAL);
                } else {
                    self.add_token(TokenType::EQUAL)
                }
            }
            '/' => {
                if self.char_match('/') {
                    loop {
                        if self.peek() == '\n' || self.is_at_end() {
                            break;
                        }
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::SLASH)
                }
            }
            '"' => self.string()?,
            '\n' => self.line += 1,
            ' ' => {}
            '\r' => {}
            '\t' => {}
            c => {
                if self.is_digit(c) {
                    self.number();
                }
            }
        }
        Ok(())
    }

    fn number(&mut self) -> Result<(), LoxErr> {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();
        }

        while self.is_digit(self.peek()) {
            self.advance();
        }

        let num_lit = &self.source[self.start..self.current];
        let num_lit: f64 = num_lit.parse()?;
        let num_lit = LiteralValue::FValue(num_lit);
        self.add_token_literal(TokenType::NUMBER, Some(num_lit));

        Ok(())
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\n';
        }
        self.source.as_bytes()[self.current + 1] as char
    }

    fn is_digit(&self, n: char) -> bool {
        return n >= '0' && n <= '9';
    }

    fn string(&mut self) -> Result<(), LoxErr> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err("Reached EOF".into());
        }

        // Closing the '"'
        self.advance();

        let val = &self.source[self.start + 1..self.current];

        let string_lit = LiteralValue::StringValue(val.to_string());

        self.add_token_literal(TokenType::STRINGLIT, Some(string_lit));

        Ok(())
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\n';
        }
        self.source.as_bytes()[self.current] as char
    }

    fn char_match(&mut self, symbol: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        let c = self.source.as_bytes()[self.current];
        self.current += 1;
        c as char == symbol
    }

    fn advance(&mut self) -> char {
        let c = self.source.as_bytes()[self.current];
        self.current += 1;
        c as char
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, None);
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: Option<LiteralValue>) {
        let lexeme: String = self.source.as_bytes()[self.start..self.current]
            .iter()
            .map(|x| *x as char)
            .collect();

        self.tokens.push(Token {
            token_type,
            lexeme,
            literal,
            line_number: self.line,
        });
    }
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    IntValue(i64),
    FValue(f64),
    StringValue(String),
    IdentifierValue(String),
}

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<LiteralValue>,
    line_number: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<LiteralValue>,
        line_number: usize,
    ) -> Token {
        Self {
            token_type,
            lexeme,
            literal,
            line_number,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}

#[derive(Debug, Clone)]
pub enum TokenType {
    // Single Char Tokens
    LEFTPAREN,
    RIGHTPAREN,
    LEFTBRACE,
    RIGHTBRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One to Two char tokens
    BANG,
    BANGEQUAL,
    EQUAL,
    EQUALEQUAL,
    GREATER,
    GREATEREQUAL,
    LESS,
    LESSEQUAL,

    // Literals
    IDENTIFIER,
    STRINGLIT,
    NUMBER,

    // Keywords
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
