use std::{collections::HashMap, num::ParseFloatError};

use crate::token::{Literal, Token, TokenType};

/// Lexer scans raw source code text to produce flat list of tokens.
pub struct Lexer {
    source_text: String,
    pub tokens: Vec<Token>,
    start: usize,
    /// Index of the character we're about to consume
    cursor: usize,
    line: i32,
    reserved_words: HashMap<String, TokenType>,
    pub errors: Vec<LexError>,
}

impl Lexer {
    pub fn new(source_text: &str) -> Lexer {
        let mut lexer = Lexer {
            source_text: String::from(source_text),
            tokens: vec![],
            start: 0,
            cursor: 0,
            line: 1,
            reserved_words: HashMap::new(),
            // Initial value is inconsequential. Just to satisfy compiler
            errors: vec![],
        };

        lexer
            .reserved_words
            .insert(String::from("and"), TokenType::AND);
        lexer
            .reserved_words
            .insert(String::from("class"), TokenType::CLASS);
        lexer
            .reserved_words
            .insert(String::from("else"), TokenType::ELSE);
        lexer
            .reserved_words
            .insert(String::from("false"), TokenType::FALSE);
        lexer
            .reserved_words
            .insert(String::from("for"), TokenType::FOR);
        lexer
            .reserved_words
            .insert(String::from("fun"), TokenType::FUN);
        lexer
            .reserved_words
            .insert(String::from("if"), TokenType::IF);
        lexer
            .reserved_words
            .insert(String::from("nil"), TokenType::NIL);
        lexer
            .reserved_words
            .insert(String::from("or"), TokenType::OR);
        lexer
            .reserved_words
            .insert(String::from("print"), TokenType::PRINT);
        lexer
            .reserved_words
            .insert(String::from("return"), TokenType::RETURN);
        lexer
            .reserved_words
            .insert(String::from("super"), TokenType::SUPER);
        lexer
            .reserved_words
            .insert(String::from("this"), TokenType::THIS);
        lexer
            .reserved_words
            .insert(String::from("true"), TokenType::TRUE);
        lexer
            .reserved_words
            .insert(String::from("var"), TokenType::VAR);
        lexer
            .reserved_words
            .insert(String::from("while"), TokenType::WHILE);

        return lexer;
    }

    fn at_end_of_source_text(&self) -> bool {
        let source_length = self.source_text.len();
        return self.cursor >= source_length;
    }

    /// Check whether the next character in the source text is equal to some expected character
    fn next_char_has(&self, expected: char) -> bool {
        if self.at_end_of_source_text() {
            return false;
        };

        let next_char = char_at(self.source_text.clone(), self.cursor as usize);
        match next_char {
            Ok(char) => {
                if char == expected {
                    return true;
                } else {
                    return false;
                }
            }
            Err(_err) => {
                return false;
            }
        }
    }

    /// Check the value of the next character in the source text without consuming the character.
    fn peek(&self) -> char {
        if self.at_end_of_source_text() {
            return '\0';
        }

        let next_char = char_at(self.source_text.clone(), self.cursor);
        match next_char {
            Ok(char) => char,
            Err(err) => {
                panic!("{}", err);
            }
        }
    }

    /// Check the value of the character two positions ahead of the most recently consumed character.
    fn peek_next(&self) -> char {
        if self.cursor + 1 >= self.source_text.len() {
            return '\0';
        }

        let next_next_char = char_at(self.source_text.clone(), self.cursor + 1);
        match next_next_char {
            Ok(char) => char,
            Err(err) => {
                panic!("{}", err);
            }
        }
    }

    pub fn scan_tokens(&mut self) {
        while !(self.at_end_of_source_text()) {
            self.start = self.cursor;
            self.scan_token();
        }

        let token = Token::new(TokenType::EOF, "", Literal::Nil, self.line);
        self.tokens.push(token);
    }

