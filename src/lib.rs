//! Validr will allow you to modify your payload after it has been deserialized and then
//! will validate it with the rules you give it.
//!
//! usage:
//! ```rust
//! #[macro_use]
//! use validr::*;
//! use serde::Deserialize;
//! use actix_web::{web, HttpResponse, ResponseError};
//!
//!
//! #[derive(Clone, Deserialize, Debug)]
//! struct TestObj {
//!     pub name: Option<String>,
//!     pub email: Option<String>,
//!     pub age: Option<u8>
//! }
//!
//! impl Validation for TestObj {
//!     fn modifiers(&self) -> Vec<Modifier<Self>> {
//!         vec![
//!             modifier_trim!(name),
//!             modifier_capitalize!(name),
//!             modifier_lowercase!(email),
//!         ]
//!     }
//!
//!     fn rules(&self) -> Vec<Rule<Self>> {
//!         vec![
//!             rule_required!(name),
//!             rule_required!(email),
//!             rule_email!(email),
//!         ]
//!     }
//! }
//!
//! async fn test_actix_route_handler(test: web::Json<TestObj>) -> HttpResponse {
//!     match test.into_inner().validate() {
//!         Ok(item) => {
//!             println!("This is your data validated and modified: {:?}", item);
//!             HttpResponse::Ok().body("Validation passed!")
//!         },
//!         Err(e) => e.error_response(),
//!     }
//! }
//! ```
//!
//! # Validation rules
//! There are some rules predefined and provided for you in a form of a macro
//! to simply include in your validation.
//!
//! ## Required
//!
//! For `Option<T: ToString + Clone>` it will check if the field is present
//! For `String` it will check if it's not empty.
//!
//! ```rust
//! #[macro_use]
//! use validr::*;
//! #[derive(serde::Deserialize, Clone)]
//! struct Test {
//!     field_name_on_self: Option<String>,
//! }
//!
//! impl Validation for Test {
//!     fn rules(&self) -> Vec<Rule<Self>> {
//!         vec![rule_required!(field_name_on_self)]
//!     }
//! }
//! ```
//!
//! ## Accepted
//!
//! This rule will work for `Option<bool>` where it checks if the field is present and true,
//! and it will work for `bool` where it checks that its true.
//!
//! ```rust
//! #[macro_use]
//! use validr::*;
//! #[derive(serde::Deserialize, Clone)]
//! struct Test {
//!     field_name_on_self: Option<bool>,
//! }
//!
//! impl Validation for Test {
//!     fn rules(&self) -> Vec<Rule<Self>> {
//!         vec![rule_accepted!(field_name_on_self)]
//!     }
//! }
//! ```
//!
//! ## Email
//!
//! For `Option<T: ToString + Clone>` it will check if the field is present and valid email
//! For `String` it will check if it's valid email.
//!
//! ```rust
//! #[macro_use]
//! use validr::*;
//! #[derive(serde::Deserialize, Clone)]
//! struct Test {
//!     field_name_on_self: Option<String>,
//! }
//!
//! impl Validation for Test {
//!     fn rules(&self) -> Vec<Rule<Self>> {
//!         vec![rule_email!(field_name_on_self)]
//!     }
//! }
//! ```
//!
//! ## Url
//!
//! For `Option<T: ToString + Clone>` it will check if the field is present and valid url
//! For `String` it will check if it's valid url.
//!
//! ```rust
//! #[macro_use]
//! use validr::*;
//! #[derive(serde::Deserialize, Clone)]
//! struct Test {
//!     field_name_on_self: Option<String>,
//! }
//!
//! impl Validation for Test {
//!     fn rules(&self) -> Vec<Rule<Self>> {
//!         vec![rule_url!(field_name_on_self)]
//!     }
//! }
//! ```
//!
//! ## Phone
//!
//! For `Option<T: ToString + Clone>` it will check if the field is present and valid phone number
//! For `String` it will check if it's valid phone number.
//!
//! ```rust
//! #[macro_use]
//! use validr::*;
//! #[derive(serde::Deserialize, Clone)]
//! struct Test {
//!     field_name_on_self: Option<String>,
//! }
//!
//! impl Validation for Test {
//!     fn rules(&self) -> Vec<Rule<Self>> {
//!         vec![rule_phone!(field_name_on_self)]
//!     }
//! }
//! ```
//!
//! ## Non Control Character
//!
//! For `Option<T: ToString + Clone>` it will check if the field is present and has no control characters
//! For `String` it will check if the field has no control characters
//!
//! ```rust
//! #[macro_use]
//! use validr::*;
//! #[derive(serde::Deserialize, Clone)]
//! struct Test {
//!     field_name_on_self: Option<String>,
//! }
//!
//! impl Validation for Test {
//!     fn rules(&self) -> Vec<Rule<Self>> {
//!         vec![rule_non_control_character!(field_name_on_self)]
//!     }
//! }
//! ```
//!
//! ## IP
//!
//! For `Option<T: ToString + Clone>` it will check if the field is present and valid IP
//! For `String` it will check if it's valid IP.
//!
//! ```rust
//! #[macro_use]
//! use validr::*;
//! #[derive(serde::Deserialize, Clone)]
//! struct Test {
//!     field_name_on_self: Option<String>,
//! }
//!
//! impl Validation for Test {
//!     fn rules(&self) -> Vec<Rule<Self>> {
//!         vec![rule_ip!(field_name_on_self)]
//!     }
//! }
//! ```
//!
//! ## IP V4
//!
//! For `Option<T: ToString + Clone>` it will check if the field is present and valid IP V4
//! For `String` it will check if it's valid IP V4.
//!
//! ```rust
//! #[macro_use]
//! use validr::*;
//! #[derive(serde::Deserialize, Clone)]
//! struct Test {
//!     field_name_on_self: Option<String>,
//! }
//!
//! impl Validation for Test {
//!     fn rules(&self) -> Vec<Rule<Self>> {
//!         vec![rule_ip_v4!(field_name_on_self)]
//!     }
//! }
//! ```
//!
//! ## IP V6
//!
//! For `Option<T: ToString + Clone>` it will check if the field is present and valid IP V6
//! For `String` it will check if it's valid IP V6.
//!
//! ```rust
//! #[macro_use]
//! use validr::*;
//! #[derive(serde::Deserialize, Clone)]
//! struct Test {
//!     field_name_on_self: Option<String>,
//! }
//!
//! impl Validation for Test {
//!     fn rules(&self) -> Vec<Rule<Self>> {
//!         vec![rule_ip_v6!(field_name_on_self)]
//!     }
//! }
//! ```
//!
//! ## Credit card
//!
//! For `Option<T: ToString + Clone>` it will check if the field is present and valid CC number
//! For `String` it will check if it's valid CC number.
//!
//! ```rust
//! #[macro_use]
//! use validr::*;
//! #[derive(serde::Deserialize, Clone)]
//! struct Test {
//!     field_name_on_self: Option<String>,
//! }
//!
//! impl Validation for Test {
//!     fn rules(&self) -> Vec<Rule<Self>> {
//!         vec![rule_credit_card!(field_name_on_self)]
//!     }
//! }
//! ```
//!
//! ## Contains
//!
//! For `Option<T: ToString + Clone>` it will check if the field is present and contains given `needle`
//! For `String` it will check if it contains given `needle`.
//!
//! ```rust
//! #[macro_use]
//! use validr::*;
//! #[derive(serde::Deserialize, Clone)]
//! struct Test {
//!     field_name_on_self: Option<String>,
//! }
//!
//! impl Validation for Test {
//!     fn rules(&self) -> Vec<Rule<Self>> {
//!         vec![rule_contains!(field_name_on_self, "needle".to_string())]
//!     }
//! }
//! ```
//!
//! ## Equal to
//!
//! It validates if two given field names are equal.
//!
//! ```rust
//! #[macro_use]
//! use validr::*;
//! #[derive(serde::Deserialize, Clone)]
//! struct Test {
//!     field_name_on_self: Option<String>,
//!     field_name_to_compare_to_on_self: Option<String>,
//! }
//!
//! impl Validation for Test {
//!     fn rules(&self) -> Vec<Rule<Self>> {
//!         vec![rule_equalt_to!(field_name_on_self, field_name_to_compare_to_on_self)]
//!     }
//! }
//! ```
//!
//! ## Not equal to
//!
//! It validates if two given field names are not equal.
//!
//! ```rust
//! #[macro_use]
//! use validr::*;
//! #[derive(serde::Deserialize, Clone)]
//! struct Test {
//!     field_name_on_self: Option<String>,
//!     field_name_to_compare_to_on_self: Option<String>,
//! }
//!
//! impl Validation for Test {
//!     fn rules(&self) -> Vec<Rule<Self>> {
//!         vec![rule_not_equalt_to!(field_name_on_self, field_name_to_compare_to_on_self)]
//!     }
//! }
//! ```
//!
//! ## In
//!
//! For `Option<T: ToString + Clone>` it will check if the field is present and will match its value to haystack of values
//! For `String` it will check if its in the haystack value
//!
//! ```rust
//! #[macro_use]
//! use validr::*;
//! #[derive(serde::Deserialize, Clone)]
//! struct Test {
//!     field_name_on_self: Option<String>,
//! }
//!
//! impl Validation for Test {
//!     fn rules(&self) -> Vec<Rule<Self>> {
//!         vec![
//!             rule_in!(field_name_on_self, vec![
//!                 "allowed_value".to_string(),
//!                 "another_allowed_value".to_string()
//!             ]),
//!         ]
//!     }
//! }
//! ```
//!
//! ## length min
//!
//! For `Option<T: ToString + Clone>` it will check if the field is present and has `min` number of chars
//! For `String` it will check if it has `min` number of chars
//!
//! ```rust
//! #[macro_use]
//! use validr::*;
//! #[derive(serde::Deserialize, Clone)]
//! struct Test {
//!     field_name_on_self: Option<String>,
//! }
//!
//! impl Validation for Test {
//!     fn rules(&self) -> Vec<Rule<Self>> {
//!         vec![rule_length_min!(field_name_on_self, 2)]
//!     }
//! }
//! ```
//!
//! ## length max
//!
//! For `Option<T: ToString + Clone>` it will check if the field is present and has `max` number of chars
//! For `String` it will check if it has `max` number of chars
//!
//! ```rust
//! #[macro_use]
//! use validr::*;
//! #[derive(serde::Deserialize, Clone)]
//! struct Test {
//!     field_name_on_self: Option<String>,
//! }
//!
//! impl Validation for Test {
//!     fn rules(&self) -> Vec<Rule<Self>> {
//!         vec![rule_length_max!(field_name_on_self, 15)]
//!     }
//! }
//! ```
//!
//! ## length equal
//!
//! For `Option<T: ToString + Clone>` it will check if the field is present and has `eq` number of chars
//! For `String` it will check if it has `eq` number of chars
//!
//! ```rust
//! #[macro_use]
//! use validr::*;
//! #[derive(serde::Deserialize, Clone)]
//! struct Test {
//!     field_name_on_self: Option<String>,
//! }
//!
//! impl Validation for Test {
//!     fn rules(&self) -> Vec<Rule<Self>> {
//!         vec![rule_length_eq!(field_name_on_self, 10)]
//!     }
//! }
//! ```
//!
//! ## length not equal
//!
//! For `Option<T: ToString + Clone>` it will check if the field is present and has `ne` number of chars
//! For `String` it will check if it has `ne` number of chars
//!
//! ```rust
//! #[macro_use]
//! use validr::*;
//! #[derive(serde::Deserialize, Clone)]
//! struct Test {
//!     field_name_on_self: Option<String>,
//! }
//!
//! impl Validation for Test {
//!     fn rules(&self) -> Vec<Rule<Self>> {
//!         vec![rule_length_ne!(field_name_on_self, 11)]
//!     }
//! }
//! ```
//!
//! ## Range
//!
//! For `Option<T: Into<f64> + PartialOrd + Clone>` it will check that the value is present and within given range.
//! For `T: Into<f64>` it will check if the value is in the given range
//!
//! ```rust
//! #[macro_use]
//! use validr::*;
//! #[derive(serde::Deserialize, Clone)]
//! struct Test {
//!     field_name_on_self: Option<u8>,
//! }
//!
//! impl Validation for Test {
//!     fn rules(&self) -> Vec<Rule<Self>> {
//!         vec![rule_range!(field_name_on_self, Some(10), Some(15))]
//!     }
//! }
//! ```
//!
//! ## Custom validation rule
//!
//! You can always implement a custom validation rule by instead of using provided
//! macros generate your own `Rule::new()` definition:
//!
//! ```rust
//! #[macro_use]
//! use validr::{Validation, error::ValidationError, Rule};
//! #[derive(serde::Deserialize, Clone)]
//! struct Test {
//!     field_name: String,
//! }
//!
//! impl Validation for Test {
//!     fn rules(&self) -> Vec<Rule<Self>> {
//!         vec![
//!             Rule::new("field_name", |obj: &Self, error: &mut ValidationError| {
//!                 if obj.field_name != "some_validation_rule".to_string() {
//!                     error.add("my_custom_error_code");
//!                 }
//!             }),
//!         ]
//!     }
//! }
//! ```
//!
//! # Field modifiers
//! Before running validation rules you can modify the input data to format it in whatever way you want.
//! There are some modifiers included, but you can certainly create a custom one to do whatever you want.
//!
//! ## Trim
//!
//! For `Option<String>` it will check if there is some value and will run the trim on the value.
//! For `String` it will simply trim it
//!
//! ```rust
//! #[macro_use]
//! use validr::*;
//! #[derive(serde::Deserialize, Clone)]
//! struct Test {
//!     field_name_on_self: Option<String>,
//! }
//!
//! impl Validation for Test {
//!     fn modifiers(&self) -> Vec<Modifier<Self>> {
//!         vec![modifier_trim!(field_name_on_self)]
//!     }
//! }
//! ```
//!
//! ## Lowercase
//!
//! For `Option<String>` it will check if there is some value and will run the lowercase on the value.
//! For `String` it will simply lowercase it
//!
//! ```rust
//! #[macro_use]
//! use validr::*;
//! #[derive(serde::Deserialize, Clone)]
//! struct Test {
//!     field_name_on_self: Option<String>,
//! }
//!
//! impl Validation for Test {
//!     fn modifiers(&self) -> Vec<Modifier<Self>> {
//!         vec![modifier_lowercase!(field_name_on_self)]
//!     }
//! }
//! ```
//!
//! ## Uppercase
//!
//! For `Option<String>` it will check if there is some value and will run the uppercase on the value.
//! For `String` it will simply uppercase it
//!
//! ```rust
//! #[macro_use]
//! use validr::*;
//! #[derive(serde::Deserialize, Clone)]
//! struct Test {
//!     field_name_on_self: Option<String>,
//! }
//!
//! impl Validation for Test {
//!     fn modifiers(&self) -> Vec<Modifier<Self>> {
//!         vec![modifier_uppercase!(field_name_on_self)]
//!     }
//! }
//! ```
//!
//! ## Capitalize
//!
//! For `Option<String>` it will check if there is some value and will run the capitalize on the value.
//! For `String` it will simply capitalize it
//!
//! Capitalize will turn the first char of the string to uppercase, and everything else will be lowercase
//!
//! ```rust
//! #[macro_use]
//! use validr::*;
//! #[derive(serde::Deserialize, Clone)]
//! struct Test {
//!     field_name_on_self: Option<String>,
//! }
//!
//! impl Validation for Test {
//!     fn modifiers(&self) -> Vec<Modifier<Self>> {
//!         vec![modifier_capitalize!(field_name_on_self)]
//!     }
//! }
//! ```
//!
//! ## Custom modifier
//!
//! Implementing custom modifier is similar to custom validation rule, you will provide a custom
//! implementation of `Modifier::new()`:
//!
//! ```rust
//! #[macro_use]
//! use validr::*;
//! #[derive(serde::Deserialize, Clone)]
//! struct Test {
//!     field_name_on_self: Option<String>,
//! }
//!
//! impl Validation for Test {
//!     fn modifiers(&self) -> Vec<Modifier<Self>> {
//!         vec![
//!             Modifier::new("field_name_on_self", |obj: &mut Self| {
//!                 obj.field_name_on_self = Some("new_value".to_string());
//!             }),
//!         ]
//!     }
//! }
//! ```
//!
mod modifier;
mod modifiers;
mod rule;
mod rules;

pub mod error;
pub mod validator;
pub mod wrappers;

use serde::Deserialize;

pub use crate::validator::Validator;
pub use modifier::Modifier;
pub use modifiers::*;
pub use rule::Rule;
pub use rules::*;

pub trait Validation: Clone + for<'de> Deserialize<'de> {
    /// Method that is intended to return vector of all the validation rules
    fn rules(&self) -> Vec<Rule<Self>> {
        vec![]
    }

    /// Method that is intended to return vector of all the modifications to the object
    /// before the validation runs
    fn modifiers(&self) -> Vec<Modifier<Self>> {
        vec![]
    }

    /// This will run the validation and return the object if all the validations pass.
    /// Object will be modified by all the modifiers and ready for using further
    #[inline]
    fn validate(self) -> Result<Self, error::ValidationErrors> {
        let rules = self.rules();
        let modifiers = self.modifiers();

        let mut validator = Validator::new(self);

        for rule in rules {
            validator = validator.add_validation(rule);
        }

        for modifier in modifiers {
            validator = validator.add_modifier(modifier);
        }

        validator.run()
    }
}

#[cfg(test)]
mod test;
