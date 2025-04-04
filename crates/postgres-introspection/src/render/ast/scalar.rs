use std::fmt;

pub struct Scalar<'a> {
    name: &'a str,
}

impl<'a> Scalar<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { name }
    }
}

impl fmt::Display for Scalar<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "scalar {}", self.name)
    }
}
