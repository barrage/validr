use actix_web::{body::BoxBody, HttpRequest, HttpResponse, Responder, ResponseError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error as StdError;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ValidationError {
    field: String,
    errors: Vec<String>,
}

impl Default for ValidationError {
    fn default() -> Self {
        Self::new()
    }
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
        matches!(
            self.errors.iter().position(|e| e.starts_with(error_code)),
            Some(_)
        )
    }

    /// Check if the error is empty
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    /// Get the number of errors
    pub fn len(&self) -> usize {
        self.errors.len()
    }

    /// Check if validation error has anything to show
    pub fn has_errors(&self) -> bool {
        !self.field.is_empty() && !self.errors.is_empty()
    }

    /// Return the error field name
    pub fn get_name(&self) -> String {
        self.field.clone()
    }

    /// Return all the errors
    pub fn get_errors(&self) -> Vec<String> {
        self.errors.clone()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ValidationErrors {
    errors: HashMap<String, ValidationError>,
}

impl Default for ValidationErrors {
    fn default() -> Self {
        Self::new()
    }
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
                if !e.contains(rule) {
                    e.add(rule);
                }
            }
        }

        if e.has_errors() {
            self.errors.insert(name, e);
        }
    }

    /// Check if the error is empty
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    /// Get the number of errors
    pub fn len(&self) -> usize {
        self.errors.len()
    }

    /// Check if there are any validation errors
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Copy single error from the hash map
    pub fn get_error(&self, key: &str) -> Result<ValidationError, String> {
        if self.has_errors() && self.errors.contains_key(key) {
            match self.errors.get_key_value(key) {
                Some((_k, error)) => Ok(error.clone()),
                None => Err("no_error".to_string()),
            }
        } else {
            Err("no_error".to_string())
        }
    }

    /// Return all the errors
    pub fn get_errors(&self) -> HashMap<String, ValidationError> {
        self.errors.clone()
    }
}

/// Allow the use of "{}" format specifier
impl std::fmt::Display for ValidationErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Implement std::error::Error for ValidationErrors
impl StdError for ValidationErrors {
    fn cause(&self) -> Option<&dyn StdError> {
        Some(self)
    }
}

/// Allow the error to be returned in actix as error response
impl ResponseError for ValidationErrors {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::UnprocessableEntity().json(self)
    }
}

/// Allow the error to be returned into responder for actix right away
impl Responder for ValidationErrors {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        self.error_response()
    }
}
