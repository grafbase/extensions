use super::*;
use crate::schema::{ProtoMessage, ScalarType, View};

pub(super) fn render_graphql_types(
    schema: &GrpcSchema,
    types_to_render: &services::TypesToRender,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    let services::TypesToRender {
        messages_to_render_as_input,
        messages_to_render_as_output,
        enums_to_render,
    } = types_to_render;

    f.write_str("\"64 bit signed integer\" scalar I64\n")?;
    f.write_str("\"64 bit unsigned integer\" scalar U64\n")?;

    for message_id in messages_to_render_as_input {
        render_message(schema, *message_id, true, f)?;
    }

    for message_id in messages_to_render_as_output {
        render_message(schema, *message_id, false, f)?;
    }

    for enum_id in enums_to_render {
        render_enum_definition(schema, *enum_id, f)?;
    }

    render_entity_types(schema, messages_to_render_as_output, f)?;

    Ok(())
}

fn render_message(
    schema: &GrpcSchema,
    message_id: ProtoMessageId,
    input: bool,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    let message = schema.view(message_id);

    match message.name.as_str() {
        ".google.protobuf.Duration"
        | ".google.protobuf.Timestamp"
        | ".google.protobuf.FieldMask"
        | ".google.protobuf.BoolValue"
        | ".google.protobuf.BytesValue"
        | ".google.protobuf.DoubleValue"
        | ".google.protobuf.FloatValue"
        | ".google.protobuf.Int32Value"
        | ".google.protobuf.Int64Value"
        | ".google.protobuf.StringValue"
        | ".google.protobuf.UInt32Value"
        | ".google.protobuf.UInt64Value" => return Ok(()),

        ".google.protobuf.Empty" => f.write_str("\n\"An empty object \" scalar EmptyObject\n")?,

        _ => (),
    }

    f.write_str("\n")?;

    if let Some(description) = message.description.as_deref() {
        render_description(f, description)?;
    }

    if input {
        write!(f, "input {} ", message.graphql_input_name(),)?;
    } else {
        write!(f, "type {} ", message.graphql_output_name(),)?;
    }

    let directives = if input {
        message.input_object_directives.as_deref()
    } else {
        message.object_directives.as_deref()
    };

    if let Some(directives) = directives {
        f.write_str(directives)?;
        f.write_str(" ")?;
    }

    f.write_str("{\n")?;

    for field in message_id.fields(schema) {
        if let Some(description) = field.description.as_deref() {
            render_description(f, description)?;
        }

        f.write_str(INDENT)?;
        f.write_str(&field.name)?;
        f.write_str(": ")?;

        if input {
            render_input_field_type(schema, &field.r#type, field.repeated, f)?;
        } else {
            render_output_field_type(schema, &field.r#type, field.repeated, field.optional, f)?;
        }

        let field_directives = if input {
            field.input_field_directives.as_deref()
        } else {
            field.output_field_directives.as_deref()
        };

        if let Some(directives) = field_directives {
            f.write_str(" ")?;
            f.write_str(directives)?;
        }

        f.write_str("\n")?;
    }

    if !input {
        for extra_field in &message.object_extra_fields {
            f.write_str(INDENT)?;
            f.write_str(extra_field)?;
            f.write_str("\n")?;
        }

        render_derives(message, f)?;
    }

    f.write_str("}\n")
}

fn render_derives(message: View<'_, ProtoMessageId, ProtoMessage>, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for entity_info in &message.derives {
        f.write_str(INDENT)?;

        let derived_field_name = if let Some(name) = &entity_info.field {
            name.clone()
        } else {
            entity_info
                .is
                .as_ref()
                .and_then(|is| is.fields.first())
                .map(|first_field| {
                    first_field
                        .input_field_name
                        .trim_end_matches("_id")
                        .trim_end_matches("Id")
                        .to_owned()
                })
                .unwrap_or_else(|| entity_info.entity.to_lowercase())
        };

        f.write_str(&derived_field_name)?;
        f.write_str(": ")?;
        f.write_str(&entity_info.entity)?;
        f.write_str(" @derive")?;

        if let Some(is) = &entity_info.is {
            write!(f, " @is(field: \"{is}\")")?;
        }

        f.write_str("\n")?;
    }

    Ok(())
}

pub(super) fn render_output_field_type(
    schema: &GrpcSchema,
    ty: &FieldType,
    repeated: bool,
    optional: bool,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    if repeated {
        f.write_str("[")?;
    }

    match ty {
        FieldType::Scalar(scalar_type) => scalar_type.render_graphql_type(f)?,
        FieldType::Message(proto_message_id) => render_message_type_name(schema, *proto_message_id, false, f)?,
        FieldType::Enum(proto_enum_id) => schema.view(*proto_enum_id).graphql_name().fmt(f)?,
    }

    if repeated {
        f.write_str("!]")?;
    } else if !optional && matches!(ty, FieldType::Scalar(_) | FieldType::Enum(_)) {
        // Only scalar and enum types can be non-null based on optional flag
        // Message types are always nullable
        f.write_str("!")?;
    }

    Ok(())
}

pub(super) fn render_input_field_type(
    schema: &GrpcSchema,
    ty: &FieldType,
    repeated: bool,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    if repeated {
        f.write_str("[")?;
    }

    match ty {
        FieldType::Scalar(scalar_type) => scalar_type.render_graphql_type(f)?,
        FieldType::Message(proto_message_id) => render_message_type_name(schema, *proto_message_id, true, f)?,
        FieldType::Enum(proto_enum_id) => schema.view(*proto_enum_id).graphql_name().fmt(f)?,
    }

    if repeated {
        f.write_str("!]")?;
    }

    Ok(())
}

fn render_enum_definition(schema: &GrpcSchema, enum_id: ProtoEnumId, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let enum_type = schema.view(enum_id);

    f.write_str("\n")?;

    if let Some(description) = &enum_type.description {
        render_description(f, description)?;
    }

    write!(f, "enum {} ", enum_type.graphql_name())?;

    if let Some(directives) = &enum_type.enum_directives {
        f.write_str(directives)?;
        f.write_str(" ")?;
    }

    f.write_str("{\n")?;

    for value in &enum_type.values {
        if let Some(description) = value.description.as_deref() {
            render_description(f, description)?;
        }

        f.write_str(INDENT)?;
        f.write_str(value.name.as_str())?;

        if let Some(directives) = &value.enum_value_directives {
            f.write_str(" ")?;
            f.write_str(directives)?;
        }

        f.write_str(",\n")?;
    }

    f.write_str("}\n")
}

pub(super) fn render_description(f: &mut fmt::Formatter<'_>, description: &str) -> fmt::Result {
    writeln!(f, "\"\"\"\n{description}\n\"\"\"", description = description.trim())
}

fn render_message_type_name(
    schema: &GrpcSchema,
    proto_message_id: ProtoMessageId,
    is_input: bool,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    let message = schema.view(proto_message_id);

    // See the docs for the mapping of well-known types: https://protobuf.dev/programming-guides/json/
    match message.name.as_str() {
        ".google.protobuf.Duration" => f.write_str("String"),
        ".google.protobuf.Timestamp" => f.write_str("String"),
        ".google.protobuf.FieldMask" => f.write_str("String"),
        ".google.protobuf.Empty" => f.write_str("EmptyObject"),
        ".google.protobuf.BoolValue" => ScalarType::Bool.render_graphql_type(f),
        ".google.protobuf.BytesValue" => ScalarType::Bytes.render_graphql_type(f),
        ".google.protobuf.DoubleValue" => ScalarType::Double.render_graphql_type(f),
        ".google.protobuf.FloatValue" => ScalarType::Float.render_graphql_type(f),
        ".google.protobuf.Int32Value" => ScalarType::Int32.render_graphql_type(f),
        ".google.protobuf.Int64Value" => ScalarType::Int64.render_graphql_type(f),
        ".google.protobuf.StringValue" => ScalarType::String.render_graphql_type(f),
        ".google.protobuf.UInt32Value" => ScalarType::UInt32.render_graphql_type(f),
        ".google.protobuf.UInt64Value" => ScalarType::UInt64.render_graphql_type(f),

        _ => {
            if is_input {
                message.graphql_input_name().fmt(f)
            } else {
                message.graphql_output_name().fmt(f)
            }
        }
    }
}

fn render_entity_types(
    schema: &GrpcSchema,
    messages_to_render_as_output: &BTreeSet<ProtoMessageId>,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    use std::collections::{HashMap, HashSet};

    // entity name -> (list of key fields, message ID for field type lookup)
    let mut entities: HashMap<String, (Vec<&str>, ProtoMessageId)> = HashMap::new();

    let mut rendered_output_types: HashSet<String> = HashSet::new();
    for message_id in messages_to_render_as_output {
        let message = &schema[*message_id];
        rendered_output_types.insert(message.graphql_output_name().to_string());
    }

    for message_id in messages_to_render_as_output {
        let message = &schema[*message_id];
        for entity_info in &message.derives {
            if rendered_output_types.contains(&entity_info.entity) {
                continue;
            }

            match &entity_info.is {
                Some(simple_is) => {
                    let key_fields: Vec<&str> = simple_is.fields.iter().map(|f| f.output_field_name.as_str()).collect();

                    entities.insert(entity_info.entity.clone(), (key_fields, *message_id));
                }
                None => {
                    entities.insert(entity_info.entity.clone(), (vec!["id"], *message_id));
                }
            }
        }
    }

    let mut sorted_entities: Vec<_> = entities.into_iter().collect();
    sorted_entities.sort_by_key(|(entity_name, _)| entity_name.clone());

    for (entity_name, (key_fields, message_id)) in sorted_entities {
        f.write_str("\n")?;

        // Format the @key directive
        if key_fields.len() == 1 {
            write!(f, "type {} @key(fields: \"{}\")", entity_name, key_fields[0])?;
        } else {
            // For composite keys, use space-separated format
            write!(f, "type {} @key(fields: \"", entity_name)?;
            for (i, field) in key_fields.iter().enumerate() {
                if i > 0 {
                    f.write_str(" ")?;
                }
                f.write_str(field)?;
            }
            f.write_str("\")")?;
        }

        f.write_str(" {\n")?;

        let message = &schema[message_id];
        let matching_entity_info = message.derives.iter().find(|info| info.entity == entity_name);

        if let Some(entity_info) = matching_entity_info {
            if let Some(simple_is) = &entity_info.is {
                // Render each key field with its corresponding type from the message
                for is_field in &simple_is.fields {
                    f.write_str(INDENT)?;
                    f.write_str(&is_field.output_field_name)?;
                    f.write_str(": ")?;

                    // Find the corresponding field in the message
                    let field_found = message_id.fields(schema).find(|f| f.name == is_field.input_field_name);

                    if let Some(field) = field_found {
                        render_output_field_type(schema, &field.r#type, false, field.optional, f)?;
                    } else {
                        // Default to String if field not found
                        f.write_str("String")?;
                    }
                    f.write_str("\n")?;
                }
            } else {
                // No is mapping, default to id: String
                f.write_str(INDENT)?;
                f.write_str("id: String\n")?;
            }
        } else {
            // No entity info, render all key fields as String
            for field in &key_fields {
                f.write_str(INDENT)?;
                f.write_str(field)?;
                f.write_str(": String\n")?;
            }
        }

        f.write_str("}\n")?;
    }

    Ok(())
}
