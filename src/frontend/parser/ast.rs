#[derive(Debug, Clone)]
pub struct Property {
    pub key: String,
    pub value: Box<Expression>,
}

#[derive(Debug, Clone)]
pub struct ObjectLiteral(pub Vec<Property>);
impl ObjectLiteral {
    pub fn add_property(&mut self, property: Property) {
        self.0.push(property);
    }
}


#[derive(Debug, Clone)]
pub enum Expression {
    BinaryExpression { left: Box<Expression>, operator: String, right: Box<Expression> },
    Identifier(String),
    ObjectLiteral(ObjectLiteral),

    String(String),
    Number(f64),
    Bool(bool),
    Null,
}

#[derive(Debug)]
pub enum Statement {
    Expr(Expression),
    Semicolon,
    Let { variable_name: String, value: Option<Expression>, is_constant: bool },
    VariableAssignment { variable_name: String, value: Expression },
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

