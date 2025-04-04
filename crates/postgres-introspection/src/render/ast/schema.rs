use std::fmt;

use super::{directive::Directive, r#enum::Enum, input::InputType, scalar::Scalar, r#type::Type};

#[derive(Default)]
pub struct Schema<'a> {
    directives: Vec<Directive<'a>>,
    input_types: Vec<InputType<'a>>,
    types: Vec<Type<'a>>,
    enums: Vec<Enum<'a>>,
    scalars: Vec<Scalar<'a>>,
}

impl<'a> Schema<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_directive(&mut self, directive: Directive<'a>) {
        self.directives.push(directive);
    }

    pub fn push_input(&mut self, input_type: InputType<'a>) {
        self.input_types.push(input_type);
    }

    pub fn push_type(&mut self, r#type: Type<'a>) {
        self.types.push(r#type);
    }

    pub fn push_enum(&mut self, r#enum: Enum<'a>) {
        self.enums.push(r#enum);
    }

    pub fn push_scalar(&mut self, scalar: Scalar<'a>) {
        self.scalars.push(scalar);
    }
}

impl fmt::Display for Schema<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.directives.is_empty() {
            writeln!(f, "extend schema")?;

            for directive in &self.directives {
                writeln!(f, "  {}", directive)?;
            }
        }

        writeln!(f)?;

        for scalar in &self.scalars {
            scalar.fmt(f)?;
            writeln!(f)?;
        }

        for r#enum in &self.enums {
            r#enum.fmt(f)?;
            writeln!(f)?;
            writeln!(f)?;
        }

        for input in &self.input_types {
            input.fmt(f)?;
            writeln!(f)?;
            writeln!(f)?;
        }

        for r#type in &self.types {
            r#type.fmt(f)?;
            writeln!(f)?;
            writeln!(f)?;
        }

        Ok(())
    }
}
