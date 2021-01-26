use super::error::{ValidationError, ValidationErrors};
use super::modifier::Modifier;
use super::rule::Rule;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Mutex;

pub struct Validator<T> {
    item: T,
    rules: Vec<Rule<T>>,
    modifiers: Vec<Modifier<T>>,
    errors: Mutex<HashMap<String, ValidationError>>,
}

impl<T> Validator<T>
where
    T: Clone + for<'de> Deserialize<'de>,
{
    pub fn new(item: T) -> Validator<T> {
        Validator {
            item,
            rules: vec![],
            modifiers: vec![],
            errors: Mutex::new(HashMap::<String, ValidationError>::new()),
        }
    }

    /// Add rule for the struct parameter
    pub fn add_validation(mut self, rule: Rule<T>) -> Self {
        self.rules.push(rule);

        self
    }

    /// Add modifier for a given key on the item
    pub fn add_modifier(mut self, modifier: Modifier<T>) -> Self {
        self.modifiers.push(modifier);

        self
    }

    /// Check if item attribute exists with some rules already defined
    pub fn contains_rule(&self, key: &str) -> bool {
        matches!(self.rules.iter().position(|r| r.field == key), Some(_))
    }

    /// Check if item attribute exists with some modifiers already defined
    pub fn contains_modifier(&self, key: &str) -> bool {
        matches!(self.modifiers.iter().position(|r| r.field == key), Some(_))
    }

    /// Get instance error for field
    fn get_error(&self, key: &str) -> ValidationError {
        if self.errors.lock().unwrap().contains_key(key) {
            if let Some(e) = self.errors.lock().unwrap().remove(key) {
                return e;
            }
        }

        ValidationError::new()
    }

    /// Set instance error for field
    fn set_error(&self, key: &str, error: ValidationError) {
        self.errors.lock().unwrap().insert(key.to_string(), error);
    }

    /// Check if the errors are empty
    fn has_errors(&self) -> bool {
        !self.errors.lock().unwrap().is_empty()
    }

    /// Pack errors together into a single ValidationErrors instance
    fn pack_errors(&mut self) -> ValidationErrors {
        let mut errors = ValidationErrors::new();

        let inner_errors = self.errors.lock().unwrap();

        for (_key, error) in inner_errors.iter() {
            errors.add(error.clone());
        }

        errors
    }

    /// Run modifiers and validators all at once and return errors, or the item
    pub fn run(mut self) -> Result<T, ValidationErrors> {
        for modifier in &self.modifiers {
            modifier.handle(&mut self.item);
        }

        for rule in &self.rules {
            let mut error = self.get_error(&rule.field);

            rule.handle(&self.item, &mut error);

            if error.has_errors() {
                self.set_error(&rule.field, error.clone());
            }
        }

        if self.has_errors() {
            Err(self.pack_errors())
        } else {
            Ok(self.item)
        }
    }
}
