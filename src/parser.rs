use crate::{
    ast::expressions::Expr,
    token::{Token, TokenType},
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
struct Parser {
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

    // fn parse_expression() -> Expr {
    //     return parse_equality();
    // }

    // fn parse_equality() -> Expr {
    //     let expr = parse_comparison();
    // }

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

struct ParseError {
    expr: Expr,
    message: String,
}
