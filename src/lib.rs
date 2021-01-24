//! # Validr
//!
//! Is a validation crate.
//!
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
//! ```rust,ignore
//! fn rules(&self) -> Vec<Rule<Self>> {
//!     vec![rule_required!(field_name_on_self)]
//! }
//! ```
//!
//! ## Email
//!
//! For `Option<T: ToString + Clone>` it will check if the field is present and valid email
//! For `String` it will check if it's valid email.
//!
//! ```rust,ignore
//! fn rules(&self) -> Vec<Rule<Self>> {
//!     vec![rule_email!(field_name_on_self)]
//! }
//! ```
//!
//! ## Url
//!
//! For `Option<T: ToString + Clone>` it will check if the field is present and valid url
//! For `String` it will check if it's valid url.
//!
//! ```rust,ignore
//! fn rules(&self) -> Vec<Rule<Self>> {
//!     vec![rule_url!(field_name_on_self)]
//! }
//! ```
//!
//! ## Phone
//!
//! For `Option<T: ToString + Clone>` it will check if the field is present and valid phone number
//! For `String` it will check if it's valid phone number.
//!
//! ```rust,ignore
//! fn rules(&self) -> Vec<Rule<Self>> {
//!     vec![rule_phone!(field_name_on_self)]
//! }
//! ```
//!
//! ## Non Control Character
//!
//! For `Option<T: ToString + Clone>` it will check if the field is present and has no control characters
//! For `String` it will check if the field has no control characters
//!
//! ```rust,ignore
//! fn rules(&self) -> Vec<Rule<Self>> {
//!     vec![rule_non_control_character!(field_name_on_self)]
//! }
//! ```
//!
//! ## IP
//!
//! For `Option<T: ToString + Clone>` it will check if the field is present and valid IP
//! For `String` it will check if it's valid IP.
//!
//! ```rust,ignore
//! fn rules(&self) -> Vec<Rule<Self>> {
//!     vec![rule_ip!(field_name_on_self)]
//! }
//! ```
//!
//! ## IP V4
//!
//! For `Option<T: ToString + Clone>` it will check if the field is present and valid IP V4
//! For `String` it will check if it's valid IP V4.
//!
//! ```rust,ignore
//! fn rules(&self) -> Vec<Rule<Self>> {
//!     vec![rule_ip_v4!(field_name_on_self)]
//! }
//! ```
//!
//! ## IP V6
//!
//! For `Option<T: ToString + Clone>` it will check if the field is present and valid IP V6
//! For `String` it will check if it's valid IP V6.
//!
//! ```rust,ignore
//! fn rules(&self) -> Vec<Rule<Self>> {
//!     vec![rule_ip_v6!(field_name_on_self)]
//! }
//! ```
//!
//! ## Credit card
//!
//! For `Option<T: ToString + Clone>` it will check if the field is present and valid CC number
//! For `String` it will check if it's valid CC number.
//!
//! ```rust,ignore
//! fn rules(&self) -> Vec<Rule<Self>> {
//!     vec![rule_credit_card!(field_name_on_self)]
//! }
//! ```
//!
//! ## Contains
//!
//! For `Option<T: ToString + Clone>` it will check if the field is present and contains given `needle`
//! For `String` it will check if it contains given `needle`.
//!
//! ```rust,ignore
//! fn rules(&self) -> Vec<Rule<Self>> {
//!     vec![rule_contains!(field_name_on_self, "needle")]
//! }
//! ```
//!
//! ## Equal to
//!
//! It validates if two given field names are equal.
//!
//! ```rust,ignore
//! fn rules(&self) -> Vec<Rule<Self>> {
//!     vec![rule_equalt_to!(field_name_on_self, field_name_to_compare_to_on_self)]
//! }
//! ```
//!
//! ## In
//!
//! For `Option<T: ToString + Clone>` it will check if the field is present and will match its value to haystack of values
//! For `String` it will check if its in the haystack value
//!
//! ```rust,ignore
//! fn rules(&self) -> Vec<Rule<Self>> {
//!     vec![
//!         rule_in!(field_name_on_self, vec![
//!             "allowed_value".to_string(),
//!             "another_allowed_value".to_string()
//!         ]),
//!     ]
//! }
//! ```
//!
//! ## Lenght min
//!
//! For `Option<T: ToString + Clone>` it will check if the field is present and has `min` number of chars
//! For `String` it will check if it has `min` number of chars
//!
//! ```rust,ignore
//! fn rules(&self) -> Vec<Rule<Self>> {
//!     vec![rule_lenght_min!(field_name_on_self, 2)]
//! }
//! ```
//!
//! ## Lenght max
//!
//! For `Option<T: ToString + Clone>` it will check if the field is present and has `max` number of chars
//! For `String` it will check if it has `max` number of chars
//!
//! ```rust,ignore
//! fn rules(&self) -> Vec<Rule<Self>> {
//!     vec![rule_lenght_max!(field_name_on_self, 15)]
//! }
//! ```
//!
//! ## Lenght equal
//!
//! For `Option<T: ToString + Clone>` it will check if the field is present and has `eq` number of chars
//! For `String` it will check if it has `eq` number of chars
//!
//! ```rust,ignore
//! fn rules(&self) -> Vec<Rule<Self>> {
//!     vec![rule_lenght_eq!(field_name_on_self, 10)]
//! }
//! ```
//!
//! ## Lenght not equal
//!
//! For `Option<T: ToString + Clone>` it will check if the field is present and has `ne` number of chars
//! For `String` it will check if it has `ne` number of chars
//!
//! ```rust,ignore
//! fn rules(&self) -> Vec<Rule<Self>> {
//!     vec![rule_lenght_ne!(field_name_on_self, 11)]
//! }
//! ```
//!
//! ## Range
//!
//! For `Option<T: Into<f64> + PartialOrd + Clone>` it will check that the value is present and within given range.
//! For `T: Into<f64>` it will check if the value is in the given range
//!
//! ```rust,ignore
//! fn rules(&self) -> Vec<Rule<Self>> {
//!     vec![rule_range!(field_name_on_self, 11)]
//! }
//! ```
//!
//! ## Custom validation rule
//!
//! You can always implement a custom validation rule by instead of using provided
//! macros generate your own `Rule::new()` definition:
//!
//! ```rust,ignore
//! use validr::{Rule, ValidationError};
//! fn rules(&self) -> Vec<Rule<Self>> {
//!     vec![
//!         Rule::new("field_name", |obj: &Self, error: &mut ValidationError| {
//!             if obj.field_name != "some_validation_rule".to_string() {
//!                 error.add("my_custom_error_code");
//!             }
//!         }),
//!     ]
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
//! ```rust,ignore
//! fn modifiers(&self) -> Vec<Modifier<Self>> {
//!     vec![modifier_trim!(field_name_on_self)]
//! }
//! ```
//!
//! ## Lowercase
//!
//! For `Option<String>` it will check if there is some value and will run the lowercase on the value.
//! For `String` it will simply lowercase it
//!
//! ```rust,ignore
//! fn modifiers(&self) -> Vec<Modifier<Self>> {
//!     vec![modifier_lowercase!(field_name_on_self)]
//! }
//! ```
//!
//! ## Uppercase
//!
//! For `Option<String>` it will check if there is some value and will run the uppercase on the value.
//! For `String` it will simply uppercase it
//!
//! ```rust,ignore
//! fn modifiers(&self) -> Vec<Modifier<Self>> {
//!     vec![modifier_uppercase!(field_name_on_self)]
//! }
//! ```
//!
//! ## Capitalize
//!
//! For `Option<String>` it will check if there is some value and will run the capitalize on the value.
//! For `String` it will simply capitalize it
//!
//! Capitalize means it will turn the first char of the string to uppercase, and everything else will be lowercase
//!
//! ```rust,ignore
//! fn modifiers(&self) -> Vec<Modifier<Self>> {
//!     vec![modifier_capitalize!(field_name_on_self)]
//! }
//! ```
//!
//! ## Custom modifier
//!
//! Implementing custom modifier is similar to custom validation rule, you will provide a custom
//! implementation of `Modifier::new()`:
//!
//! ```rust,ignore
//! use validr::Modifier;
//! fn modifiers(&self) -> Vec<Modifier<Self>> {
//!     vec![
//!         Modifier::new("field_name", |obj: &mut Self| {
//!             obj.field_name = "new_value".to_string();
//!         }),
//!     ]
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
mod test {
    use super::*;
    use actix_web::{http, web, HttpResponse, ResponseError};
    use serde::Deserialize;

