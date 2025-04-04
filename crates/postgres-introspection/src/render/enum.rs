use std::fmt;

use super::directive::Directive;

pub struct Enum<'a> {
    name: &'a str,
    directives: Vec<Directive<'a>>,
    variants: Vec<EnumVariant<'a>>,
}

impl<'a> Enum<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            directives: Vec::new(),
            variants: Vec::new(),
        }
    }

    pub fn push_directive(&mut self, directive: Directive<'a>) {
        self.directives.push(directive);
    }

    pub fn push_variant(&mut self, value: EnumVariant<'a>) {
        self.variants.push(value);
    }
}

impl fmt::Display for Enum<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "enum {}", self.name)?;

        for directive in &self.directives {
            write!(f, " {}", directive)?;
        }

        f.write_str(" {")?;

        for variant in &self.variants {
            write!(f, "\n  {}", variant)?;
        }

        write!(f, "\n}}")
    }
}

pub struct EnumVariant<'a> {
    name: &'a str,
    directives: Vec<Directive<'a>>,
}

impl<'a> EnumVariant<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            directives: Vec::new(),
        }
    }

    pub fn push_directive(&mut self, directive: Directive<'a>) {
        self.directives.push(directive);
    }
}

impl fmt::Display for EnumVariant<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.name.fmt(f)?;

        for directive in &self.directives {
            write!(f, " {}", directive)?;
        }

        Ok(())
    }
}
