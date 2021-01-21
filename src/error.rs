use serde::Serialize;
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize)]
pub struct ValidationError {
    field: String,
    errors: Vec<String>,
}

impl ValidationError {
    /// Get new validation error
    pub fn new() -> Self {
        ValidationError {
            field: "".to_string(),
            errors: vec![],
        }
    }

    /// Set validation error field name
    pub fn set_field_name(&mut self, name: &str) {
        self.field = name.to_string();
    }

    /// Add error code
    pub fn add(&mut self, error: &str) {
        self.errors.push(error.to_string());
    }

    /// Check if it already contains certain error code
    pub fn contains(&self, error_code: &str) -> bool {
        match self.errors.iter().position(|e| e.starts_with(error_code)) {
            Some(_) => true,
            _ => false,
        }
    }

    /// Check if validation error has anything to show
    pub fn has_errors(&self) -> bool {
        !self.field.is_empty() && self.errors.len() > 0
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ValidationErrors {
    errors: HashMap<String, ValidationError>,
}

impl ValidationErrors {
    /// Create new validation errors holder
    pub fn new() -> Self {
        ValidationErrors {
            errors: HashMap::new(),
        }
    }

    /// Add validation error
    pub fn add(&mut self, error: ValidationError) {
        let name = error.field.clone();
        let mut e = error.clone();

        if self.errors.contains_key(&name) {
            e = self.errors.remove(&name).unwrap();

            for rule in &error.errors {
                if !e.contains(&rule) {
                    e.add(&rule);
                }
            }
        }

        if e.has_errors() {
            self.errors.insert(name, e);
        }
    }

    /// Check if there are any validation errors
    pub fn has_errors(&self) -> bool {
        self.errors.len() > 0
    }

    /// Copy single error from the hash map
    pub fn get_error(&self, key: &str) -> Result<ValidationError, ()> {
        if self.has_errors() && self.errors.contains_key(key) {
            match self.errors.get_key_value(key) {
                Some((_k, error)) => Ok(error.clone()),
                None => Err(()),
            }
        } else {
            Err(())
        }
    }
}
