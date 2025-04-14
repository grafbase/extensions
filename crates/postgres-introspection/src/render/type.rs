use std::fmt::{self, Write};

use super::{directive::Directive, field::Field};

pub struct Type<'a> {
    name: &'a str,
    directives: Vec<Directive<'a>>,
    fields: Vec<Field<'a>>,
}

impl<'a> Type<'a> {
    pub fn new(name: &'a str) -> Type<'a> {
        Type {
            name,
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

impl fmt::Display for Type<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "type {}", self.name)?;

        if !self.directives.is_empty() {
            f.write_char('\n')?;

            for directive in self.directives.iter() {
                write!(f, "  {directive}")?;

                f.write_char('\n')?;
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
