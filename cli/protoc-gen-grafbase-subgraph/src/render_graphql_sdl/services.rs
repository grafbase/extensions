use super::*;
use crate::schema::{GraphQLOperationType, GrpcSchema, ProtoMethod, ProtoMethodId, View};
use std::fmt;

pub(super) fn render_services(schema: &GrpcSchema, f: &mut fmt::Formatter<'_>) -> Result<TypesToRender, fmt::Error> {
    if schema.services.is_empty() {
        return Ok(TypesToRender::default());
    }

    let mut query_methods = schema
        .iter_methods()
        .filter(|method| method.graphql_operation_type(schema) == GraphQLOperationType::Query)
        .peekable();

    let mut mutation_methods = schema
        .iter_methods()
        .filter(|method| method.graphql_operation_type(schema) == GraphQLOperationType::Mutation)
        .peekable();

    let mut subscription_methods = schema
        .iter_methods()
        .filter(|method| method.graphql_operation_type(schema) == GraphQLOperationType::Subscription)
        .peekable();

    if query_methods.peek().is_some() {
        f.write_str("type Query {\n")?;
        for method in query_methods {
            render_method_field(schema, method, f)?;
        }

        f.write_str("}\n\n")?;
    }

    if mutation_methods.peek().is_some() {
        f.write_str("type Mutation {\n")?;
        for method in mutation_methods {
            render_method_field(schema, method, f)?;
        }

        f.write_str("}\n\n")?;
    }

    if subscription_methods.peek().is_some() {
        f.write_str("type Subscription {\n")?;
        for method in subscription_methods {
            render_method_field(schema, method, f)?;
        }

        f.write_str("}\n\n")?;
    }

    Ok(collect_types_to_render(schema))
}

pub(super) fn collect_types_to_render(schema: &GrpcSchema) -> TypesToRender {
    let mut types_to_render = TypesToRender::default();

    if schema.services.is_empty() {
        return types_to_render;
    }

    for method in schema.iter_methods() {
        collect_message_id_and_enum_ids_recursively(
            schema,
            &method.input_type,
            &mut types_to_render.messages_to_render_as_input,
            &mut types_to_render.enums_to_render,
        );

        collect_message_id_and_enum_ids_recursively(
            schema,
            &method.output_type,
            &mut types_to_render.messages_to_render_as_output,
            &mut types_to_render.enums_to_render,
        );
    }

    types_to_render
}

fn render_method_field(
    schema: &GrpcSchema,
    method: View<'_, ProtoMethodId, ProtoMethod>,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    let service = schema.view(method.service_id);

    if let Some(description) = method.description.as_ref() {
        super::graphql_types::render_description(f, description)?;
    }

    f.write_str(INDENT)?;
    write!(f, "{}_{}(input: ", service.graphql_name(), method.name)?;

    render_input_field_type(schema, &method.input_type, false, f)?;

    f.write_str("): ")?;

    render_output_field_type(schema, &method.output_type, false, f)?;

    write!(
        f,
        " @grpcMethod(service: \"{}\", method: \"{}\")",
        service.name, method.name
    )?;

    if let Some(directives) = method.directives.as_deref() {
        write!(f, " {directives}")?;
    }

    f.write_str("\n")?;

    Ok(())
}

#[derive(Debug, Default)]
pub(crate) struct TypesToRender {
    pub(super) messages_to_render_as_input: BTreeSet<ProtoMessageId>,
    pub(super) messages_to_render_as_output: BTreeSet<ProtoMessageId>,
    pub(super) enums_to_render: BTreeSet<ProtoEnumId>,
}

fn collect_message_id_and_enum_ids_recursively(
    schema: &GrpcSchema,
    field_type: &FieldType,
    message_ids: &mut BTreeSet<ProtoMessageId>,
    enum_ids: &mut BTreeSet<ProtoEnumId>,
) {
    match field_type {
        FieldType::Scalar(_scalar_type) => (),
        FieldType::Enum(proto_enum_id) => {
            enum_ids.insert(*proto_enum_id);
        }
        FieldType::Message(proto_message_id) => {
            if message_ids.insert(*proto_message_id) {
                for field in proto_message_id.fields(schema) {
                    collect_message_id_and_enum_ids_recursively(schema, &field.r#type, message_ids, enum_ids);
                }
            }
        }
    }
}
