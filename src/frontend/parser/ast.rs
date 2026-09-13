#[derive(Debug)]
pub enum Expression {
    BinaryExpression {left: Box<Expression>, operator: String, right: Box<Expression>},
    //Literal(),
    Identifier(String),
    Number(f64),
}

#[derive(Debug)]
pub enum Statement {
    Expr(Expression),
    Semicolon,
    Let { name:  String, value: Option<Expression>, is_constant: bool },
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

    pub fn body(&self) -> &Vec<Statement> {
        &self.body
    }
}

