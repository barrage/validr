use actix_http::{Error, Response};
use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
use futures_util::future::{ok, Ready};
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error as StdError;

#[derive(Clone, Debug, Serialize, PartialEq)]
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

#[derive(Clone, Debug, Serialize, PartialEq)]
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
    type Error = ValidationErrors;

    type Future = Ready<Result<Response, Self::Error>>;

    fn respond_to(self, _: &HttpRequest) -> Self::Future {
        let err: Error = self.into();
        ok(err.into())
    }
}
