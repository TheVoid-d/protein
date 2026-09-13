use crate::runtime::RuntimeValue;
use std::collections::{HashMap, HashSet};
use std::process::exit;

#[derive(Debug, PartialEq)]
pub struct Scope {
    /// The HashMap which stores a variable name and an information about the value `(RuntimeValue)`
    variables: HashMap<String, RuntimeValue>,
    constants: HashSet<String>, // The HashSet which stores the constant variables

    parent_scope: Option<Box<Scope>>,
}

impl Scope {
    /// Declares a variable. Variable can be either constant or not. Constant variables `MUST`
    /// have a value and be not null, whereas default variables able to be null
    pub fn declare_variable(
        &mut self,
        is_constant: bool,
        variable_name: String,
        value: Option<RuntimeValue>,
    ) -> RuntimeValue {
        if is_constant {
            if let Some(val) = value {
                self.variables.insert(variable_name.clone(), val.clone());
                self.constants.insert(variable_name);
                return val;
            }
            self.error("Cannot declare a constant variable with no value.");
            return RuntimeValue::Null;
        }

        let val = value.unwrap_or_else(|| RuntimeValue::Null);
        self.variables.insert(variable_name.clone(), val.clone());
        println!("Added variable {variable_name} = {val:?} to a scope {self:?} storage.");
        val
    }

    /// Assigns a new value to a variable. If there is no variable in a scope, creates a new.
    pub fn assign_variable(
        &mut self,
        variable_name: &str,
        value: Option<RuntimeValue>,
    ) -> RuntimeValue {
        let val = value.unwrap_or_else(|| RuntimeValue::Null);

        if let Some(target_scope) = self.find_scope_mut(&variable_name) {
            if target_scope.constants.contains(variable_name) {
                &self.error("Cannot reassign constant value!");
                return RuntimeValue::Null;
            }
            target_scope
                .variables
                .insert(variable_name.to_string(), val.clone());
        } else {
            self.variables
                .insert(variable_name.to_string(), val.clone());
        }
        val
    }
    /// Returns the variable value or throws an error if there is no value in a scope
    pub fn inspect_variable(&self, variable_name: &str) -> RuntimeValue {
        if let Some(scope) = self.find_scope(variable_name) {
            if let Some(value) = scope.variables.get(variable_name) {
                return value.clone();
            }
        }
        self.error("No variable found in a scope.");
        return RuntimeValue::Null;
    }

    /// Returns the scope which contains the variable OR throws an error
    pub fn find_scope(&self, variable_name: &str) -> Option<&Scope> {
        if self
            .variables
            .contains_key::<String>(&variable_name.to_string())
        {
            return Option::from(self);
        }
        self.parent_scope.
            as_deref()?.
            find_scope(variable_name)
    }
    pub fn find_scope_mut(&mut self, variable_name: &str) -> Option<&mut Scope> {
        if self.variables.contains_key(variable_name) {
            return Some(self);
        }
        self.parent_scope
            .as_deref_mut()?
            .find_scope_mut(variable_name)
    }
    pub fn new(parent_scope: Option<Box<Scope>>) -> Self {
        Self {
            variables: HashMap::new(),
            constants: HashSet::new(),
            parent_scope,
        }
    }
    pub fn program_scope() -> Self {
        let mut scope = Self::new(None);
        scope.assign_variable("null", Some(RuntimeValue::Null));
        scope.assign_variable("true", Some(RuntimeValue::Bool(true)));
        scope.assign_variable("false", Some(RuntimeValue::Bool(false)));

        scope
    }
    fn error(&self, error_message: &str) {
        println!(
            "\n ========== \n Runtime Error!  \n Error Message: {error_message}\n ========== \n",
        );
        exit(-1);
    }
}
