use crate::{
    ast::expressions::Expr,
    token::{Literal, Token, TokenType},
};

/// Lox Expression Grammar
/// expression → equality ;
/// equality → comparison ( ( "!=" | "==" ) comparison )* ;
/// comparison → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
/// term → factor ( ( "-" | "+" ) factor )* ;
/// factor → unary ( ( "/" | "*" ) unary )* ;
/// unary → ( "!" | "-" ) unary
/// | primary ;
/// primary → NUMBER | STRING | "true" | "false" | "nil"
/// | "(" expression ")" ;
pub struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
}

/// Top-down recursive descent parser. Each grammar rule is implemented as a
/// method on the `Parser` struct.
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens,
            cursor: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        return self.parse_expression();
    }

    fn parse_expression(&mut self) -> Result<Expr, ParseError> {
        return self.parse_equality();
    }

    fn parse_equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_comparison()?;

        while self.match_any(&[TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator = self.previous().clone();
            let right = self.parse_comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                right: Box::new(right),
                operator: operator.clone(),
            }
        }

        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_term()?;

        while self.match_any(&[
            TokenType::GREATER,
            TokenType::GREATER_EQUAL,
            TokenType::LESS,
            TokenType::LESS_EQUAL,
        ]) {
            let operator = self.previous().clone();
            let right = self.parse_term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                right: Box::new(right),
                operator,
            };
        }

        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_factor()?;

        while self.match_any(&[TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous().clone();
            let right = self.parse_factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                right: Box::new(right),
                operator,
            }
        }

        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_unary()?;

        while self.match_any(&[TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous().clone();
            let right = self.parse_unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                right: Box::new(right),
                operator,
            }
        }

        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_any(&[TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous().clone();
            let right = self.parse_unary()?;

            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }

        return self.parse_primary();
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        if self.match_any(&[TokenType::FALSE]) {
            return Ok(Expr::Literal {
                value: Literal::Bool(false),
            });
        }

        if self.match_any(&[TokenType::TRUE]) {
            return Ok(Expr::Literal {
                value: Literal::Bool(true),
            });
        }

        if self.match_any(&[TokenType::NIL]) {
            return Ok(Expr::Literal {
                value: Literal::Nil,
            });
        }

        if self.match_any(&[TokenType::STRING]) {
            return Ok(Expr::Literal {
                value: self.previous().literal.clone(),
            });
        }
        if self.match_any(&[TokenType::NUMBER]) {
            return Ok(Expr::Literal {
                value: self.previous().literal.clone(),
            });
        }

        if self.match_any(&[TokenType::LEFT_PAREN]) {
            let expr = self.parse_expression()?;
            self.consume(TokenType::RIGHT_PAREN, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping {
                expression: Box::new(expr),
            });
        }

        // Hit a token that can't start an expression
        return Err(ParseError {
            token: self.peek().clone(),
            message: String::from("Expected an expression"),
        });
    }

    fn is_at_end(&self) -> bool {
        match self.tokens[self.cursor].token_type {
            TokenType::EOF => true,
            _ => false,
        }
    }

    fn peek(&self) -> &Token {
        return &self.tokens[self.cursor];
    }

    fn previous(&self) -> &Token {
        return &self.tokens[self.cursor - 1];
    }

    fn advance(&mut self) -> &Token {
        let current_token = &self.tokens[self.cursor];
        match current_token.token_type {
            TokenType::EOF => return current_token,
            _ => {
                self.cursor += 1;
                return current_token;
            }
        }
    }

    fn consume(
        &mut self,
        expected_type: TokenType,
        error_message: &str,
    ) -> Result<&Token, ParseError> {
        if self.current_is(&expected_type) {
            return Ok(self.advance());
        } else {
            Err(ParseError {
                token: self.peek().clone(),
                message: String::from(error_message),
            })
        }
    }

    /// After we encounter a parser error, we discard tokens until we can find a
    /// new 'safe/stable' location to begin parsing from again.
    fn synchronise(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::SEMICOLON {
                return;
            };

            match self.peek().token_type {
                TokenType::CLASS
                | TokenType::FUN
                | TokenType::VAR
                | TokenType::FOR
                | TokenType::IF
                | TokenType::WHILE
                | TokenType::PRINT
                | TokenType::RETURN => return,
                _ => {
                    self.advance();
                }
            }
        }

        return;
    }

    /// Returns true if current token matches expected type.
    /// Always returns false if parser at EOF/.
    fn current_is(&self, expected: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        let current_tokens_type = &self.tokens[self.cursor].token_type;
        current_tokens_type == expected
    }

    /// Checks if current token matches any of the given types. If so, consumes
    /// token and advances cursor. Otherwise returns false and leaves cursor pos
    /// unchanged.
    fn match_any(&mut self, types: &[TokenType]) -> bool {
        for token_type in types.iter() {
            if self.current_is(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }
}

pub struct ParseError {
    pub token: Token,
    pub message: String,
}
