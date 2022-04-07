use std::string::ToString;

pub trait SomeOrStringWrapper {
    fn required(&self) -> bool;
    fn accepted(&self) -> bool;
    fn email(&self) -> bool;
    fn url(&self) -> bool;
    fn phone(&self) -> bool;
    fn non_control_character(&self) -> bool;
    fn ip(&self) -> bool;
    fn ip_v4(&self) -> bool;
    fn ip_v6(&self) -> bool;
    fn credit_card(&self) -> bool;
    fn rule_contains(&self, needle: String) -> bool;
    fn r#in<B: ToString>(&self, haystack: Vec<B>) -> bool;
    fn length_min(&self, min: usize) -> bool;
    fn length_max(&self, max: usize) -> bool;
    fn length_eq(&self, eq: usize) -> bool;
}

impl<A> SomeOrStringWrapper for &Option<A>
where
    A: ToString + Clone,
{
    fn required(&self) -> bool {
        if self.is_none() {
            true
        } else if let Some(v) = self {
            v.to_string().is_empty()
        } else {
            false
        }
    }
    fn accepted(&self) -> bool {
        if self.is_none() {
            true
        } else if let Some(v) = self {
            v.to_string() != "true"
        } else {
            true
        }
    }
    fn email(&self) -> bool {
        if let Some(v) = self {
            !crate::helpers::email::validate_email(v.to_string())
        } else {
            false
        }
    }
    fn url(&self) -> bool {
        if let Some(v) = self {
            !crate::helpers::url::validate_url(v.to_string())
        } else {
            false
        }
    }
    fn phone(&self) -> bool {
        if let Some(v) = self {
            !crate::helpers::phone::validate_phone(v.to_string())
        } else {
            false
        }
    }
    fn non_control_character(&self) -> bool {
        if let Some(v) = self {
            !crate::helpers::non_control_character::validate_non_control_character(v.to_string())
        } else {
            false
        }
    }
    fn ip(&self) -> bool {
        if let Some(v) = self {
            !crate::helpers::ip::validate_ip(v.to_string())
        } else {
            false
        }
    }
    fn ip_v4(&self) -> bool {
        if let Some(v) = self {
            !crate::helpers::ip::validate_ip_v4(v.to_string())
        } else {
            false
        }
    }
    fn ip_v6(&self) -> bool {
        if let Some(v) = self {
            !crate::helpers::ip::validate_ip_v6(v.to_string())
        } else {
            false
        }
    }
    fn credit_card(&self) -> bool {
        if let Some(v) = self {
            !crate::helpers::card::validate_credit_card(v.to_string())
        } else {
            false
        }
    }
    fn rule_contains(&self, needle: String) -> bool {
        if let Some(v) = self {
            !v.to_string().contains(&needle)
        } else {
            false
        }
    }
    fn r#in<B>(&self, haystack: Vec<B>) -> bool
    where
        B: ToString,
    {
        if let Some(v) = self {
            !haystack
                .iter()
                .map(|x| x.to_string())
                .any(|x| x == v.to_string())
        } else {
            false
        }
    }
    fn length_min(&self, min: usize) -> bool {
        if let Some(v) = self {
            v.to_string().len() < min
        } else {
            false
        }
    }
    fn length_max(&self, max: usize) -> bool {
        if let Some(v) = self {
            v.to_string().len() > max
        } else {
            false
        }
    }
    fn length_eq(&self, eq: usize) -> bool {
        if let Some(v) = self {
            v.to_string().len() != eq
        } else {
            false
        }
    }
}

