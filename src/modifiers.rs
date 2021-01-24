#[macro_export]
macro_rules! modifier_trim {
    ($name:ident) => {
        $crate::Modifier::new(stringify!($name), |obj: &mut Self| {
            use $crate::wrappers::modifiers::SomeOrStringWrapper;
            obj.$name = $crate::wrappers::modifiers::SomeOrString(obj.$name.clone())
                .0
                .m_trim()
        })
    };
}

#[macro_export]
macro_rules! modifier_lowercase {
    ($name:ident) => {
        $crate::Modifier::new(stringify!($name), |obj: &mut Self| {
            use $crate::wrappers::modifiers::SomeOrStringWrapper;
            obj.$name = $crate::wrappers::modifiers::SomeOrString(obj.$name.clone())
                .0
                .m_lowercase()
        })
    };
}

#[macro_export]
macro_rules! modifier_uppercase {
    ($name:ident) => {
        $crate::Modifier::new(stringify!($name), |obj: &mut Self| {
            use $crate::wrappers::modifiers::SomeOrStringWrapper;
            obj.$name = $crate::wrappers::modifiers::SomeOrString(obj.$name.clone())
                .0
                .m_uppercase()
        })
    };
}

#[macro_export]
macro_rules! modifier_capitalize {
    ($name:ident) => {
        $crate::Modifier::new(stringify!($name), |obj: &mut Self| {
            use $crate::wrappers::modifiers::SomeOrStringWrapper;
            obj.$name = $crate::wrappers::modifiers::SomeOrString(obj.$name.clone())
                .0
                .m_capitalize()
        })
    };
}
