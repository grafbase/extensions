use std::{
    borrow::Cow,
    fmt::{self},
};

use super::{directive::Directive, field::Field};

pub struct InputType<'a> {
    name: Cow<'a, str>,
    directives: Vec<Directive<'a>>,
    fields: Vec<Field<'a>>,
}

impl<'a> InputType<'a> {
    pub fn new(name: impl Into<Cow<'a, str>>) -> InputType<'a> {
        InputType {
            name: name.into(),
            directives: Vec::new(),
            fields: Vec::new(),
        }
    }

    pub fn push_directive(&mut self, directive: Directive<'a>) {
        self.directives.push(directive);
    }

    pub fn push_field(&mut self, field: Field<'a>) {
        self.fields.push(field);
    }
}

impl fmt::Display for InputType<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "input {}", self.name)?;

        if !self.directives.is_empty() {
            for directive in self.directives.iter() {
                write!(f, " {directive} ")?;
            }
        }

        f.write_str("{\n")?;

        if !self.fields.is_empty() {
            for field in self.fields.iter() {
                writeln!(f, "  {field}")?;
            }
        }

        f.write_str("}")
    }
}
