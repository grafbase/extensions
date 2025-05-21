use std::fmt::Write;
use std::{borrow::Cow, fmt};

use indenter::indented;

use super::directive::{Argument, Directive};

#[derive(Debug)]
pub struct Field<'a> {
    pub(super) name: Cow<'a, str>,
    pub(super) r#type: Cow<'a, str>,
    pub(super) directives: Vec<Directive<'a>>,
    pub(super) arguments: Vec<Argument<'a>>,
    pub(super) description: Option<Cow<'a, str>>,
    pub(super) render_multiline: bool,
}

impl<'a> Field<'a> {
    pub fn new(name: impl Into<Cow<'a, str>>, r#type: impl Into<Cow<'a, str>>) -> Self {
        Self {
            name: name.into(),
            r#type: r#type.into(),
            directives: Vec::new(),
            arguments: Vec::new(),
            description: None,
            render_multiline: false,
        }
    }

    pub fn push_directive(&mut self, directive: Directive<'a>) {
        self.directives.push(directive);
    }

    pub fn push_argument(&mut self, argument: Argument<'a>) {
        if argument.has_description() {
            self.render_multiline = true;
        }

        self.arguments.push(argument);
    }

    pub fn set_description(&mut self, description: impl Into<Cow<'a, str>>) {
        self.description = Some(description.into());
    }
}

impl fmt::Display for Field<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(description) = &self.description {
            let indent = "  ";
            writeln!(indented(f).with_str(indent), r#"""""#)?;
            writeln!(indented(f).with_str(indent), "{description}")?;
            writeln!(indented(f).with_str(indent), r#"""""#)?;
        }

        write!(f, "  {}", self.name)?;

        if !self.arguments.is_empty() {
            if self.render_multiline {
                writeln!(f, "(")?;
            } else {
                write!(f, "(")?;
            }

            for (i, argument) in self.arguments.iter().enumerate() {
                if i > 0 && !self.render_multiline {
                    write!(f, ", ")?;
                }

                if self.render_multiline {
                    writeln!(indented(f).with_str("    "), "{},", argument)?;
                } else {
                    write!(f, "{}", argument)?;
                }
            }

            if self.render_multiline {
                write!(f, "  ): {}", self.r#type)?;
            } else {
                write!(f, "): {}", self.r#type)?;
            }
        } else {
            write!(f, ": {}", self.r#type)?;
        }

        for directive in &self.directives {
            write!(f, " {}", directive)?;
        }

        Ok(())
    }
}
