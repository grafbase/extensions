use std::{borrow::Cow, fmt};

use super::directive::Directive;

pub struct Enum<'a> {
    pub(super) name: &'a str,
    pub(super) directives: Vec<Directive<'a>>,
    pub(super) variants: Vec<EnumVariant<'a>>,
    pub(super) description: Option<Cow<'a, str>>,
}

impl<'a> Enum<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            directives: Vec::new(),
            variants: Vec::new(),
            description: None,
        }
    }

    pub fn push_directive(&mut self, directive: Directive<'a>) {
        self.directives.push(directive);
    }

    pub fn push_variant(&mut self, value: EnumVariant<'a>) {
        self.variants.push(value);
    }

    pub fn set_description(&mut self, description: impl Into<Cow<'a, str>>) {
        self.description = Some(description.into());
    }
}

impl fmt::Display for Enum<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(description) = self.description.as_deref() {
            writeln!(f, r#"""""#)?;
            writeln!(f, "{description}")?;
            writeln!(f, r#"""""#)?;
        }

        write!(f, "enum {}", self.name)?;

        for directive in &self.directives {
            write!(f, " {}", directive)?;
        }

        f.write_str(" {")?;

        for variant in &self.variants {
            write!(f, "\n{}", variant)?;
        }

        write!(f, "\n}}")
    }
}

pub struct EnumVariant<'a> {
    name: &'a str,
    directives: Vec<Directive<'a>>,
    description: Option<Cow<'a, str>>,
}

impl<'a> EnumVariant<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            directives: Vec::new(),
            description: None,
        }
    }

    pub fn push_directive(&mut self, directive: Directive<'a>) {
        self.directives.push(directive);
    }

    pub fn set_description(&mut self, description: impl Into<Cow<'a, str>>) {
        self.description = Some(description.into());
    }
}

impl fmt::Display for EnumVariant<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(description) = self.description.as_deref() {
            writeln!(f, r#"  """"#)?;
            writeln!(f, "  {description}")?;
            writeln!(f, r#"  """"#)?;
        }

        write!(f, "  {}", self.name)?;

        for directive in &self.directives {
            write!(f, " {}", directive)?;
        }

        Ok(())
    }
}
