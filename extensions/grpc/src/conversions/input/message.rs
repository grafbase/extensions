use crate::schema;
use serde::de;

pub(crate) struct MessageDeserialize<'a> {
    pub(crate) message_definition: &'a schema::Message,
    pub(crate) schema: &'a schema::Schema,
    pub(crate) out: &'a mut Vec<u8>,
}

impl<'de> de::DeserializeSeed<'de> for MessageDeserialize<'_> {
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_map(self)?;

        Ok(())
    }
}

impl<'de> de::Visitor<'de> for MessageDeserialize<'_> {
    type Value = ();

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str(&format!("a message of type {}", self.message_definition.name))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        while let Some(key) = map.next_key::<&str>()? {
            let field = self.message_definition.fields.get(key).ok_or_else(|| {
                de::Error::custom(format!(
                    "unknown field {} on message {}",
                    key, self.message_definition.name
                ))
            })?;

            let tag = field.number;

            if field.repeated {
                map.next_value_seed(super::repeated::RepeatedFieldVisitor {
                    tag,
                    schema: self.schema,
                    field,
                    out: self.out,
                })?;
            } else {
                map.next_value_seed(super::field::FieldVisitor {
                    schema: self.schema,
                    field_type: &field.ty,
                    tag,
                    out: self.out,
                })?;
            }
        }

        Ok(())
    }
}
