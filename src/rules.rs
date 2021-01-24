#[macro_export]
macro_rules! rule_required {
    ($name:ident) => {
        $crate::Rule::new(
            stringify!($name),
            |obj: &Self, error: &mut $crate::error::ValidationError| {
                use $crate::wrappers::rules::SomeOrStringWrapper;
                if $crate::wrappers::rules::SomeOrString(&obj.$name)
                    .0
                    .required()
                {
                    error.add("required");
                }
            },
        )
    };
}

#[macro_export]
macro_rules! rule_email {
    ($name:ident) => {
        $crate::Rule::new(
            stringify!($name),
            |obj: &Self, error: &mut $crate::error::ValidationError| {
                use $crate::wrappers::rules::SomeOrStringWrapper;
                if $crate::wrappers::rules::SomeOrString(&obj.$name).0.email() {
                    error.add("email");
                }
            },
        )
    };
}

#[macro_export]
macro_rules! rule_url {
    ($name:ident) => {
        $crate::Rule::new(
            stringify!($name),
            |obj: &Self, error: &mut $crate::error::ValidationError| {
                use $crate::wrappers::rules::SomeOrStringWrapper;
                if $crate::wrappers::rules::SomeOrString(&obj.$name).0.url() {
                    error.add("url");
                }
            },
        )
    };
}

#[macro_export]
macro_rules! rule_phone {
    ($name:ident) => {
        $crate::Rule::new(
            stringify!($name),
            |obj: &Self, error: &mut $crate::error::ValidationError| {
                use $crate::wrappers::rules::SomeOrStringWrapper;
                if $crate::wrappers::rules::SomeOrString(&obj.$name).0.phone() {
                    error.add("phone");
                }
            },
        )
    };
}

#[macro_export]
macro_rules! rule_non_control_character {
    ($name:ident) => {
        $crate::Rule::new(
            stringify!($name),
            |obj: &Self, error: &mut $crate::error::ValidationError| {
                use $crate::wrappers::rules::SomeOrStringWrapper;
                if $crate::wrappers::rules::SomeOrString(&obj.$name)
                    .0
                    .non_control_character()
                {
                    error.add("non_control_character");
                }
            },
        )
    };
}

#[macro_export]
macro_rules! rule_ip {
    ($name:ident) => {
        $crate::Rule::new(
            stringify!($name),
            |obj: &Self, error: &mut $crate::error::ValidationError| {
                use $crate::wrappers::rules::SomeOrStringWrapper;
                if $crate::wrappers::rules::SomeOrString(&obj.$name).0.ip() {
                    error.add("ip");
                }
            },
        )
    };
}

#[macro_export]
macro_rules! rule_ip_v4 {
    ($name:ident) => {
        $crate::Rule::new(
            stringify!($name),
            |obj: &Self, error: &mut $crate::error::ValidationError| {
                use $crate::wrappers::rules::SomeOrStringWrapper;
                if $crate::wrappers::rules::SomeOrString(&obj.$name).0.ip_v4() {
                    error.add("ip_v4");
                }
            },
        )
    };
}

#[macro_export]
macro_rules! rule_ip_v6 {
    ($name:ident) => {
        $crate::Rule::new(
            stringify!($name),
            |obj: &Self, error: &mut $crate::error::ValidationError| {
                use $crate::wrappers::rules::SomeOrStringWrapper;
                if $crate::wrappers::rules::SomeOrString(&obj.$name).0.ip_v6() {
                    error.add("ip_v6");
                }
            },
        )
    };
}

#[macro_export]
macro_rules! rule_credit_card {
    ($name:ident) => {
        $crate::Rule::new(
            stringify!($name),
            |obj: &Self, error: &mut $crate::error::ValidationError| {
                use $crate::wrappers::rules::SomeOrStringWrapper;
                if $crate::wrappers::rules::SomeOrString(&obj.$name)
                    .0
                    .credit_card()
                {
                    error.add("credit_card");
                }
            },
        )
    };
}

#[macro_export]
macro_rules! rule_contains {
    ($name:ident, $needle:expr) => {
        $crate::Rule::new(
            stringify!($name),
            |obj: &Self, error: &mut $crate::error::ValidationError| {
                use $crate::wrappers::rules::SomeOrStringWrapper;
                if $crate::wrappers::rules::SomeOrString(&obj.$name)
                    .0
                    .rule_contains($needle)
                {
                    error.add(&format!("contains:{}", $needle));
                }
            },
        )
    };
}

