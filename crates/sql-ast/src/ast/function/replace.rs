use super::Function; // Assuming 'super::Function' is accessible
use crate::ast::{Expression, FunctionType}; // Assuming these paths are correct

/// Represents the nature of a string to be used in SQL,
/// particularly for functions like REPLACE where escaping might differ.
#[derive(Debug, Clone)]
pub enum SqlStringPattern<'a> {
    /// A literal string. During SQL rendering, this will typically be enclosed in single quotes,
    /// with internal single quotes escaped (e.g., 'a literal string', 'O''Malley').
    Literal(&'a str),
    /// A string whose content should be interpreted using PostgreSQL's E'' escape syntax.
    /// The provided string is the raw content (e.g., "\n", "foo\\bar").
    /// The SQL renderer will wrap this content appropriately (e.g., E'\n', E'foo\\\\bar').
    EscapedContent(&'a str),
}

#[derive(Debug, Clone)]
pub struct Replace<'a> {
    pub(crate) expression: Box<Expression<'a>>,
    pub(crate) old_value: SqlStringPattern<'a>,
    pub(crate) new_value: &'a str,
}

/// Constructs a `REPLACE` function AST node.
///
/// # Arguments
/// * `expression` - The base string expression.
/// * `old_value` - The pattern to search for, represented by `SqlStringPattern`.
///   This allows specifying either a literal string or content for E'' escaping.
/// * `new_value` - The literal string to replace with.
pub fn replace<'a, E>(expression: E, old_value: SqlStringPattern<'a>, new_value: &'a str) -> Function<'a>
where
    E: Into<Expression<'a>>,
{
    let fun = Replace {
        expression: Box::new(expression.into()),
        old_value,
        new_value,
    };

    fun.into()
}

impl<'a> From<Replace<'a>> for Function<'a> {
    fn from(value: Replace<'a>) -> Self {
        Self {
            r#type: FunctionType::Replace(value),
            alias: None,
        }
    }
}
