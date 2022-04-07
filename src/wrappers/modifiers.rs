pub trait SomeOrStringWrapper {
    fn m_trim(self) -> Self;
    fn m_lowercase(self) -> Self;
    fn m_uppercase(self) -> Self;
    fn m_capitalize(self) -> Self;
}

impl SomeOrStringWrapper for Option<String> {
    fn m_trim(self) -> Self {
        self.map(|v| v.trim().to_string())
    }
    fn m_lowercase(self) -> Self {
        self.map(|v| v.to_lowercase())
    }
    fn m_uppercase(self) -> Self {
        self.map(|v| v.to_uppercase())
    }
    fn m_capitalize(self) -> Self {
        if let Some(v) = self {
            let mut c = v.chars();

            c.next()
                .map(|f| f.to_uppercase().collect::<String>() + c.as_str())
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
