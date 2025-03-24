use crate::schema;
use serde::de;

pub(crate) struct ArgumentsDeserialize<'a> {
    pub(crate) schema: &'a schema::Schema,
    pub(crate) message: &'a schema::Message,
    pub(crate) out: &'a mut Vec<u8>,
}

impl<'de> de::DeserializeSeed<'de> for ArgumentsDeserialize<'_> {
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_map(self)?;

        Ok(())
    }
}

impl<'de> de::Visitor<'de> for ArgumentsDeserialize<'_> {
    type Value = ();

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let key: Option<&str> = map.next_key()?;

        match key {
            Some("input") => {
                map.next_value_seed(super::message::MessageDeserialize {
                    schema: self.schema,
                    message_definition: self.message,
                    out: self.out,
                })?;
                Ok(())
            }
            _ => Err(de::Error::custom("expected a single argument called `input`")),
        }
    }

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an `input` field")
    }
}
