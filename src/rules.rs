#[macro_export]
macro_rules! rule_required {
    ($name:ident) => {
        $crate::rule::Rule::new(
            stringify!($name),
            |obj: &Self, error: &mut $crate::error::ValidationError| {
                if obj.$name.is_none() {
                    error.add("required");
                } else if let Some(v) = &obj.$name {
                    if v.is_empty() {
                        error.add("required");
                    }
                }
            },
        )
    };
}

#[macro_export]
macro_rules! rule_email {
    ($name:ident) => {
        $crate::rule::Rule::new(
            stringify!($name),
            |obj: &Self, error: &mut $crate::error::ValidationError| {
                if let Some(v) = &obj.$name {
                    if !::validator::validate_email(v) {
                        error.add("email");
                    }
                }
            },
        )
    };
}

#[macro_export]
macro_rules! rule_url {
    ($name:ident) => {
        $crate::rule::Rule::new(
            stringify!($name),
            |obj: &Self, error: &mut $crate::error::ValidationError| {
                if let Some(v) = &obj.$name {
                    if !::validator::validate_url(v) {
                        error.add("url");
                    }
                }
            },
        )
    };
}

#[macro_export]
macro_rules! rule_phone {
    ($name:ident) => {
        $crate::rule::Rule::new(
            stringify!($name),
            |obj: &Self, error: &mut $crate::error::ValidationError| {
                if let Some(v) = &obj.$name {
                    if !::validator::validate_phone(v) {
                        error.add("phone");
                    }
                }
            },
        )
    };
}

#[macro_export]
macro_rules! rule_non_control_character {
    ($name:ident) => {
        $crate::rule::Rule::new(
            stringify!($name),
            |obj: &Self, error: &mut $crate::error::ValidationError| {
                if let Some(v) = &obj.$name {
                    if !::validator::validate_non_control_character(v) {
                        error.add("non_control_character");
                    }
                }
            },
        )
    };
}

#[macro_export]
macro_rules! rule_ip {
    ($name:ident) => {
        $crate::rule::Rule::new(
            stringify!($name),
            |obj: &Self, error: &mut $crate::error::ValidationError| {
                if let Some(v) = &obj.$name {
                    if !::validator::validate_ip(v) {
                        error.add("ip");
                    }
                }
            },
        )
    };
}

#[macro_export]
macro_rules! rule_ip_v4 {
    ($name:ident) => {
        $crate::rule::Rule::new(
            stringify!($name),
            |obj: &Self, error: &mut $crate::error::ValidationError| {
                if let Some(v) = &obj.$name {
                    if !::validator::validate_ip_v4(v) {
                        error.add("ip_v4");
                    }
                }
            },
        )
    };
}

#[macro_export]
macro_rules! rule_ip_v6 {
    ($name:ident) => {
        $crate::rule::Rule::new(
            stringify!($name),
            |obj: &Self, error: &mut $crate::error::ValidationError| {
                if let Some(v) = &obj.$name {
                    if !::validator::validate_ip_v6(v) {
                        error.add("ip_v6");
                    }
                }
            },
        )
    };
}

#[macro_export]
macro_rules! rule_credit_card {
    ($name:ident) => {
        $crate::rule::Rule::new(
            stringify!($name),
            |obj: &Self, error: &mut $crate::error::ValidationError| {
                if let Some(v) = &obj.$name {
                    if !::validator::validate_credit_card(v) {
                        error.add("credit_card");
                    }
                }
            },
        )
    };
}

#[macro_export]
macro_rules! rule_contains {
    ($name:ident, $needle:expr) => {
        $crate::rule::Rule::new(
            stringify!($name),
            |obj: &Self, error: &mut $crate::error::ValidationError| {
                if let Some(v) = &obj.$name {
                    if !::validator::validate_contains(v, $needle) {
                        error.add(&format!("contains:{}", $needle));
                    }
                }
            },
        )
    };
}

#[macro_export]
macro_rules! rule_equalt_to {
    ($name:ident, $second_name:ident) => {
        $crate::rule::Rule::new(
            stringify!($name),
            |obj: &Self, error: &mut $crate::error::ValidationError| {
                if obj.$name.is_some() && obj.$name.is_some() {
                    if obj.$name != obj.$second_name {
                        error.add(
                            "equalt_to:{}!={}",
                            stringify!($name),
                            stringify!($second_name),
                        );
                    }
                }
            },
        )
    };
}

#[macro_export]
macro_rules! rule_in {
    ($name:ident, $items:expr) => {
        $crate::rule::Rule::new(
            stringify!($name),
            |obj: &Self, error: &mut $crate::error::ValidationError| {
                if let Some(v) = &obj.$name {
                    if !$items.contains(&v) {
                        let string_items = $items
                            .iter()
                            .map(|x| x.to_string())
                            .collect::<Vec<String>>();
                        error.add(&format!("in:{}", string_items.join(",")));
                    }
                }
            },
        )
    };
}

#[macro_export]
macro_rules! rule_lenght_min {
    ($name:ident, $min:expr) => {
        $crate::rule::Rule::new(
            stringify!($name),
            |obj: &Self, error: &mut $crate::error::ValidationError| {
                if let Some(v) = &obj.$name {
                    if v.len() < $min {
                        error.add(&format!("lenght_min:{}", $min));
                    }
                }
            },
        )
    };
}

#[macro_export]
macro_rules! rule_lenght_max {
    ($name:ident, $max:expr) => {
        $crate::rule::Rule::new(
            stringify!($name),
            |obj: &Self, error: &mut $crate::error::ValidationError| {
                if let Some(v) = &obj.$name {
                    if v.len() > $max {
                        error.add(&format!("lenght_max:{}", $max));
                    }
                }
            },
        )
    };
}

#[macro_export]
macro_rules! rule_lenght_eq {
    ($name:ident, $eq:expr) => {
        $crate::rule::Rule::new(
            stringify!($name),
            |obj: &Self, error: &mut $crate::error::ValidationError| {
                if let Some(v) = &obj.$name {
                    if v.len() != $eq {
                        error.add(&format!("lenght_eq:{}", $eq));
                    }
                }
            },
        )
    };
}

#[macro_export]
macro_rules! rule_range {
    ($name:ident, $min:expr, $max:expr) => {
        $crate::rule::Rule::new(
            stringify!($name),
            |obj: &Self, error: &mut $crate::error::ValidationError| {
                if let Some(v) = &obj.$name {
                    if !::validator::validate_range(v, $min, $max) {
                        let min: String = match $min {
                            Some(v) => v.to_string(),
                            None => "-inf".to_string(),
                        };
                        let max: String = match $max {
                            Some(v) => v.to_string(),
                            None => "+inf".to_string(),
                        };

                        error.add(&format!("range:{}-{}", min, max));
                    }
                }
            },
        )
    };

    ($name:ident, $min:expr) => {
        $crate::rule::Rule::new(
            stringify!($name),
            |obj: &Self, error: &mut $crate::error::ValidationError| {
                if let Some(v) = &obj.$name {
                    if !::validator::validate_range(v, Some(&$min), None) {
                        error.add(format!("range:{}-{}", $min, "inf"));
                    }
                }
            },
        )
    };
}
