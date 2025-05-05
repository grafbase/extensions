use std::{
    borrow::Cow,
    fmt::{self, Write},
};

use super::{directive::Directive, field::Field};

pub struct Type<'a> {
    name: Cow<'a, str>,
    directives: Vec<Directive<'a>>,
    fields: Vec<Field<'a>>,
    description: Option<Cow<'a, str>>,
}

impl<'a> Type<'a> {
    pub fn new(name: impl Into<Cow<'a, str>>) -> Type<'a> {
        Type {
            name: name.into(),
            directives: Vec::new(),
            fields: Vec::new(),
            description: None,
        }
    }

    pub fn has_fields(&self) -> bool {
        !self.fields.is_empty()
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

impl fmt::Display for Type<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(description) = &self.description {
            writeln!(f, r#"""""#)?;
            writeln!(f, "{description}")?;
            writeln!(f, r#"""""#)?;
        }

        write!(f, "type {}", self.name)?;

        if !self.directives.is_empty() {
            f.write_char('\n')?;

            for directive in self.directives.iter() {
                write!(f, "  {directive}")?;

                f.write_char('\n')?;
            }
        } else {
            f.write_char(' ')?;
        }

        f.write_str("{\n")?;

        if !self.fields.is_empty() {
            for field in self.fields.iter() {
                writeln!(f, "{field}")?;
            }
        }

        f.write_str("}")
    }
}
