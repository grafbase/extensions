use std::{borrow::Cow, fmt};

use super::directive::{Argument, Directive};

pub struct Field<'a> {
    name: Cow<'a, str>,
    r#type: Cow<'a, str>,
    directives: Vec<Directive<'a>>,
    arguments: Vec<Argument<'a>>,
    description: Option<Cow<'a, str>>,
}

impl<'a> Field<'a> {
    pub fn new(name: impl Into<Cow<'a, str>>, r#type: impl Into<Cow<'a, str>>) -> Self {
        Self {
            name: name.into(),
            r#type: r#type.into(),
            directives: Vec::new(),
            arguments: Vec::new(),
            description: None,
        }
    }

    pub fn push_directive(&mut self, directive: Directive<'a>) {
        self.directives.push(directive);
    }

    pub fn push_argument(&mut self, argument: Argument<'a>) {
        self.arguments.push(argument);
    }

    pub fn set_description(&mut self, description: impl Into<Cow<'a, str>>) {
        self.description = Some(description.into());
    }
}

impl fmt::Display for Field<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(description) = &self.description {
            writeln!(f, "  # {description}")?;
        }

        write!(f, "  {}", self.name)?;

        if !self.arguments.is_empty() {
            write!(f, "(")?;

            for (i, argument) in self.arguments.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }

                write!(f, "{}", argument)?;
            }

            write!(f, "): {}", self.r#type)?;
        } else {
            write!(f, ": {}", self.r#type)?;
        }

        for directive in &self.directives {
            write!(f, " {}", directive)?;
        }

        Ok(())
    }
}
