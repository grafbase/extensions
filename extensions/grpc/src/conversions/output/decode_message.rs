use super::*;
use crate::schema;
use bytes::Buf as _;
use prost::encoding;

pub(super) fn decode_message(
    proto: &mut bytes::Bytes,
    message_definition: &schema::Message,
    schema: &schema::Schema,
    ctx: encoding::DecodeContext,
) -> Result<Value, prost::DecodeError> {
    let mut map: Vec<(Value, Value)> = Vec::with_capacity(message_definition.fields.len());

    while proto.has_remaining() {
        let (tag, wire_type) = prost::encoding::decode_key(proto)?;

        let field = message_definition
            .fields
            .iter()
            .find(|(_, field)| field.number == tag)
            .map(|(_, f)| f);

        let Some(value) = decode_value(proto, wire_type, field.map(|f| &f.ty), schema, ctx.clone())? else {
            continue;
        };

        let Some(field) = field else {
            continue;
        };

        let previous_entry = map.iter_mut().find(|(key, _)| key.is_str(&field.name));

        if field.repeated {
            match previous_entry {
                Some((_, Value::List(existing))) => existing.push(value),
                Some(_) => unreachable!(),
                None => {
                    map.push((Value::String(field.name.clone().into()), Value::List(vec![value])));
                }
            }
        } else {
            match (previous_entry, value) {
                (Some((_, Value::Map(previous))), Value::Map(new)) => {
                    previous.extend(new);
                }
                // https://protobuf.dev/programming-guides/encoding/#last-one-wins
                (Some((_, previous)), value) => *previous = value,
                (None, value) => map.push((Value::String(field.name.clone().into()), value)),
            }
        }
    }

    Ok(Value::Map(map))
}
