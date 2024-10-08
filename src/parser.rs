use crate::expr::*;
use crate::statement;
use crate::statement::Statement;
use crate::LiteralValue;
use crate::LoxErr;
use crate::Token;
use crate::TokenType;
use core::panic;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements: Vec<Statement> = Vec::new();
        let mut errors: Vec<LoxErr> = Vec::new();
        while !self.is_at_end() {
            let statement = self.statement();
            match statement {
                Ok(stmt) => statements.push(stmt),
                Err(e) => {
                    println!("{}", e);
                    errors.push(e)
                }
            }
        }
        statements
    }

    fn statement(&mut self) -> Result<Statement, LoxErr> {
        let variac = vec![TokenType::PRINT];
        if self.match_token(&variac) {
            return Ok(self.print_statement()?);
        }
        let variac = vec![TokenType::VAR];
        if self.match_token(&variac) {
            return Ok(self.assignment_statement()?);
        }
        let variac = vec![TokenType::LEFTBRACE];
        if self.match_token(&variac) {
            return Ok(self.block_statement()?);
        }
        let variac = vec![TokenType::ASSERT];
        if self.match_token(&variac) {
            return Ok(self.assert_statement()?);
        }
        let variac = vec![TokenType::IF];
        if self.match_token(&variac) {
            return Ok(self.if_statement()?);
        }
        let variac = vec![TokenType::WHILE];
        if self.match_token(&variac) {
            return Ok(self.while_statement()?);
        }
        let variac = vec![TokenType::FOR];
        if self.match_token(&variac) {
            return Ok(self.for_loop()?);
        }
        Ok(self.expression_statement()?)
    }

    fn for_loop(&mut self) -> Result<Statement, LoxErr> {
        self.consume(TokenType::LEFTPAREN, "Expected '(' after for")?;
        Ok(self.for_loop_init()?)
    }

    fn for_loop_init(&mut self) -> Result<Statement, LoxErr> {
        let mut init;
        let variac = vec![TokenType::SEMICOLON];
        if self.match_token(&variac) {
            init = None;
        }
        let variac = vec![TokenType::VAR];
        if self.match_token(&variac) {
            init = Some(self.declaration()?);
        } else {
            let expr = Some(self.expression_statement()?);
            init = expr;
        }

        println!("VAR {:?}", init);

        let condition;
        if !self.check(&TokenType::SEMICOLON) {
            condition = Some(self.expression()?);
        } else {
            condition = None
        }

        println!("Condition {:?}", condition);

        self.consume(TokenType::SEMICOLON, "Expected ; after condition")?;

        let increment;
        if !self.check(&TokenType::RIGHTPAREN) {
            increment = Some(self.expression()?);
        } else {
            increment = None
        }
        self.consume(TokenType::RIGHTPAREN, "Expected ) after condition")?;

        println!("Increment {:?}", increment);

        let mut body = self.statement()?;

        if increment.is_some() {
            body = Statement::Block {
                statements: vec![
                    *Box::new(body),
                    Statement::Expression {
                        expression: *Box::new(increment.unwrap()),
                    },
                ],
            };
        }

        let cond;
        match condition {
            None => {
                cond = Expr::LiteralExpr {
                    literal: LiteralValue::True,
                }
            }
            Some(c) => cond = c,
        }

        body = Statement::While {
            predicate: cond,
            happy_path: Box::new(body),
        };

        if init.is_some() {
            body = Statement::Block {
                statements: vec![*Box::new(init.unwrap()), *Box::new(body)],
            };
        }

        Ok(body)
    }

    fn while_statement(&mut self) -> Result<Statement, LoxErr> {
        self.consume(TokenType::LEFTPAREN, "Expected '(' after while")?;
        let expr = self.expression()?;
        self.consume(TokenType::RIGHTPAREN, "Expected ')' after condition")?;
        let happy_path = self.statement()?;
        Ok(Statement::While {
            predicate: expr,
            happy_path: Box::new(happy_path),
        })
    }

    fn if_statement(&mut self) -> Result<Statement, LoxErr> {
        self.consume(TokenType::LEFTPAREN, "Expected '(' after if")?;
        let expr = self.expression()?;
        self.consume(TokenType::RIGHTPAREN, "Expected ')' after if")?;
        let happy_path = self.statement()?;
        let els = if self.match_token(&vec![TokenType::ELSE]) {
            let stm = self.statement()?;
            Some(Box::new(stm))
        } else {
            None
        };
        Ok(Statement::If {
            conditional: expr,
            happy_path: Box::new(happy_path),
            sad_path: els,
        })
    }

    fn block_statement(&mut self) -> Result<Statement, LoxErr> {
        let mut statements = vec![];
        while !self.check(&TokenType::RIGHTBRACE) && !self.is_at_end() {
            let decl = self.declaration()?;
            statements.push(decl);
        }
        self.consume(TokenType::RIGHTBRACE, "Expected } after {")?;
        Ok(Statement::Block { statements })
    }

    fn print_statement(&mut self) -> Result<Statement, LoxErr> {
        let expr = self.expression()?;
        self.consume(TokenType::SEMICOLON, "Expected ; after statement")?;
        Ok(Statement::Print { expression: expr })
    }

    fn assert_statement(&mut self) -> Result<Statement, LoxErr> {
        let val1 = self.expression()?;
        self.consume(
            TokenType::SEMICOLON,
            "Expected ';' after variable declaration",
        )?;
        Ok(Statement::Assert { expression_a: val1 })
    }

    fn declaration(&mut self) -> Result<Statement, LoxErr> {
        if self.match_token(&vec![TokenType::VAR]) {
            match self.assignment_statement() {
                Ok(stmt) => Ok(stmt),
                Err(msg) => {
                    // self.synchronize();
                    Err(msg)
                }
            }
        } else {
            self.statement()
        }
    }

    fn assignment_statement(&mut self) -> Result<Statement, LoxErr> {
        let token = self.consume(TokenType::IDENTIFIER, "Expected Variable Name")?;

        if self.match_token(&vec![TokenType::EQUAL]) {
            let initializer = self.expression()?;
            self.consume(
                TokenType::SEMICOLON,
                "Expected ';' after variable declaration",
            )?;

            return Ok(Statement::Var {
                indentifier: token.lexeme,
                expression: initializer,
            });
        }
        panic!("Cannot reach here for now")
    }

    fn expression_statement(&mut self) -> Result<Statement, LoxErr> {
        let expr = self.expression()?;
        self.consume(TokenType::SEMICOLON, "Expected ; after statement")?;
        Ok(Statement::Expression { expression: expr })
    }

    fn expression(&mut self) -> Result<Expr, LoxErr> {
        Ok(self.assignment()?)
    }

    fn or(&mut self) -> Result<Expr, LoxErr> {
        let mut expr = self.and()?;
        while self.match_token(&vec![TokenType::OR]) {
            let op = self.previous();
            let right = self.and()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, LoxErr> {
        let mut expr = self.equality()?;
        while self.match_token(&vec![TokenType::AND]) {
            let op = self.previous();
            let right = self.and()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn assignment(&mut self) -> Result<Expr, LoxErr> {
        let expr = self.or()?;
        let variac = vec![TokenType::EQUAL];
        if self.match_token(&variac) {
            let _ = self.previous();
            let value = self.assignment()?;

            match expr {
                Expr::Var { identifier } => Ok(Expr::Assignment {
                    name: identifier,
                    value: Box::new(value),
                }),
                _ => Err("Invalid assignment target".into()),
            }
        } else {
            Ok(expr)
        }
    }

    fn equality(&mut self) -> Result<Expr, LoxErr> {
        let mut expr = self.comparision()?;
        let variac = vec![TokenType::BANGEQUAL, TokenType::EQUALEQUAL];
        while self.match_token(&variac) {
            let operator = self.previous();
            let right = self.comparision()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: operator,
                right: Box::new(right),
            }
        }
        Ok(expr)
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

    fn comparision(&mut self) -> Result<Expr, LoxErr> {
        let mut expr = self.term()?;
        let variac = vec![
            TokenType::GREATER,
            TokenType::GREATEREQUAL,
            TokenType::LESS,
            TokenType::LESSEQUAL,
        ];
        while self.match_token(&variac) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: operator,
                right: Box::new(right),
            }
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, LoxErr> {
        let mut expr = self.factor()?;
        let variac = vec![TokenType::MINUS, TokenType::PLUS];
        while self.match_token(&variac) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: operator,
                right: Box::new(right),
            }
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, LoxErr> {
        if self.match_token(&vec![TokenType::BANG, TokenType::MINUS]) {
            let op = self.previous();
            let rhs = self.unary()?;
            return Ok(Expr::Unary {
                operator: op,
                right: Box::new(rhs),
            });
        } else {
            self.primary()
        }
    }

    fn factor(&mut self) -> Result<Expr, LoxErr> {
        let mut expr = self.unary()?;
        let variac = vec![TokenType::SLASH, TokenType::STAR];
        while self.match_token(&variac) {
            let operator = self.previous();
            let right = self.unary()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                op: operator,
                right: Box::new(right),
            }
        }
        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expr, LoxErr> {
        if self.match_token(&vec![TokenType::FALSE]) {
            return Ok(Expr::LiteralExpr {
                literal: LiteralValue::False,
            });
        }
        if self.match_token(&vec![TokenType::TRUE]) {
            return Ok(Expr::LiteralExpr {
                literal: LiteralValue::True,
            });
        }
        if self.match_token(&vec![TokenType::NIL]) {
            return Ok(Expr::LiteralExpr {
                literal: LiteralValue::Nil,
            });
        }
        if self.match_token(&vec![TokenType::NUMBER, TokenType::STRINGLIT]) {
            return Ok(Expr::LiteralExpr {
                literal: self.previous().literal.unwrap(),
            });
        }
        if self.match_token(&vec![TokenType::LEFTPAREN]) {
            let expr = self.expression()?;
            self.consume(TokenType::RIGHTPAREN, "Expect ')' after expression '('")?;
            return Ok(Expr::Grouping {
                expression: Box::new(expr),
            });
        }
        if self.match_token(&vec![TokenType::IDENTIFIER]) {
            let identifier = self.tokens[self.current - 1].clone();
            return Ok(Expr::Var {
                identifier: identifier.lexeme,
            });
        }
        panic!("Should never reach this point")
    }

    fn consume(&mut self, ttype: TokenType, message: &str) -> Result<Token, LoxErr> {
        if self.check(&ttype) {
            self.advance();
            return Ok(self.previous());
        }
        Err(format!("{message}").into())
    }

    fn previous(&mut self) -> Token {
        match self.tokens.get(self.current - 1) {
            Some(t) => t.clone(),
            None => panic!("Undefined token"),
        }
    }
    #[allow(dead_code)]
    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().token_type == TokenType::EOF {
                return;
            }
        }
        match self.peek().token_type {
            TokenType::CLASS
            | TokenType::FUN
            | TokenType::VAR
            | TokenType::FOR
            | TokenType::IF
            | TokenType::WHILE
            | TokenType::PRINT
            | TokenType::RETURN => return,
            _ => (),
        }
        self.advance();
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
        let parsed_expression = parser.expression().unwrap();
        let parsed_expression = parsed_expression.to_string();
        assert_eq!("(+ 1 5)", parsed_expression);
    }

    #[test]
    fn test_cmp() {
        let source = "1 + 2 == 5 + 7";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let parsed_expression = parser.expression().unwrap().to_string();
        assert_eq!("(== (+ 1 2) (+ 5 7))", parsed_expression);
    }
    #[test]
    fn test_cmp_paren() {
        let source = "1 + 2 == (5 + 7)";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let parsed_expression = parser.expression().unwrap().to_string();
        assert_eq!("(== (+ 1 2) (group (+ 5 7)))", parsed_expression);
    }
}
