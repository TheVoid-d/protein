use std::collections::HashMap;
use std::ops::Deref;
use std::process::exit;
use crate::frontend::parser::ast::{Expression, ObjectLiteral, Program, Property, Statement};
use crate::runtime::RuntimeValue;
use crate::runtime::scope::Scope;
use log::error;

pub struct Interpreter {}
impl Interpreter {
    pub fn evaluate_program(&self, program: Program, parent_scope: &mut Scope) -> RuntimeValue {
        let mut last_evaluated = RuntimeValue::Null;

        for statement in program.body() {
            last_evaluated = match self.evaluate(statement, parent_scope) {
                RuntimeValue::Null => last_evaluated,
                other => other,
            }
        }

        last_evaluated
    }

    pub fn evaluate(&self, statement: &Statement, scope: &mut Scope) -> RuntimeValue {
        match statement {
            Statement::Semicolon => return RuntimeValue::Null,
            Statement::Let {
                variable_name: name,
                value,
                is_constant,
            } => self.evaluate_variable_declaration((name, value, is_constant), scope),
            Statement::Expr(expr) => self.evaluate_expression(&expr, scope),
            Statement::VariableAssignment {
                variable_name,
                value,
            } => self.evaluate_variable_assignment((variable_name, value), scope),
        }
    }

    pub fn evaluate_expression(&self, expression: &Expression, scope: &mut Scope) -> RuntimeValue {
        match expression {
            Expression::Identifier(val) => self.evaluate_identifier(val, scope),
            Expression::Number(num) => RuntimeValue::Number(*num),
            Expression::String(str) => RuntimeValue::String(str.to_string()),
            Expression::Bool(val) => RuntimeValue::Bool(*val),
            Expression::Null => RuntimeValue::Null,

            Expression::BinaryExpression {
                left,
                operator,
                right,
            } => self.evaluate_binary_expression((left, operator, right), scope),

            Expression::ObjectLiteral(ObjectLiteral(properties)) => {
                self.evaluate_object_literal(properties, scope)
            }
        }
    }
    pub fn evaluate_identifier(&self, variable_name: &String, scope: &mut Scope) -> RuntimeValue {
        scope.inspect_variable(variable_name)
    }

    fn evaluate_binary_expression(
        &self,
        binary: (&Box<Expression>, &String, &Box<Expression>),
        scope: &mut Scope,
    ) -> RuntimeValue {
        let left = self.evaluate_expression(binary.0, scope);
        let right = self.evaluate_expression(binary.2, scope); 

        if left.is_number() && right.is_number() {
            return self.evaluate_numeric_binary_expression((
                left.as_number().unwrap(),
                binary.1,
                right.as_number().unwrap(),
            ));
        }
        RuntimeValue::Null
    }

    fn evaluate_numeric_binary_expression(&self, binary: (f64, &String, f64)) -> RuntimeValue {
        match binary.1.as_str() {
            "+" => RuntimeValue::Number(binary.0 + binary.2),
            "-" => RuntimeValue::Number(binary.0 - binary.2),
            "*" => RuntimeValue::Number(binary.0 * binary.2),
            "/" => RuntimeValue::Number(binary.0 / binary.2),
            "%" => RuntimeValue::Number(binary.0 % binary.2),
            "^" => RuntimeValue::Number(binary.0.powf(binary.2)),

            _ => {
                self.error(&format!("Found invalid operator during binary expression evaluating \n operator {:?}", binary.0));
                RuntimeValue::Null
            }
        }
    }

    fn error(&self, message: &str) -> ! {
        println!(
                    "\n ===== \n Runtime Error \n {:?} \n ====== \n",
                    message
                );
        exit(-1);
    } 
    pub fn new() -> Self {
        Self {}
    }
    fn evaluate_variable_declaration(
        &self,
        statement: (&String, &Option<Expression>, &bool),
        scope: &mut Scope,
    ) -> RuntimeValue {
        let variable_name = statement.0.to_string();
        let is_constant = *statement.2;

        let value: Option<RuntimeValue> = match statement.1 {
            Some(expr) => Option::from(self.evaluate_expression(expr, scope)),
            None => None,
        };

        scope.declare_variable(is_constant, variable_name, value)
    }

    fn evaluate_variable_assignment(
        &self,
        variable_assignment: (&String, &Expression),
        scope: &mut Scope,
    ) -> RuntimeValue {
        let value = Option::from(self.evaluate_expression(variable_assignment.1, scope));
        scope.assign_variable(variable_assignment.0, value)
    }

    fn evaluate_object_literal(&self, properties: &Vec<Property>, scope: &mut Scope) -> RuntimeValue {
        let mut literal_props: HashMap<String, RuntimeValue> = HashMap::new();
        println!("eval literal");
        for property in properties {
            let key = property.key.to_string();
            let value = self.evaluate_expression(property.value.deref(), scope);
            literal_props.insert(
                key,
                value
            );
        }
        println!("Literal Props: {literal_props:?}");
        RuntimeValue::ObjectLiteral(literal_props)
    }
}
