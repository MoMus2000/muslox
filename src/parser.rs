use core::panic;
use std::fmt::Binary;

use crate::expr::*;
use crate::LiteralValue;
use crate::LoxErr;
use crate::Token;
use crate::TokenType;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        println!("Inside equality");
        let mut expr = self.comparision();
        let variac = vec![TokenType::BANGEQUAL, TokenType::EQUALEQUAL];
        while self.match_token(&variac) {
            let operator = self.previous();
            let right = self.comparision();
            expr = Expr::Binary {
                left: Box::new(expr),
                op: operator,
                right: Box::new(right),
            }
        }
        expr
    }

    fn match_token(&mut self, variatic: &Vec<TokenType>) -> bool {
        for token in variatic {
            if self.check(token) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn peek(&mut self) -> Token {
        match self.tokens.get(self.current) {
            Some(t) => t.clone(),
            None => panic!("Undefined token"),
        }
    }

    fn is_at_end(&mut self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn check(&mut self, ttype: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        return self.peek().token_type == *ttype;
    }

    fn comparision(&mut self) -> Expr {
        println!("Inside comparision");
        let mut expr = self.term();
        let variac = vec![
            TokenType::GREATER,
            TokenType::GREATEREQUAL,
            TokenType::LESS,
            TokenType::LESSEQUAL,
        ];
        while self.match_token(&variac) {
            let operator = self.previous();
            let right = self.term();
            expr = Expr::Binary {
                left: Box::new(expr),
                op: operator,
                right: Box::new(right),
            }
        }
        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        println!("Inside term");
        let variac = vec![TokenType::MINUS, TokenType::PLUS];
        while self.match_token(&variac) {
            let operator = self.previous();
            let right = self.factor();
            expr = Expr::Binary {
                left: Box::new(expr),
                op: operator,
                right: Box::new(right),
            }
        }
        println!("Expression inside term {:?}", expr);
        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_token(&vec![TokenType::BANG, TokenType::MINUS]) {
            let op = self.previous();
            let rhs = self.unary();
            Expr::Unary {
                operator: op,
                right: Box::new(rhs),
            }
        } else {
            self.primary()
        }
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        let variac = vec![TokenType::SLASH, TokenType::STAR];
        while self.match_token(&variac) {
            let operator = self.previous();
            let right = self.unary();

            expr = Expr::Binary {
                left: Box::new(expr),
                op: operator,
                right: Box::new(right),
            }
        }
        expr
    }

    fn primary(&mut self) -> Expr {
        if self.match_token(&vec![TokenType::FALSE]) {
            return Expr::LiteralExpr {
                literal: LiteralValue::False,
            };
        }
        if self.match_token(&vec![TokenType::TRUE]) {
            return Expr::LiteralExpr {
                literal: LiteralValue::True,
            };
        }
        if self.match_token(&vec![TokenType::NIL]) {
            return Expr::LiteralExpr {
                literal: LiteralValue::Nil,
            };
        }
        if self.match_token(&vec![TokenType::NUMBER, TokenType::STRINGLIT]) {
            return Expr::LiteralExpr {
                literal: self.previous().literal.unwrap(),
            };
        }
        if self.match_token(&vec![TokenType::LEFTPAREN]) {
            let expr = self.expression();
            self.consume(TokenType::RIGHTPAREN, "Expect ')' after expression ')'");
            return Expr::Grouping {
                expression: Box::new(expr),
            };
        }
        panic!("Should never reach this point")
    }

    fn consume(&mut self, ttype: TokenType, message: &str) {
        let token = self.peek();
        if token.token_type == ttype {
            self.advance();
        }
        panic!("{}", message)
    }

    fn previous(&mut self) -> Token {
        match self.tokens.get(self.current - 1) {
            Some(t) => t.clone(),
            None => panic!("Undefined token"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::LiteralValue;
    use crate::Scanner;
    use crate::Token;
    use crate::TokenType;

    use super::Parser;

    #[test]
    fn test_addition() {
        let tokens = vec![
            Token {
                token_type: TokenType::NUMBER,
                lexeme: "1".to_string(),
                literal: Some(LiteralValue::FValue(1.0)),
                line_number: 0,
            },
            Token {
                token_type: TokenType::PLUS,
                lexeme: "+".to_string(),
                literal: None,
                line_number: 0,
            },
            Token {
                token_type: TokenType::NUMBER,
                lexeme: "5".to_string(),
                literal: Some(LiteralValue::FValue(5.0)),
                line_number: 0,
            },
            Token {
                token_type: TokenType::SEMICOLON,
                lexeme: ";".to_string(),
                literal: None,
                line_number: 0,
            },
        ];
        let mut parser = Parser::new(tokens);
        let parsed_expression = parser.expression();
        let parsed_expression = parsed_expression.to_string();
        assert_eq!("(+ (1 5))", parsed_expression);
    }

    #[test]
    fn test_cmp() {
        let source = "1 + 2 == 5 + 7";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let parsed_expression = parser.expression().to_string();
        assert_eq!("(== (+ (1 2)) (+ (5 7)))", parsed_expression);
    }
}
