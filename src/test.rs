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
    pub agree_first: Option<bool>,
    pub agree_second: bool,
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
            rule_length_min!(name, 2),
            rule_length_max!(name, 10),
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
            rule_equalt_to!(ip, ip_v4),
            rule_not_equalt_to!(ip, ip_v6),
            rule_accepted!(agree_first),
            rule_accepted!(agree_second),
        ]
    }
}

#[derive(Clone, Deserialize, Debug)]
struct TestObj2 {
    pub name: String,
}

impl Validation for TestObj2 {
    fn rules(&self) -> Vec<Rule<Self>> {
        vec![rule_in!(name, vec!["one".to_string(), "two".to_string()])]
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

/// Test actix route handler
async fn test_actix_route_handler_2(test: web::Json<TestObj2>) -> HttpResponse {
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
async fn test_regular_string_in_rule_passing() {
    let data = TestObj2 {
        name: "one".to_string(),
    };

    let response = test_actix_route_handler_2(web::Json(data)).await;

    assert_eq!(response.status(), http::StatusCode::OK);
}

#[actix_web::main]
#[test]
async fn test_regular_string_in_rule_failing() {
    let data = TestObj2 {
        name: "three".to_string(),
    };

    let response = test_actix_route_handler_2(web::Json(data)).await;

    assert_eq!(response.status(), http::StatusCode::UNPROCESSABLE_ENTITY);
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
        agree_first: Some(true),
        agree_second: true,
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
        ip: Some("127.1.1.1".to_string()),
        ip_v4: Some("127.1.1.1".to_string()),
        ip_v6: None,
        another_name: "".to_string(),
        agree_first: Some(true),
        agree_second: true,
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
        ip: Some("127.1.1.1".to_string()),
        ip_v4: Some("127.1.1.1".to_string()),
        ip_v6: None,
        another_name: "".to_string(),
        agree_first: Some(true),
        agree_second: true,
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
        agree_first: Some(true),
        agree_second: true,
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
        agree_first: Some(true),
        agree_second: true,
    };

    match obj.validate() {
        Ok(_) => panic!("Was expected to validate short 'name' property"),
        Err(e) => match e.get_error("name") {
            Ok(e) => assert!(e.contains("length_min")),
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
        agree_first: Some(true),
        agree_second: true,
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
fn test_try_to_pass_rule_in() {
    let obj = TestObj {
        name: "".to_string(),
        email: Some("test@test.com".to_string()),
        age: None,
        ip: None,
        ip_v4: None,
        ip_v6: None,
        another_name: "".to_string(),
        agree_first: Some(true),
        agree_second: true,
    };

    match obj.validate() {
        Ok(_) => (),
        Err(e) => match e.get_error("email") {
            Ok(e) => assert!(!e.contains("in")),
            Err(_) => (),
        },
    };
}

#[test]
fn test_try_to_fail_rule_in() {
    let obj = TestObj {
        name: "".to_string(),
        email: Some("test+wrong@test.com".to_string()),
        age: None,
        ip: None,
        ip_v4: None,
        ip_v6: None,
        another_name: "".to_string(),
        agree_first: Some(true),
        agree_second: true,
    };

    match obj.validate() {
        Ok(_) => panic!("Was expected to validate wrong email property"),
        Err(e) => match e.get_error("email") {
            Ok(e) => assert!(e.contains("in")),
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
        agree_first: Some(true),
        agree_second: true,
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
        agree_first: Some(true),
        agree_second: true,
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
        agree_first: Some(true),
        agree_second: true,
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
        agree_first: Some(true),
        agree_second: true,
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
        agree_first: Some(true),
        agree_second: true,
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
        agree_first: Some(true),
        agree_second: true,
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
        agree_first: Some(true),
        agree_second: true,
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

#[test]
fn test_equal_to_rule() {
    let obj = TestObj {
        name: "".to_string(),
        email: None,
        age: None,
        ip: Some("127.0.0.1".to_string()),
        ip_v4: Some("127.0.0.2".to_string()),
        ip_v6: None,
        another_name: "".to_string(),
        agree_first: Some(true),
        agree_second: true,
    };

    match obj.validate() {
        Ok(_) => panic!("Was expected to validate IP all properties"),
        Err(e) => match e.get_error("ip") {
            Ok(e) => {
                assert!(e.contains("equalt_to:ip!=ip_v4"))
            }
            Err(_) => (),
        },
    };
}

#[test]
fn test_not_equal_to_rule() {
    let obj = TestObj {
        name: "".to_string(),
        email: None,
        age: None,
        ip: Some("127.0.0.1".to_string()),
        ip_v4: Some("127.0.0.2".to_string()),
        ip_v6: Some("127.0.0.1".to_string()),
        another_name: "".to_string(),
        agree_first: Some(true),
        agree_second: true,
    };

    match obj.validate() {
        Ok(_) => panic!("Was expected to validate IP all properties"),
        Err(e) => match e.get_error("ip") {
            Ok(e) => {
                assert!(e.contains("not_equalt_to:ip==ip_v6"))
            }
            Err(_) => (),
        },
    };
}

#[test]
fn test_fail_accepted_rule_in_option() {
    let obj = TestObj {
        name: "".to_string(),
        email: None,
        age: None,
        ip: Some("127.0.0.1".to_string()),
        ip_v4: Some("127.0.0.2".to_string()),
        ip_v6: Some("127.0.0.1".to_string()),
        another_name: "".to_string(),
        agree_first: Some(false),
        agree_second: true,
    };

    match obj.validate() {
        Ok(_) => panic!("Was expected to validate agree_first"),
        Err(e) => match e.get_error("agree_first") {
            Ok(e) => {
                assert!(e.contains("accepted"))
            }
            Err(_) => (),
        },
    };
}

#[test]
fn test_fail_accepted_rule() {
    let obj = TestObj {
        name: "".to_string(),
        email: None,
        age: None,
        ip: Some("127.0.0.1".to_string()),
        ip_v4: Some("127.0.0.2".to_string()),
        ip_v6: Some("127.0.0.1".to_string()),
        another_name: "".to_string(),
        agree_first: Some(true),
        agree_second: false,
    };

    match obj.validate() {
        Ok(_) => panic!("Was expected to validate agree_second"),
        Err(e) => match e.get_error("agree_second") {
            Ok(e) => {
                assert!(e.contains("accepted"))
            }
            Err(_) => (),
        },
    };
}
