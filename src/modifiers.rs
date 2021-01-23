#[macro_export]
macro_rules! modifier_trim {
    ($name:ident) => {
        $crate::Modifier::new(stringify!($name), |obj: &mut Self| {
            if let Some(v) = &obj.$name {
                obj.$name = Some(v.trim().to_string());
            }
        })
    };
}

#[macro_export]
macro_rules! modifier_lowercase {
    ($name:ident) => {
        $crate::Modifier::new(stringify!($name), |obj: &mut Self| {
            if let Some(v) = &obj.$name {
                obj.$name = Some(v.to_lowercase());
            }
        })
    };
}

#[macro_export]
macro_rules! modifier_uppercase {
    ($name:ident) => {
        $crate::Modifier::new(stringify!($name), |obj: &mut Self| {
            if let Some(v) = &obj.$name {
                obj.$name = Some(v.to_uppercase());
            }
        })
    };
}

#[macro_export]
macro_rules! modifier_capitalize {
    ($name:ident) => {
        $crate::Modifier::new(stringify!($name), |obj: &mut Self| {
            if let Some(v) = &obj.$name {
                let mut c = v.chars();

                obj.$name = match c.next() {
                    None => Some(String::new()),
                    Some(f) => Some(f.to_uppercase().collect::<String>() + c.as_str()),
                };
            }
        })
    };
}
