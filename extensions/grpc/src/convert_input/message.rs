use serde::de;

use crate::{Grpc, directives::ProtoMessage};

pub(crate) struct MessageDeserialize<'a> {
    pub(crate) schema: &'a Grpc,
    pub(crate) message: &'a ProtoMessage,
    pub(crate) out: &'a mut Vec<u8>,
}

impl<'de> de::DeserializeSeed<'de> for MessageDeserialize<'de> {
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_map(MessageVisitor { message: self.message })
    }
}

struct MessageVisitor<'a> {
    message: &'a ProtoMessage,
}

impl<'de> de::Visitor<'de> for MessageVisitor<'de> {
    type Value = ();

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str(&format!("a message of type {}", self.message.name))
    }
}
