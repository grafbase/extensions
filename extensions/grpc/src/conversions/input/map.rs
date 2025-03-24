use crate::schema;
use prost::encoding;
use serde::de;

pub(crate) struct MapDeserialize<'a> {
    pub(crate) schema: &'a schema::Schema,
    pub(crate) key_type: &'a schema::FieldType,
    pub(crate) value_type: &'a schema::FieldType,
    pub(crate) tag: u32,
    pub(crate) out: &'a mut Vec<u8>,
}

impl<'de> de::DeserializeSeed<'de> for MapDeserialize<'_> {
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_map(self)
    }
}

impl<'de> de::Visitor<'de> for MapDeserialize<'_> {
    type Value = ();

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut submessage = Vec::new();

        while map
            .next_key_seed(super::field::FieldVisitor {
                schema: self.schema,
                field_type: self.key_type,
                tag: 1,
                out: &mut submessage,
            })?
            .is_some()
        {
            map.next_value_seed(super::field::FieldVisitor {
                schema: self.schema,
                field_type: self.value_type,
                tag: 2,
                out: &mut submessage,
            })?;

            encoding::encode_key(self.tag, encoding::WireType::LengthDelimited, self.out);
            encoding::encode_length_delimiter(submessage.len(), self.out)
                .map_err(|e| de::Error::custom(format!("Failed to encode length delimiter: {}", e)))?;

            self.out.extend_from_slice(submessage.as_slice());

            submessage.clear();
        }

        Ok(())
    }

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "a map<{}, {}>",
            self.key_type.proto_name(self.schema),
            self.value_type.proto_name(self.schema)
        )
    }
}