    #[derive(Clone, Deserialize, Debug)]
    struct TestObj {
        pub name: String,
        pub email: Option<String>,
        pub age: Option<u8>,
        pub ip: Option<String>,
        pub ip_v4: Option<String>,
        pub ip_v6: Option<String>,
        pub another_name: String,
    }

    impl Validation for TestObj {
        fn modifiers(&self) -> Vec<Modifier<Self>> {
            vec![
                modifier_trim!(name),
                modifier_capitalize!(name),
                modifier_lowercase!(email),
            ]
        }

        fn rules(&self) -> Vec<Rule<Self>> {
            vec![
                rule_required!(name),
                rule_lenght_min!(name, 2),
                rule_lenght_max!(name, 10),
                rule_email!(email),
                rule_range!(age, Some(15.5), Some(25)),
                rule_in!(
                    email,
                    vec!["test@test.com".to_string(), "test2@test.com".to_string()]
                ),
                rule_in!(age, vec![15, 16, 17]),
                rule_ip!(ip),
                rule_ip_v4!(ip_v4),
                rule_ip_v6!(ip_v6),
            ]
        }
    }

    /// Test actix route handler
    async fn test_actix_route_handler(test: web::Json<TestObj>) -> HttpResponse {
        match test.into_inner().validate() {
            Ok(item) => {
                println!("This is your data validated and modified: {:?}", item);
                HttpResponse::Ok().body("Validation passed!")
            }
            Err(e) => e.error_response(),
        }
    }