impl SomeOrStringWrapper for &String {
    fn required(&self) -> bool {
        self.is_empty()
    }
    fn accepted(&self) -> bool {
        *self == "true"
    }
    fn email(&self) -> bool {
        crate::helpers::email::validate_email(*self)
    }
    fn url(&self) -> bool {
        crate::helpers::url::validate_url(*self)
    }
    fn phone(&self) -> bool {
        crate::helpers::phone::validate_phone(*self)
    }
    fn non_control_character(&self) -> bool {
        crate::helpers::non_control_character::validate_non_control_character(*self)
    }
    fn ip(&self) -> bool {
        crate::helpers::ip::validate_ip(*self)
    }
    fn ip_v4(&self) -> bool {
        crate::helpers::ip::validate_ip_v4(*self)
    }
    fn ip_v6(&self) -> bool {
        crate::helpers::ip::validate_ip_v6(*self)
    }
    fn credit_card(&self) -> bool {
        crate::helpers::card::validate_credit_card(*self)
    }
    fn rule_contains(&self, needle: String) -> bool {
        self.contains(&needle)
    }
    fn r#in<B>(&self, haystack: Vec<B>) -> bool
    where
        B: ToString,
    {
        !haystack
            .iter()
            .map(|x| x.to_string())
            .any(|x| x == self.to_string())
    }
    fn length_min(&self, min: usize) -> bool {
        self.len() < min
    }
    fn length_max(&self, max: usize) -> bool {
        self.len() > max
    }
    fn length_eq(&self, eq: usize) -> bool {
        self.len() != eq
    }
}

impl SomeOrStringWrapper for &bool {
    fn required(&self) -> bool {
        false
    }
    fn accepted(&self) -> bool {
        *self != &true
    }
    fn email(&self) -> bool {
        false
    }
    fn url(&self) -> bool {
        false
    }
    fn phone(&self) -> bool {
        false
    }
    fn non_control_character(&self) -> bool {
        false
    }
    fn ip(&self) -> bool {
        false
    }
    fn ip_v4(&self) -> bool {
        false
    }
    fn ip_v6(&self) -> bool {
        false
    }
    fn credit_card(&self) -> bool {
        false
    }
    fn rule_contains(&self, _needle: String) -> bool {
        false
    }
    fn r#in<B>(&self, _haystack: Vec<B>) -> bool
    where
        B: ToString,
    {
        false
    }
    fn length_min(&self, _min: usize) -> bool {
        false
    }
    fn length_max(&self, _max: usize) -> bool {
        false
    }
    fn length_eq(&self, _eq: usize) -> bool {
        false
    }
}

pub struct SomeOrString<T: SomeOrStringWrapper>(pub T);

pub trait SomeOrNumberWrapper {
    fn range<T: Into<f64>, V: Into<f64>>(&self, min: Option<T>, max: Option<V>) -> bool;
}

pub struct SomeOrNumber<T: SomeOrNumberWrapper>(pub T);

impl<A> SomeOrNumberWrapper for &Option<A>
where
    A: Into<f64> + PartialOrd + Clone,
{
    fn range<T: Into<f64>, V: Into<f64>>(&self, min: Option<T>, max: Option<V>) -> bool {
        if let Some(v) = self {
            let value: f64 = v.clone().into();
            if let Some(max) = max {
                if value > max.into() {
                    return true;
                }
            }

            if let Some(min) = min {
                if value < min.into() {
                    return true;
                }
            }

            false
        } else {
            false
        }
    }
}

impl SomeOrNumberWrapper for usize {
    fn range<T: Into<f64>, V: Into<f64>>(&self, min: Option<T>, max: Option<V>) -> bool {
        if let Some(max) = max {
            if (*self as f64) > max.into() {
                return true;
            }
        }

        if let Some(min) = min {
            if (*self as f64) < min.into() {
                return true;
            }
        }

        false
    }
}

impl SomeOrNumberWrapper for u8 {
    fn range<T: Into<f64>, V: Into<f64>>(&self, min: Option<T>, max: Option<V>) -> bool {
        if let Some(max) = max {
            if (*self as f64) > max.into() {
                return true;
            }
        }

        if let Some(min) = min {
            if (*self as f64) < min.into() {
                return true;
            }
        }

        false
    }
}

