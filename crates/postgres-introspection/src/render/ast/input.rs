use std::{
    borrow::Cow,
    fmt::{self},
};

use super::{directive::Directive, field::Field};

pub struct InputType<'a> {
    name: Cow<'a, str>,
    directives: Vec<Directive<'a>>,
    fields: Vec<Field<'a>>,
    description: Option<Cow<'a, str>>,
}

impl<'a> InputType<'a> {
    pub fn new(name: impl Into<Cow<'a, str>>) -> InputType<'a> {
        InputType {
            name: name.into(),
            directives: Vec::new(),
            fields: Vec::new(),
            description: None,
        }
    }

    pub fn push_directive(&mut self, directive: Directive<'a>) {
        self.directives.push(directive);
    }

    pub fn push_field(&mut self, field: Field<'a>) {
        self.fields.push(field);
    }

    pub fn set_description(&mut self, description: impl Into<Cow<'a, str>>) {
        self.description = Some(description.into());
    }
}

impl fmt::Display for InputType<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(description) = self.description.as_deref() {
            writeln!(f, "# {description}")?;
        }

        write!(f, "input {}", self.name)?;

        if !self.directives.is_empty() {
            for directive in self.directives.iter() {
                write!(f, " {directive} ")?;
            }
        }

        if self.directives.is_empty() {
            write!(f, " ")?;
        }

        f.write_str("{\n")?;

        if self.fields.is_empty() {
            writeln!(f, "  _: Boolean")?;
        } else {
            for field in self.fields.iter() {
                writeln!(f, "{field}")?;
            }
        }

        f.write_str("}")?;

        Ok(())
    }
}
