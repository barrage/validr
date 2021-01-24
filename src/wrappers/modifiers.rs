pub trait SomeOrStringWrapper {
    fn m_trim(self) -> Self;
    fn m_lowercase(self) -> Self;
    fn m_uppercase(self) -> Self;
    fn m_capitalize(self) -> Self;
}

impl SomeOrStringWrapper for Option<String> {
    fn m_trim(self) -> Self {
        if let Some(v) = self {
            Some(v.clone().trim().to_string())
        } else {
            None
        }
    }
    fn m_lowercase(self) -> Self {
        if let Some(v) = self {
            Some(v.clone().to_lowercase())
        } else {
            None
        }
    }
    fn m_uppercase(self) -> Self {
        if let Some(v) = self {
            Some(v.clone().to_uppercase())
        } else {
            None
        }
    }
    fn m_capitalize(self) -> Self {
        if let Some(v) = self {
            let mut c = v.chars();

            match c.next() {
                None => None,
                Some(f) => Some(f.to_uppercase().collect::<String>() + c.as_str()),
            }
        } else {
            None
        }
    }
}

impl SomeOrStringWrapper for String {
    fn m_trim(self) -> Self {
        self.trim().to_string()
    }
    fn m_lowercase(self) -> Self {
        self.to_lowercase()
    }
    fn m_uppercase(self) -> Self {
        self.to_uppercase()
    }
    fn m_capitalize(self) -> Self {
        let mut c = self.chars();

        match c.next() {
            None => self,
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }
}

pub struct SomeOrString<T: SomeOrStringWrapper>(pub T);
