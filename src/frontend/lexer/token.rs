#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Token {
    value: String,
    token_type: TokenType,
    
}
impl Token {
    pub fn new(value: String, token_type: TokenType) -> Self {
        Self { value, token_type }
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn token_type(&self) -> &TokenType {
        &self.token_type
    }
    pub fn eof() -> Self {
        Self {value: String::from("EOF"), token_type: TokenType::EOF}
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TokenType {
    OpenParen, CloseParen, // ( )
    OpenBracket, CloseBracket, // { }

    BinaryOperation, // + - / * % ^z
    Equals,

    Semicolon, Colon, // ; :
    Comma,

    Let,
    Const,

    Identifier,
    Number,

    EOF, // End Of A File
}