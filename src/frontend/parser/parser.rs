use crate::frontend::lexer::lexer::Lexer;
use crate::frontend::lexer::token::{Token, TokenType};
use crate::frontend::parser::ast::Expression::BinaryExpression;
use crate::frontend::parser::ast::{Expression, Program, Statement};
use std::process::exit;

/// class which is used for parsing tokens into a `Abstract Syntax Tree (AST)`
#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
    tokens: Vec<Token>,
    cursor: usize,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            lexer: Lexer::new(),
            tokens: Vec::new(),
            cursor: 0,
        }
    }
    pub fn produce_ast(&mut self, source_code: String) -> Program {
        self.tokens = self.lexer.tokenize(&source_code);
        self.cursor = 0;

        println!("\n ====== \n Tokens: {:?} \n ====== \n", self.tokens);

        let mut program = Program::new();

        while !self.is_eof() {
            program.add_element(self.parse_statement());
        }
        program
    }
    fn parse_statement(&mut self) -> Statement {
        println!("Parsing statement:{:?}", self.at());
        match self.at().token_type() {
            TokenType::Semicolon => {
                self.eat();
                Statement::Semicolon
            }
            TokenType::Let | TokenType::Const => {
                todo!()
            }

            _ => {
                let expression = self.parse_expression();
                if self.at().token_type().eq(&TokenType::Semicolon) {
                    self.eat(); // Past semicolon
                }
                Statement::Expr(expression)
            }
        }
    }

    fn parse_expression(&mut self) -> Expression {
        self.parse_additive_expression()
    }
    fn parse_additive_expression(&mut self) -> Expression {
        let mut left = self.parse_multiplicative_expression();

        while (self.at().value().eq("+") || self.at().value().eq("-")) {
            let operator = self.eat().value().to_string();
            let right = self.parse_expression();

            left = BinaryExpression {
                left: Box::from(left),
                operator,
                right: Box::from(right),
            };
        }
        left
    }
    fn parse_multiplicative_expression(&mut self) -> Expression {
        let mut left = self.parse_primary_expression();

        while (self.at().value().eq("*") || self.at().value().eq("/") || self.at().value().eq("^"))
        {
            let operator = self.eat().value().to_string();
            let right = self.parse_primary_expression();

            left = BinaryExpression {
                left: Box::from(left),
                operator,
                right: Box::from(right),
            };
        }
        left
    }
    fn parse_primary_expression(&mut self) -> Expression {
        match self.at().token_type() {
            TokenType::Identifier => Expression::Identifier(self.eat().value().to_string()),
            TokenType::Number => Expression::Number(self.eat().value().parse().unwrap()),
            TokenType::OpenParen => {
                self.eat();
                let expression = self.parse_expression();
                self.expect(TokenType::CloseParen, "Expected closing parenthesis");
                expression
            }
            _ => {
                self.error("Unexpected token during parsing");
                self.eat();
                Expression::Identifier("NULL".to_string())
            }
        }
    }

    fn expect(&mut self, token_type: TokenType, message: &str) -> Token {
        let current = self.eat();
        if current.token_type().eq(&token_type) {
            return current;
        }
        println!(
            "\n ========== \n Parser Error! \n Expected {token_type:?} | \n Provided {:?} \n Error Message: {message}\n ========== \n",
            self.at()
        );
        exit(-1);
    }
    /// Returns `true` if current `TokenType` is `EOF`
    fn is_eof(&self) -> bool {
        self.at().token_type().eq(&TokenType::EOF)
    }

    /// Returns the current token
    fn at(&self) -> Token {
        self.tokens
            .get(self.cursor)
            .cloned()
            .unwrap_or_else(|| Token::eof())
    }
    fn eat(&mut self) -> Token {
        let current_token = self.at();
        if !self.is_eof() {
            self.cursor += 1;
        }
        current_token
    }
    fn error(&self, error_message: &str) {
        println!(
            "\n ========== \n Parser Error! \n Current Token {:?} | \n Error Message: {error_message}\n ========== \n",
            self.at()
        );
        exit(-1);
    }
}
