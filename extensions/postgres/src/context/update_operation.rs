use serde::Deserialize;
use serde_json::Value;

/// Represents all possible update operations for a column in Postgres
/// 
/// The operation comes in the format: `column_name: { operation: value }`
/// Example: `{ "name": { "set": "John" } }`
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum UpdateOperation {
    /// Set the column to a specific value
    Set { set: Value },
    
    /// Increment the column by the specified value
    Increment { increment: Value },
    
    /// Decrement the column by the specified value
    Decrement { decrement: Value },
    
    /// Delete a key from a JSON object
    DeleteKey { deleteKey: Value },
    
    /// Multiply the column by the specified value
    Multiply { multiply: Value },
    
    /// Divide the column by the specified value
    Divide { divide: Value },
    
    /// Append a value to an array or concatenate with a string
    Append { append: Value },
    
    /// Prepend a value to an array or concatenate with a string
    Prepend { prepend: Value },
    
    /// Delete a key path from a JSON object
    DeleteAtPath { deleteAtPath: Value },
}

impl UpdateOperation {
    /// Returns the operation type as a string
    pub fn operation_type(&self) -> &'static str {
        match self {
            Self::Set { .. } => "set",
            Self::Increment { .. } => "increment",
            Self::Decrement { .. } => "decrement",
            Self::DeleteKey { .. } => "deleteKey",
            Self::Multiply { .. } => "multiply",
            Self::Divide { .. } => "divide",
            Self::Append { .. } => "append",
            Self::Prepend { .. } => "prepend",
            Self::DeleteAtPath { .. } => "deleteAtPath",
        }
    }

    /// Extract the value from any variant
    pub fn value(&self) -> &Value {
        match self {
            Self::Set { set } => set,
            Self::Increment { increment } => increment,
            Self::Decrement { decrement } => decrement,
            Self::DeleteKey { deleteKey } => deleteKey,
            Self::Multiply { multiply } => multiply,
            Self::Divide { divide } => divide,
            Self::Append { append } => append,
            Self::Prepend { prepend } => prepend,
            Self::DeleteAtPath { deleteAtPath } => deleteAtPath,
        }
    }

    /// Consumes the operation and returns the contained value
    pub fn into_value(self) -> Value {
        match self {
            Self::Set { set } => set,
            Self::Increment { increment } => increment,
            Self::Decrement { decrement } => decrement,
            Self::DeleteKey { deleteKey } => deleteKey,
            Self::Multiply { multiply } => multiply,
            Self::Divide { divide } => divide,
            Self::Append { append } => append,
            Self::Prepend { prepend } => prepend,
            Self::DeleteAtPath { deleteAtPath } => deleteAtPath,
        }
    }
}