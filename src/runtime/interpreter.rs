use crate::frontend::parser::ast::{Expression, Program, Statement};
use crate::runtime::RuntimeValue;
use crate::runtime::scope::Scope;
use log::error;

pub struct Interpreter {}
impl Interpreter {
    pub fn evaluate_program(&self, program: Program, parent_scope: &mut Scope) -> RuntimeValue {
        let mut last_evaluated = RuntimeValue::Null;

        for statement in program.body() {
            last_evaluated = self.evaluate(statement, parent_scope);
        }

        last_evaluated
    }

    pub fn evaluate(&self, statement: &Statement, scope: &mut Scope) -> RuntimeValue {
        match statement {
            Statement::Semicolon => return RuntimeValue::Null,
            Statement::Let {
                name,
                value,
                is_constant,
            } => {
                self.evaluate_variable_declaration(( name, value, is_constant), scope)
            }
            Statement::Expr(expr) => {
                self.evaluate_expression(&expr, scope)
            }
        }
    }

    pub fn evaluate_expression(&self, expression: &Expression, scope: &mut Scope) -> RuntimeValue {
        match expression {
            Expression::Identifier(val) => {
                self.evaluate_identifier(val, scope)
            },
            Expression::Number(num) => {
                RuntimeValue::Number(*num)
            },
            Expression::BinaryExpression {
                left,
                operator,
                right,
            } => {
                self.evaluate_binary_expression((left, operator, right), scope)
            },
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
            return self.evaluate_numeric_binary_expression(
                (
                    left.as_number().unwrap(),
                    binary.1,
                    right.as_number().unwrap(),
                )
            );
        }
        RuntimeValue::Null
    }

    fn evaluate_numeric_binary_expression(
        &self,
        binary: (f64, &String, f64),
    ) -> RuntimeValue {
        match binary.1.as_str() {
            "+" => RuntimeValue::Number(binary.0 + binary.2),
            "-" => RuntimeValue::Number(binary.0 - binary.2),
            "*" => RuntimeValue::Number(binary.0 * binary.2),
            "/" => RuntimeValue::Number(binary.0 / binary.2),
            "%" => RuntimeValue::Number(binary.0 % binary.2),
            "^" => RuntimeValue::Number(binary.0.powf(binary.2)),

            _ => {
                error!(
                    "\n ===== \n Runtime Error \n Found invalid operator during binary expression evaluating \n operator {:?} \n ====== \n",
                    binary.1
                );
                RuntimeValue::Null
            }
        }
    }

    pub fn new() -> Self {
        Self {}
    }
    fn evaluate_variable_declaration(&self, statement: (&String, &Option<Expression>, &bool), scope: &mut Scope) -> RuntimeValue {
        let variable_name = statement.0.to_string();
        let is_constant = *statement.2;

        let value: Option<RuntimeValue> = match statement.1 {
            Some(expr) => Option::from(self.evaluate_expression(expr, scope)),
            None => {None},
        };

        scope.declare_variable(is_constant, variable_name, value)

    }
}