impl SomeOrNumberWrapper for u16 {
    fn range<T: Into<f64>, V: Into<f64>>(&self, min: Option<T>, max: Option<V>) -> bool {
        if let Some(max) = max {
            if (*self as f64) > max.into() {
                return true;
            }
        }

        if let Some(min) = min {
            if (*self as f64) < min.into() {
                return true;
            }
        }

        false
    }
}

impl SomeOrNumberWrapper for u32 {
    fn range<T: Into<f64>, V: Into<f64>>(&self, min: Option<T>, max: Option<V>) -> bool {
        if let Some(max) = max {
            if (*self as f64) > max.into() {
                return true;
            }
        }

        if let Some(min) = min {
            if (*self as f64) < min.into() {
                return true;
            }
        }

        false
    }
}

impl SomeOrNumberWrapper for u64 {
    fn range<T: Into<f64>, V: Into<f64>>(&self, min: Option<T>, max: Option<V>) -> bool {
        if let Some(max) = max {
            if (*self as f64) > max.into() {
                return true;
            }
        }

        if let Some(min) = min {
            if (*self as f64) < min.into() {
                return true;
            }
        }

        false
    }
}

impl SomeOrNumberWrapper for isize {
    fn range<T: Into<f64>, V: Into<f64>>(&self, min: Option<T>, max: Option<V>) -> bool {
        if let Some(max) = max {
            if (*self as f64) > max.into() {
                return true;
            }
        }

        if let Some(min) = min {
            if (*self as f64) < min.into() {
                return true;
            }
        }

        false
    }
}

impl SomeOrNumberWrapper for i8 {
    fn range<T: Into<f64>, V: Into<f64>>(&self, min: Option<T>, max: Option<V>) -> bool {
        if let Some(max) = max {
            if (*self as f64) > max.into() {
                return true;
            }
        }

        if let Some(min) = min {
            if (*self as f64) < min.into() {
                return true;
            }
        }

        false
    }
}

impl SomeOrNumberWrapper for i16 {
    fn range<T: Into<f64>, V: Into<f64>>(&self, min: Option<T>, max: Option<V>) -> bool {
        if let Some(max) = max {
            if (*self as f64) > max.into() {
                return true;
            }
        }

        if let Some(min) = min {
            if (*self as f64) < min.into() {
                return true;
            }
        }

        false
    }
}

impl SomeOrNumberWrapper for i32 {
    fn range<T: Into<f64>, V: Into<f64>>(&self, min: Option<T>, max: Option<V>) -> bool {
        if let Some(max) = max {
            if (*self as f64) > max.into() {
                return true;
            }
        }

        if let Some(min) = min {
            if (*self as f64) < min.into() {
                return true;
            }
        }

        false
    }
}

impl SomeOrNumberWrapper for i64 {
    fn range<T: Into<f64>, V: Into<f64>>(&self, min: Option<T>, max: Option<V>) -> bool {
        if let Some(max) = max {
            if (*self as f64) > max.into() {
                return true;
            }
        }

        if let Some(min) = min {
            if (*self as f64) < min.into() {
                return true;
            }
        }

        false
    }
}

impl SomeOrNumberWrapper for f32 {
    fn range<T: Into<f64>, V: Into<f64>>(&self, min: Option<T>, max: Option<V>) -> bool {
        if let Some(max) = max {
            if (*self as f64) > max.into() {
                return true;
            }
        }

        if let Some(min) = min {
            if (*self as f64) < min.into() {
                return true;
            }
        }

        false
    }
}

impl SomeOrNumberWrapper for f64 {
    fn range<T: Into<f64>, V: Into<f64>>(&self, min: Option<T>, max: Option<V>) -> bool {
        if let Some(max) = max {
            if *self > max.into() {
                return true;
            }
        }

        if let Some(min) = min {
            if *self < min.into() {
                return true;
            }
        }

        false
    }
}