#[macro_export]
macro_rules! rule_equalt_to {
    ($name:ident, $second_name:ident) => {
        $crate::Rule::new(
            stringify!($name),
            |obj: &Self, error: &mut $crate::error::ValidationError| {
                if obj.$name != obj.$second_name {
                    error.add(
                        "equalt_to:{}!={}",
                        stringify!($name),
                        stringify!($second_name),
                    );
                }
            },
        )
    };
}

#[macro_export]
macro_rules! rule_in {
    ($name:ident, $items:expr) => {
        $crate::Rule::new(
            stringify!($name),
            |obj: &Self, error: &mut $crate::error::ValidationError| {
                use $crate::wrappers::rules::SomeOrStringWrapper;
                if $crate::wrappers::rules::SomeOrString(&obj.$name)
                    .0
                    .r#in($items)
                {
                    let string_items = $items
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>();
                    error.add(&format!("in:{}", string_items.join(",")));
                }
            },
        )
    };
}

#[macro_export]
macro_rules! rule_lenght_min {
    ($name:ident, $min:expr) => {
        $crate::Rule::new(
            stringify!($name),
            |obj: &Self, error: &mut $crate::error::ValidationError| {
                use $crate::wrappers::rules::SomeOrStringWrapper;
                if $crate::wrappers::rules::SomeOrString(&obj.$name)
                    .0
                    .lenght_min($min)
                {
                    error.add(&format!("lenght_min:{}", $min));
                }
            },
        )
    };
}

#[macro_export]
macro_rules! rule_lenght_max {
    ($name:ident, $max:expr) => {
        $crate::Rule::new(
            stringify!($name),
            |obj: &Self, error: &mut $crate::error::ValidationError| {
                use $crate::wrappers::rules::SomeOrStringWrapper;
                if $crate::wrappers::rules::SomeOrString(&obj.$name)
                    .0
                    .lenght_max($max)
                {
                    error.add(&format!("lenght_max:{}", $max));
                }
            },
        )
    };
}

#[macro_export]
macro_rules! rule_lenght_eq {
    ($name:ident, $eq:expr) => {
        $crate::Rule::new(
            stringify!($name),
            |obj: &Self, error: &mut $crate::error::ValidationError| {
                use $crate::wrappers::rules::SomeOrStringWrapper;
                if $crate::wrappers::rules::SomeOrString(&obj.$name)
                    .0
                    .lenght_eq($eq)
                {
                    error.add(&format!("lenght_eq:{}", $eq));
                }
            },
        )
    };
}

#[macro_export]
macro_rules! rule_lenght_ne {
    ($name:ident, $ne:expr) => {
        $crate::Rule::new(
            stringify!($name),
            |obj: &Self, error: &mut $crate::error::ValidationError| {
                use $crate::wrappers::rules::SomeOrStringWrapper;
                if !$crate::wrappers::rules::SomeOrString(&obj.$name)
                    .0
                    .lenght_eq($ne)
                {
                    error.add(&format!("lenght_ne:{}", $ne));
                }
            },
        )
    };
}

#[macro_export]
macro_rules! rule_range {
    ($name:ident, $min:expr, $max:expr) => {
        $crate::Rule::new(
            stringify!($name),
            |obj: &Self, error: &mut $crate::error::ValidationError| {
                use $crate::wrappers::rules::SomeOrNumberWrapper;
                if $crate::wrappers::rules::SomeOrNumber(&obj.$name)
                    .0
                    .range($min, $max)
                {
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
            },
        )
    };

    ($name:ident, $min:expr) => {
        $crate::Rule::new(
            stringify!($name),
            |obj: &Self, error: &mut $crate::error::ValidationError| {
                use $crate::wrappers::rules::SomeOrNumberWrapper;
                if $crate::wrappers::rules::SomeOrNumber(&obj.$name)
                    .0
                    .range(Some($min), None)
                {
                    error.add(format!("range:{}-{}", $min, "inf"));
                }
            },
        )
    };
}
