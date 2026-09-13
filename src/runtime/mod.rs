use std::collections::HashMap;

pub mod scope;
pub mod interpreter;

#[derive(Debug, PartialEq, Clone)]
pub enum RuntimeValue {
    Number(f64),
    Bool(bool),
    Null,
    String(String),
    ObjectLiteral(HashMap<String, RuntimeValue>)

}

impl RuntimeValue {
    pub fn is_number(&self) -> bool {
        matches!(self, RuntimeValue::Number(_))
    }

    pub fn as_number(&self) -> Option<f64> {
        match self {
            RuntimeValue::Number(num) => Option::from(*num),
            _ => {
                Option::None
            }
        }
    }
}