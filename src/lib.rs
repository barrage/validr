/// # Validr
/// Is a validation package made for internal project, we tried using other crates that were available
/// but none of them were quite what we were looking for and required us to either edit them too much
/// or bend over backwards to make them work the way we wanted to.
///
/// Validr will allow you to modify your payload after it has been deserialized and then
/// will validate it with the rules you give it.
///
/// It does not assume most of the things except that all of the attributes on your struct
/// are `Option` type and you are using the `required` rule to validate them and make sure
/// they contain value that you need.
///
/// usage:
/// ```rust
/// #[macro_use]
/// use validr::*;
/// use serde::Deserialize;
/// use actix_web::{web, HttpResponse, ResponseError};
///
///
/// #[derive(Clone, Deserialize, Debug)]
/// struct TestObj {
///     pub name: Option<String>,
///     pub email: Option<String>,
///     pub age: Option<u8>
/// }
///
/// impl Validation for TestObj {
///     fn modifiers(&self) -> Vec<Modifier<Self>> {
///         vec![
///             modifier_trim!(name),
///             modifier_capitalize!(name),
///             modifier_lowercase!(email),
///         ]
///     }
///
///     fn rules(&self) -> Vec<Rule<Self>> {
///         vec![
///             rule_required!(name),
///             rule_required!(email),
///             rule_email!(email),
///         ]
///     }
/// }
///
/// async fn test_actix_route_handler(test: web::Json<TestObj>) -> HttpResponse {
///     match test.into_inner().validate() {
///         Ok(item) => {
///             println!("This is your data validated and modified: {:?}", item);
///             HttpResponse::Ok().body("Validation passed!")
///         },
///         Err(e) => e.error_response(),
///     }
/// }
/// ```
///
mod modifier;
mod modifiers;
mod rule;
mod rules;

pub mod error;
pub mod validator;

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
        pub name: Option<String>,
        pub email: Option<String>,
        pub age: Option<u8>,
        pub ip: Option<String>,
        pub ip_v4: Option<String>,
        pub ip_v6: Option<String>,
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
                rule_range!(age, Some(&15), Some(&25)),
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
            name: None,
            email: None,
            age: None,
            ip: None,
            ip_v4: None,
            ip_v6: None,
        };

        let response = test_actix_route_handler(web::Json(data)).await;

        assert_eq!(response.status(), http::StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[test]
    fn test_it_will_trim_and_capitalize_name() {
        let obj = TestObj {
            name: Some(" john".to_string()),
            email: None,
            age: None,
            ip: None,
            ip_v4: None,
            ip_v6: None,
        };

        let response = obj.validate().unwrap();

        assert_eq!(response.name, Some("John".to_string()));
    }

    #[test]
    fn test_if_will_fail_required_name() {
        let obj = TestObj {
            name: None,
            email: None,
            age: None,
            ip: None,
            ip_v4: None,
            ip_v6: None,
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
            name: Some("a".to_string()),
            email: None,
            age: None,
            ip: None,
            ip_v4: None,
            ip_v6: None,
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
            name: None,
            email: Some("test_wrong_email.com".to_string()),
            age: None,
            ip: None,
            ip_v4: None,
            ip_v6: None,
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
            name: Some("John".to_string()),
            email: Some("test@test.com".to_string()),
            age: Some(12),
            ip: None,
            ip_v4: None,
            ip_v6: None,
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
    fn test_age_fails_in_rule_and_range_rule_together() {
        let obj = TestObj {
            name: Some("John".to_string()),
            email: Some("test@test.com".to_string()),
            age: Some(27),
            ip: None,
            ip_v4: None,
            ip_v6: None,
        };

        match obj.validate() {
            Ok(_) => panic!("Was expected to validate 'in_rule' and 'range' age property"),
            Err(e) => match e.get_error("age") {
                Ok(e) => {
                    assert!(e.contains("in"));
                    assert!(e.contains("range"));
                }
                Err(_) => panic!("Seems like there is no error for 'age' field"),
            },
        };
    }

    #[test]
    fn test_if_ip_fails() {
        let obj = TestObj {
            name: Some("John".to_string()),
            email: Some("test@test.com".to_string()),
            age: Some(12),
            ip: Some("invalid_ip".to_string()),
            ip_v4: None,
            ip_v6: None,
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
            name: Some("John".to_string()),
            email: Some("test@test.com".to_string()),
            age: Some(12),
            ip: None,
            ip_v4: Some("invalid_ip_v4".to_string()),
            ip_v6: None,
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
            name: Some("John".to_string()),
            email: Some("test@test.com".to_string()),
            age: Some(12),
            ip: None,
            ip_v4: None,
            ip_v6: Some("invalid_ip_v6".to_string()),
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
            name: Some("John".to_string()),
            email: Some("test@test.com".to_string()),
            age: Some(12),
            ip: Some("127.0.0.1".to_string()),
            ip_v4: Some("127.0.0.1".to_string()),
            ip_v6: Some("::1".to_string()),
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
