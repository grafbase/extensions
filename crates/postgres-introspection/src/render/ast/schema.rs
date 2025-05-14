use std::{cmp::Ordering, collections::HashSet, fmt};

use super::{
    directive::{ArgumentValue, Directive},
    r#enum::Enum,
    input::InputType,
    scalar::Scalar,
    r#type::Type,
};

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

    /// Kind of a yolo version of unused types check. But hey, it works. If somebody has lots of time
    /// in their hands, would be great to write this with cynic ;)
    pub fn remove_unused_types(&mut self) {
        let mut used_types = HashSet::new();

        // First pass: collect type references from regular types
        for r#type in &self.types {
            for field in &r#type.fields {
                let type_name = field.r#type.replace("[", "").replace("]", "").replace("!", "");
                used_types.insert(type_name);

                // Process any arguments
                for argument in &field.arguments {
                    track_argument_inputs(&mut used_types, &argument.value);
                }
            }
        }

        // Keep resolving dependencies until we reach a fixed point
        let mut size_before = 0;
        while size_before != used_types.len() {
            size_before = used_types.len();

            // Process input types that are already marked as used
            for input in &self.input_types {
                if used_types.contains(input.name.as_ref()) {
                    for field in &input.fields {
                        let type_name = field.r#type.replace("[", "").replace("]", "").replace("!", "");
                        used_types.insert(type_name);

                        // Process any arguments
                        for argument in &field.arguments {
                            track_argument_inputs(&mut used_types, &argument.value);
                        }
                    }
                }
            }
        }

        // Filter scalars to only keep those that are used
        let mut scalars = Vec::new();
        for scalar in self.scalars.drain(..) {
            if used_types.contains(scalar.name) {
                scalars.push(scalar);
            }
        }

        self.scalars = scalars;

        // Filter input types to only keep those that are used
        let mut input_types = Vec::new();
        for input in self.input_types.drain(..) {
            if used_types.contains(input.name.as_ref()) {
                input_types.push(input);
            }
        }
        self.input_types = input_types;

        self.scalars.sort_by_key(|scalar| scalar.name);
        self.enums.sort_by_key(|r#enum| r#enum.name);
        self.input_types.sort_by(|a, b| a.name.cmp(&b.name));

        self.types.sort_by(|a, b| match (a.name.as_ref(), b.name.as_ref()) {
            ("Query", "Mutation") => Ordering::Less,
            ("Mutation", "Query") => Ordering::Greater,
            ("Query", _) => Ordering::Greater,
            ("Mutation", _) => Ordering::Greater,
            _ => a.name.cmp(&b.name),
        });
    }
}

fn track_argument_inputs<'a>(used_inputs: &mut HashSet<String>, value: &'a ArgumentValue<'a>) {
    match value {
        ArgumentValue::Constant(constant) => {
            let type_name = constant.replace("[", "").replace("]", "").replace("!", "");
            used_inputs.insert(type_name);
        }
        ArgumentValue::Array(values) => values
            .iter()
            .for_each(|value| track_argument_inputs(used_inputs, value)),
        ArgumentValue::MultiLineArray { values, .. } => values
            .iter()
            .for_each(|value| track_argument_inputs(used_inputs, value)),
        _ => todo!(),
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
