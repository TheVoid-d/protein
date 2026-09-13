use crate::frontend::lexer::token::{Token, TokenType};
use std::collections::HashMap;
use std::process::exit;
use std::sync::LazyLock;

/**
The class, which is used for transforming input
string-like source code into a Vector of tokens,
which then will be parsed in a parser
*/
#[derive(Debug)]
pub struct Lexer {}
static KEYWORDS: LazyLock<HashMap<&'static str, TokenType>> =
    LazyLock::new(|| HashMap::from([("let", TokenType::Let), ("const", TokenType::Const)]));
impl Lexer {
    /**
    The main function for tokenizing source code
    */
    pub fn tokenize(&self, src: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut source = src.chars().peekable();

        println!("source: {source:?}");
        while let Some(&current) = source.peek() {
            // println!("Current token is {current:?}");
            match current {
                '(' => {
                    source.next();
                    tokens.push(Token::new(current.to_string(), TokenType::OpenParen))
                },
                ')' => {
                    source.next();
                    tokens.push(Token::new(current.to_string(), TokenType::CloseParen))
                },

                '{' => {
                    source.next();
                    tokens.push(Token::new(current.to_string(), TokenType::OpenBracket))
                },
                '}' => {
                    source.next();
                    tokens.push(Token::new(current.to_string(), TokenType::CloseBracket))
                },

                '-' | '+' | '*' | '/' | '%' | '^' => {
                    source.next();
                    tokens.push(Token::new(current.to_string(), TokenType::BinaryOperation))
                },

                ';' => {
                    source.next();
                    tokens.push(Token::new(current.to_string(), TokenType::Semicolon))
                },
                ':' => {
                    source.next();
                    tokens.push(Token::new(current.to_string(), TokenType::Colon))
                },
                '=' => {
                    source.next();
                    tokens.push(Token::new(current.to_string(), TokenType::Equals))
                },

                _ => {
                    if current.is_ascii_digit() {
                        let mut num: String = String::new();

                        while let Some(&c) = source.peek() {
                            if c.is_ascii_digit() {
                                num.push(source.next().unwrap());
                            } else {
                                break;
                            }
                        }
                        tokens.push(Token::new(num, TokenType::Number))
                    } else if current.is_alphabetic() {
                        let mut identifier = String::new();

                        while let Some(&c) = source.peek() {
                            if c.is_alphabetic() {
                                identifier.push(c);
                            } else {
                                break;
                            }
                            source.next();
                        }
                        tokens.push(Token::new(
                            identifier.clone(),
                            self.get_token_type(&identifier),
                        ));
                    } else if self.is_skippable(current) {
                        source.next();
                    } else {
                        println!("Unrecognized character found in source: {}", current);
                        exit(-1)
                    }
                }
            }
        }
        tokens.push(Token::eof());
        tokens
    }
    fn get_token_type(&self, str: &str) -> TokenType {
        KEYWORDS
            .get(str)
            .cloned()
            .unwrap_or_else(|| TokenType::Identifier)
    }
    fn is_skippable(&self, c: char) -> bool {
        match c {
            ' ' | '\n' | '\t' | '\r' => true,
            _ => false,
        }
    }

    pub fn new() -> Self {
        Self {}
    }
}
