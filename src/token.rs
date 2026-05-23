#[derive(Debug)]
pub enum Literal {
    Number(f64),
    Str(String),
    Bool(bool),
    Nil,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    // Single-character tokens
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,
    // One or two character tokens
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,
    // Literals
    IDENTIFIER,
    STRING,
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

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Literal,
    pub line: i32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: &str, literal: Literal, line: i32) -> Token {
        Token {
            token_type,
            lexeme: String::from(lexeme),
            literal,
            line,
        }
    }

    fn to_string(&self) -> String {
        return format!(
            "{:?} {} {:?}",
            self.token_type,
            self.lexeme.as_str(),
            self.literal,
        );
    }
}