    fn scan_token(&mut self) {
        let char = self.advance();

        match char {
            // These lexemes are only one char
            '(' => self.add_token(TokenType::LEFT_PAREN),
            ')' => self.add_token(TokenType::RIGHT_PAREN),
            '{' => self.add_token(TokenType::LEFT_BRACE),
            '}' => self.add_token(TokenType::RIGHT_BRACE),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            ':' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),
            //
            '!' => {
                if self.next_char_has('=') {
                    self.cursor += 1;
                    self.add_token(TokenType::BANG_EQUAL);
                } else {
                    self.add_token(TokenType::BANG);
                }
            }
            '=' => {
                if self.next_char_has('=') {
                    self.cursor += 1;
                    self.add_token(TokenType::EQUAL_EQUAL);
                } else {
                    self.add_token(TokenType::EQUAL);
                }
            }
            '<' => {
                if self.next_char_has('=') {
                    self.cursor += 1;
                    self.add_token(TokenType::LESS_EQUAL);
                } else {
                    self.add_token(TokenType::LESS);
                }
            }
            '>' => {
                if self.next_char_has('=') {
                    self.cursor += 1;
                    self.add_token(TokenType::GREATER_EQUAL);
                } else {
                    self.add_token(TokenType::GREATER);
                }
            }
            '/' => {
                if self.next_char_has('/') {
                    while self.peek() != '\n' && !self.at_end_of_source_text() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::SLASH);
                }
            }
            // Ignore whitespace
            ' ' | '\r' | '\t' => return,
            '\n' => {
                self.line += 1;
            }
            // Literals
            '"' => {
                self.parse_string();
            }
            _ => {
                if is_digit(char) {
                    self.parse_number();
                } else if is_alpha(char) {
                    self.parse_identifier();
                } else {
                    self.errors.push(LexError {
                        line: self.line,
                        message: format!("Unrecognised character: {}", char),
                    });
                }
            }
        }
    }

    /**
     *  cursor always points to the index of the next character to be consumed.
     * advance() reads the character at cursor, stores it in current_char,
     * then increments cursor to point past it.
     *
     * After advance() returns, current_char holds the character we just consumed,
     * and cursor points to the one after it. peek() is for looking ahead without consuming.
     */
    fn advance(&mut self) -> char {
        let current_char = char_at(self.source_text.clone(), self.cursor);
        match current_char {
            Ok(char) => {
                self.cursor += 1;
                return char;
            }
            Err(err) => {
                panic!("{}", err);
            }
        }
    }

    fn parse_string(&mut self) {
        while self.peek() != '"' && !self.at_end_of_source_text() {
            if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.at_end_of_source_text() {
            self.errors.push(LexError {
                line: self.line,
                message: String::from("Unterminated string"),
            });
        }

        self.advance();

        let value =
            &self.source_text[(self.start + 1) as usize..(self.cursor - 1) as usize].to_string(); //investigate borrowing logic here
        self.add_token_with_literal(TokenType::STRING, Literal::Str(String::from(value)));
    }

    fn parse_number(&mut self) {
        while is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && is_digit(self.peek_next()) {
            self.advance();

            while is_digit(self.peek()) {
                self.advance();
            }
        }

        let mut number = String::new();
        self.source_text[self.start as usize..self.cursor as usize].clone_into(&mut number);

        let number_as_float: Result<f64, ParseFloatError> = number.parse();
        match number_as_float {
            Ok(num) => {
                self.add_token_with_literal(TokenType::NUMBER, Literal::Number(num));
            }
            Err(parse_err) => {
                self.errors.push(LexError {
                    line: self.line,
                    message: parse_err.to_string(),
                });
            }
        }
    }

    fn parse_identifier(&mut self) {
        while is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text = &self.source_text[self.start..self.cursor];
        let reserved_words = self.reserved_words.clone();
        let token_type = reserved_words.get(text);

        match token_type {
            Some(token_type) => {
                self.add_token(token_type.clone());
            }
            None => self.add_token(TokenType::IDENTIFIER),
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        return self.add_token_with_literal(token_type, Literal::Nil);
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Literal) {
        let text = &self.source_text[self.start..self.cursor];

        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
    }
}

fn char_at(text: String, index: usize) -> Result<char, String> {
    let mut string_iterator = text.chars();
    let char_at = string_iterator.nth(index);

    match char_at {
        Some(val) => Ok(val),
        None => {
            let err_msg = format!(
                "Failed to get character at index {} for string {}.",
                index, text
            );
            return Err(err_msg);
        }
    }
}

fn is_digit(c: char) -> bool {
    return c >= '0' && c <= '9';
}

fn is_alpha(c: char) -> bool {
    return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
}

fn is_alphanumeric(c: char) -> bool {
    return is_alpha(c) || is_digit(c);
}

pub struct LexError {
    pub line: i32,
    pub message: String,
}
