pub mod token;

use std::{collections::HashMap, num::ParseFloatError};

use crate::lexer::token::{Literal, Token, TokenType};

/// Lexer scans raw source code text to produce flat list of tokens.
pub struct Lexer {
    source_text: String,
    pub tokens: Vec<Token>,
    start: i32,
    /// Index of the character we're about to consume
    cursor: i32,
    line: i32,
    reserved_words: HashMap<String, TokenType>,
    /// Only access after calling lexer.advance()
    current_char: char,
}

impl Lexer {
    pub fn new(source_text: &str) -> Lexer {
        let mut lexer = Lexer {
            source_text: String::from(source_text),
            tokens: vec![],
            start: 0,
            cursor: 0,
            line: 0,
            reserved_words: HashMap::new(),
            // Initial value is inconsequential. Just to satisfy compiler
            current_char: ' ',
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
        return self.cursor >= source_length as i32;
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
        if (self.cursor + 1) >= self.source_text.len() as i32 {
            return '\0';
        }

        let next_char = char_at(self.source_text.clone(), self.cursor as usize);
        match next_char {
            Ok(char) => char,
            Err(err) => {
                panic!("{}", err);
            }
        }
    }

    /// Check the value of the character two positions ahead of the most recently consumed character.
    fn peek_next(&self) -> char {
        if self.at_end_of_source_text() {
            return '\0';
        }

        let next_next_char = char_at(self.source_text.clone(), self.cursor as usize);
        match next_next_char {
            Ok(char) => char,
            Err(err) => {
                panic!("{}", err);
            }
        }
    }
}

pub fn scan_tokens(mut lexer: Lexer) -> Lexer {
    while !(lexer.at_end_of_source_text()) {
        lexer.start = lexer.cursor;
        lexer = scan_token(lexer);
    }

    let token = Token::new(TokenType::EOF, "", Literal::Nil, lexer.line);
    lexer.tokens.push(token);
    return lexer;
}

fn scan_token(mut lexer: Lexer) -> Lexer {
    lexer = advance(lexer);
    let char = lexer.current_char;

    match char {
        // These lexemes are only one char
        '(' => lexer = add_token(lexer, TokenType::LEFT_PAREN),
        ')' => lexer = add_token(lexer, TokenType::RIGHT_PAREN),
        '{' => lexer = add_token(lexer, TokenType::LEFT_BRACE),
        '}' => lexer = add_token(lexer, TokenType::RIGHT_BRACE),
        ',' => lexer = add_token(lexer, TokenType::COMMA),
        '.' => lexer = add_token(lexer, TokenType::DOT),
        '-' => lexer = add_token(lexer, TokenType::MINUS),
        '+' => lexer = add_token(lexer, TokenType::PLUS),
        ':' => lexer = add_token(lexer, TokenType::SEMICOLON),
        '*' => lexer = add_token(lexer, TokenType::STAR),
        //
        '!' => {
            if lexer.next_char_has('=') {
                lexer.cursor += 1;
                lexer = add_token(lexer, TokenType::BANG_EQUAL);
            } else {
                lexer = add_token(lexer, TokenType::BANG);
            }
            return lexer;
        }
        '=' => {
            if lexer.next_char_has('=') {
                lexer.cursor += 1;
                lexer = add_token(lexer, TokenType::EQUAL_EQUAL);
            } else {
                lexer = add_token(lexer, TokenType::EQUAL);
            }
            return lexer;
        }
        '<' => {
            if lexer.next_char_has('=') {
                lexer.cursor += 1;
                lexer = add_token(lexer, TokenType::LESS_EQUAL);
            } else {
                lexer = add_token(lexer, TokenType::LESS);
            }
            return lexer;
        }
        '>' => {
            if lexer.next_char_has('=') {
                lexer.cursor += 1;
                lexer = add_token(lexer, TokenType::GREATER_EQUAL);
            } else {
                lexer = add_token(lexer, TokenType::GREATER);
            }
            return lexer;
        }
        '/' => {
            if lexer.next_char_has('/') {
                while lexer.peek() != '\n' && !lexer.at_end_of_source_text() {
                    lexer = advance(lexer);
                }
            } else {
                lexer = add_token(lexer, TokenType::SLASH);
            }
            return lexer;
        }
        // Ignore whitespace
        ' ' => return lexer,
        '\r' => return lexer,
        '\t' => return lexer,
        '\n' => {
            lexer.line += 1;
            return lexer;
        }
        // Literals
        '"' => {
            lexer = parse_string(lexer);
            return lexer;
        }
        _ => {
            if is_digit(char) {
                lexer = parse_number(lexer);
            } else if is_alpha(char) {
                lexer = parse_identifier(lexer);
            } else {
                // error(&lexer, lexer.line, "Unrecognised character");
                panic!("Unrecognised character"); // Todo: Use error handling at main.rs
            }
        }
    }
    return lexer;
}

/**
 *  cursor always points to the index of the next character to be consumed.
 * advance() reads the character at cursor, stores it in current_char,
 * then increments cursor to point past it.
 *
 * After advance() returns, current_char holds the character we just consumed,
 * and cursor points to the one after it. peek() is for looking ahead without consuming.
 */
fn advance(mut lexer: Lexer) -> Lexer {
    let current_char = char_at(lexer.source_text.clone(), lexer.cursor as usize);
    match current_char {
        Ok(char) => {
            lexer.cursor += 1;
            lexer.current_char = char;
        }
        Err(err) => {
            panic!("{}", err);
        }
    }

    return lexer;
}

fn parse_string(mut lexer: Lexer) -> Lexer {
    while lexer.peek() != '"' && !lexer.at_end_of_source_text() {
        if lexer.peek() == '\n' {
            lexer.line += 1;
        }

        lexer = advance(lexer);
    }

    if lexer.at_end_of_source_text() {
        crate::error(lexer.line, "Unterminated string");
    }

    lexer = advance(lexer);

    let value =
        &lexer.source_text[(lexer.start + 1) as usize..(lexer.cursor - 1) as usize].to_string(); //investigate borrowing logic here
    lexer = add_token_with_literal(lexer, TokenType::STRING, Literal::Str(String::from(value)));

    return lexer;
}

fn parse_number(mut lexer: Lexer) -> Lexer {
    while is_digit(lexer.peek()) {
        lexer = advance(lexer);
    }

    if lexer.peek() == '.' && is_digit(lexer.peek_next()) {
        lexer = advance(lexer);

        while is_digit(lexer.peek()) {
            lexer = advance(lexer);
        }
    }

    let mut number = String::new();
    lexer.source_text[lexer.start as usize..lexer.cursor as usize].clone_into(&mut number);

    let number_as_float: Result<f64, ParseFloatError> = number.parse();
    match number_as_float {
        Ok(num) => {
            lexer = add_token_with_literal(lexer, TokenType::NUMBER, Literal::Number(num));
        }
        Err(err) => panic!("Unexpected error while parsing number: {}", err),
    }

    return lexer;
}

fn parse_identifier(mut lexer: Lexer) -> Lexer {
    while is_alphanumeric(lexer.peek()) {
        lexer = advance(lexer);
    }

    let text = &lexer.source_text[lexer.start as usize..lexer.cursor as usize];
    let reserved_words = lexer.reserved_words.clone();
    let token_type = reserved_words.get(text);

    match token_type {
        Some(token_type) => {
            lexer = add_token(lexer, token_type.clone());
        }
        None => lexer = add_token(lexer, TokenType::IDENTIFIER),
    }

    return lexer;
}

fn add_token(lexer: Lexer, token_type: TokenType) -> Lexer {
    return add_token_with_literal(lexer, token_type, Literal::Nil);
}

fn add_token_with_literal(mut lexer: Lexer, token_type: TokenType, literal: Literal) -> Lexer {
    let text = &lexer.source_text[lexer.start as usize..lexer.cursor as usize];

    lexer
        .tokens
        .push(Token::new(token_type, text, literal, lexer.line));

    return lexer;
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
