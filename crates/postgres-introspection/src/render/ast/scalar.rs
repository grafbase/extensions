use std::borrow::Cow;
use std::fmt;

pub struct Scalar<'a> {
    name: &'a str,
    description: Option<Cow<'a, str>>,
}

impl<'a> Scalar<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            description: None,
        }
    }

    pub fn set_description(&mut self, description: impl Into<Cow<'a, str>>) {
        self.description = Some(description.into());
    }
}

impl fmt::Display for Scalar<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(description) = self.description.as_deref() {
            writeln!(f, r#"""""#)?;
            writeln!(f, "{description}")?;
            writeln!(f, r#"""""#)?;
        }

        writeln!(f, "scalar {}", self.name)
    }
}
