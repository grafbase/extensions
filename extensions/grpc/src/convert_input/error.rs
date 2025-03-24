use std::fmt;

#[derive(Debug)]
pub(crate) struct MessageSerializerError(String);

impl fmt::Display for MessageSerializerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl std::error::Error for MessageSerializerError {}

impl serde::ser::Error for MessageSerializerError {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        MessageSerializerError(msg.to_string())
    }
}

impl From<MessageSerializerError> for grafbase_sdk::types::Error {
    fn from(value: MessageSerializerError) -> Self {
        grafbase_sdk::types::Error::new(value.0)
    }
}