    #[actix_web::main]
    #[test]
    async fn test_actix_integration_fails_validation() {
        let data = TestObj {
            name: "".to_string(),
            email: None,
            age: None,
            ip: None,
            ip_v4: None,
            ip_v6: None,
            another_name: "".to_string(),
        };

        let response = test_actix_route_handler(web::Json(data)).await;

        assert_eq!(response.status(), http::StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[test]
    fn test_it_will_trim_name() {
        let obj = TestObj {
            name: " John".to_string(),
            email: None,
            age: None,
            ip: None,
            ip_v4: None,
            ip_v6: None,
            another_name: "".to_string(),
        };

        let response = obj.validate().unwrap();

        assert_eq!(response.name, "John".to_string());
    }

    #[test]
    fn test_it_will_capitalize_name() {
        let obj = TestObj {
            name: "john".to_string(),
            email: None,
            age: None,
            ip: None,
            ip_v4: None,
            ip_v6: None,
            another_name: "".to_string(),
        };

        let response = obj.validate().unwrap();

        assert_eq!(response.name, "John".to_string());
    }

    #[test]
    fn test_if_will_fail_required_name() {
        let obj = TestObj {
            name: "".to_string(),
            email: None,
            age: None,
            ip: None,
            ip_v4: None,
            ip_v6: None,
            another_name: "".to_string(),
        };

        match obj.validate() {
            Ok(_) => panic!("Was expected to validate missing 'name' property"),
            Err(e) => match e.get_error("name") {
                Ok(e) => assert!(e.contains("required")),
                Err(_) => panic!("Seems like there is no error for 'name' field"),
            },
        };
    }

    #[test]
    fn test_if_will_fail_name_too_short() {
        let obj = TestObj {
            name: "a".to_string(),
            email: None,
            age: None,
            ip: None,
            ip_v4: None,
            ip_v6: None,
            another_name: "".to_string(),
        };

        match obj.validate() {
            Ok(_) => panic!("Was expected to validate short 'name' property"),
            Err(e) => match e.get_error("name") {
                Ok(e) => assert!(e.contains("lenght_min")),
                Err(_) => panic!("Seems like there is no error for 'name' field"),
            },
        };
    }

    #[test]
    fn test_if_will_fail_valid_email() {
        let obj = TestObj {
            name: "".to_string(),
            email: Some("test_wrong_email.com".to_string()),
            age: None,
            ip: None,
            ip_v4: None,
            ip_v6: None,
            another_name: "".to_string(),
        };

        match obj.validate() {
            Ok(_) => panic!("Was expected to validate wrong email property"),
            Err(e) => match e.get_error("email") {
                Ok(e) => assert!(e.contains("email")),
                Err(_) => panic!("Seems like there is no error for 'email' field"),
            },
        };
    }

    #[test]
    fn test_if_it_fail_age_test() {
        let obj = TestObj {
            name: "John".to_string(),
            email: Some("test@test.com".to_string()),
            age: Some(12),
            ip: None,
            ip_v4: None,
            ip_v6: None,
            another_name: "".to_string(),
        };

        match obj.validate() {
            Ok(_) => panic!("Was expected to validate too low age property"),
            Err(e) => match e.get_error("age") {
                Ok(e) => assert!(e.contains("range")),
                Err(_) => panic!("Seems like there is no error for 'age' field"),
            },
        };
    }

    #[test]
    fn test_age_fails_in_rule() {
        let obj = TestObj {
            name: "John".to_string(),
            email: Some("test@test.com".to_string()),
            age: Some(18),
            ip: None,
            ip_v4: None,
            ip_v6: None,
            another_name: "".to_string(),
        };

        match obj.validate() {
            Ok(_) => panic!("Was expected to validate 'in_rule' age property"),
            Err(e) => match e.get_error("age") {
                Ok(e) => {
                    assert!(e.contains("in"));
                }
                Err(_) => panic!("Seems like there is no error for 'age' field"),
            },
        };
    }

    #[test]
    fn test_age_fails_range_rule() {
        let obj = TestObj {
            name: "John".to_string(),
            email: Some("test@test.com".to_string()),
            age: Some(27),
            ip: None,
            ip_v4: None,
            ip_v6: None,
            another_name: "".to_string(),
        };

        match obj.validate() {
            Ok(_) => panic!("Was expected to validate 'range' age property"),
            Err(e) => match e.get_error("age") {
                Ok(e) => {
                    assert!(e.contains("range"));
                }
                Err(_) => panic!("Seems like there is no error for 'age' field"),
            },
        };
    }

    #[test]
    fn test_if_ip_fails() {
        let obj = TestObj {
            name: "John".to_string(),
            email: Some("test@test.com".to_string()),
            age: Some(12),
            ip: Some("invalid_ip".to_string()),
            ip_v4: None,
            ip_v6: None,
            another_name: "".to_string(),
        };

        match obj.validate() {
            Ok(_) => panic!("Was expected to validate IP property"),
            Err(e) => match e.get_error("ip") {
                Ok(e) => assert!(e.contains("ip")),
                Err(_) => panic!("Seems like there is no error for 'ip' field"),
            },
        };
    }

    #[test]
    fn test_if_ip_v4_fails() {
        let obj = TestObj {
            name: "John".to_string(),
            email: Some("test@test.com".to_string()),
            age: Some(12),
            ip: None,
            ip_v4: Some("invalid_ip_v4".to_string()),
            ip_v6: None,
            another_name: "".to_string(),
        };

        match obj.validate() {
            Ok(_) => panic!("Was expected to validate IP V4 property"),
            Err(e) => match e.get_error("ip_v4") {
                Ok(e) => assert!(e.contains("ip_v4")),
                Err(_) => panic!("Seems like there is no error for 'ip_v4' field"),
            },
        };
    }

    #[test]
    fn test_if_ip_v6_fails() {
        let obj = TestObj {
            name: "John".to_string(),
            email: Some("test@test.com".to_string()),
            age: Some(12),
            ip: None,
            ip_v4: None,
            ip_v6: Some("invalid_ip_v6".to_string()),
            another_name: "".to_string(),
        };

        match obj.validate() {
            Ok(_) => panic!("Was expected to validate IP V6 property"),
            Err(e) => match e.get_error("ip_v6") {
                Ok(e) => assert!(e.contains("ip_v6")),
                Err(_) => panic!("Seems like there is no error for 'ip_v6' field"),
            },
        };
    }

    #[test]
    fn test_if_ips_are_ok() {
        let obj = TestObj {
            name: "John".to_string(),
            email: Some("test@test.com".to_string()),
            age: Some(12),
            ip: Some("127.0.0.1".to_string()),
            ip_v4: Some("127.0.0.1".to_string()),
            ip_v6: Some("::1".to_string()),
            another_name: "".to_string(),
        };

        match obj.validate() {
            Ok(_) => panic!("Was expected to validate IP all properties"),
            Err(e) => match e.get_error("ip_v6") {
                Ok(e) => {
                    assert!(!e.contains("ip"));
                    assert!(!e.contains("ip_v4"));
                    assert!(!e.contains("ip_v6"));
                }
                Err(_) => (),
            },
        };
    }
}
