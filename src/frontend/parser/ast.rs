#[derive(Debug)]
pub enum Expression {
    BinaryExpression {left: Box<Expression>, operator: String, right: Box<Expression>},
    Literal(i32),
    Identifier(String),
    Number(f64),
}

#[derive(Debug)]
pub enum Statement {
    Expr(Expression),
    Semicolon,
    Let {name: String, value: Expression},
}
pub enum NodeType {
    Program,

    Semicolon,
}

#[derive(Debug)]
pub struct Program {
    body: Vec<Statement>,
}
impl Program {
    pub fn new() -> Self {
        Self { body: Vec::new() }
    }
    pub fn add_element(&mut self, value: Statement) {
        self.body.push(value);
    }
}

