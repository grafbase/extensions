use std::{
    borrow::Cow,
    fmt::{self, Write},
};

pub enum ArgumentValue<'a> {
    String(Cow<'a, str>),
    Constant(Cow<'a, str>),
    Array(Vec<ArgumentValue<'a>>),
    MultiLineArray {
        indent: &'static str,
        values: Vec<ArgumentValue<'a>>,
    },
}

impl fmt::Display for ArgumentValue<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgumentValue::String(s) => write!(f, "\"{}\"", s),
            ArgumentValue::Constant(c) => c.fmt(f),
            ArgumentValue::Array(arr) => {
                f.write_char('[')?;

                let items = arr.len();

                for (i, argument) in arr.iter().enumerate() {
                    argument.fmt(f)?;

                    if i < items - 1 {
                        f.write_str(", ")?;
                    }
                }

                f.write_char(']')
            }
            ArgumentValue::MultiLineArray { indent, values } => {
                f.write_str("[\n")?;

                let items = values.len();

                for (i, argument) in values.iter().enumerate() {
                    write!(f, "{indent}  {argument}")?;

                    if i < items - 1 {
                        f.write_str(",\n")?;
                    } else {
                        f.write_char('\n')?;
                    }
                }

                write!(f, "{indent}]")
            }
        }
    }
}

pub struct Argument<'a> {
    pub(super) name: Cow<'a, str>,
    pub(super) value: ArgumentValue<'a>,
    pub(super) description: Option<Cow<'a, str>>,
    pub(super) directives: Vec<Directive<'a>>,
}

impl<'a> Argument<'a> {
    pub fn new(name: impl Into<Cow<'a, str>>, value: ArgumentValue<'a>) -> Self {
        Argument {
            name: name.into(),
            value,
            description: None,
            directives: Vec::new(),
        }
    }

    pub fn string(name: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) -> Self {
        Argument::new(name.into(), ArgumentValue::String(value.into()))
    }

    pub fn constant(name: &'a str, value: impl Into<Cow<'a, str>>) -> Self {
        Argument::new(name, ArgumentValue::Constant(value.into()))
    }

    pub fn set_description(&mut self, description: impl Into<Cow<'a, str>>) {
        self.description = Some(description.into());
    }

    pub fn push_directive(&mut self, directive: Directive<'a>) {
        self.directives.push(directive);
    }

    pub fn has_description(&self) -> bool {
        self.description.is_some()
    }
}

impl fmt::Display for Argument<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(description) = &self.description {
            writeln!(f, r#"""""#)?;
            writeln!(f, "{description}")?;
            writeln!(f, r#"""""#)?;
        }

        write!(f, "{}: {}", self.name, self.value)?;

        for directive in &self.directives {
            write!(f, " {}", directive)?;
        }

        Ok(())
    }
}

pub struct Directive<'a> {
    name: &'a str,
    arguments: Vec<Argument<'a>>,
    render_multiline: bool,
}

impl<'a> Directive<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            arguments: Vec::new(),
            render_multiline: false,
        }
    }

    pub fn push_argument(&mut self, argument: Argument<'a>) {
        if argument.description.is_some() {
            self.render_multiline = true;
        }

        self.arguments.push(argument);
    }

    pub fn render_multiline(&mut self) {
        self.render_multiline = true;
    }
}

impl fmt::Display for Directive<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char('@')?;
        f.write_str(self.name)?;

        if !self.arguments.is_empty() {
            f.write_char('(')?;

            if self.render_multiline {
                writeln!(f)?;
            }

            for (i, arg) in self.arguments.iter().enumerate() {
                if self.render_multiline {
                    write!(f, "    {}", arg)?;

                    if i < self.arguments.len() - 1 {
                        writeln!(f, ",")?;
                    }
                } else {
                    arg.fmt(f)?;

                    if i < self.arguments.len() - 1 {
                        f.write_str(", ")?;
                    }
                }
            }

            if self.render_multiline {
                write!(f, "\n  )")?;
            } else {
                f.write_char(')')?;
            }
        }

        Ok(())
    }
}
