use super::*;
use crate::schema;
use prost::encoding;
use serde::ser::{self};

pub(crate) struct MessageSerialize<'a> {
    proto: &'a bytes::Bytes,
    message_definition: &'a schema::Message,
    schema: &'a schema::Schema,
}

impl<'a> MessageSerialize<'a> {
    pub(crate) fn new(
        proto: &'a bytes::Bytes,
        message_definition: &'a schema::Message,
        schema: &'a schema::Schema,
    ) -> Self {
        Self {
            proto,
            message_definition,
            schema,
        }
    }
}

impl ser::Serialize for MessageSerialize<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut proto = self.proto.clone();
        let root_value = decode_message(
            &mut proto,
            self.message_definition,
            self.schema,
            encoding::DecodeContext::default(),
        )
        .map_err(|e| serde::ser::Error::custom(format!("Failed to decode message: {}", e)))?;

        ProtoSerialize {
            value: &root_value,
            schema: self.schema,
        }
        .serialize(serializer)
    }
}
