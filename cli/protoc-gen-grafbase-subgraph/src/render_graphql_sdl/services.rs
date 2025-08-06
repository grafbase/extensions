use super::*;
use crate::schema::{GraphQLOperationType, GrpcSchema, ProtoMethod, ProtoMethodId, View};
use std::fmt;

pub(super) fn render_services_filtered(
    schema: &GrpcSchema,
    service_ids: Option<&[crate::schema::ProtoServiceId]>,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    if schema.services.is_empty() {
        return Ok(());
    }

    // Collect methods based on service filter
    let methods: Vec<_> = match service_ids {
        Some(ids) => schema
            .iter_methods()
            .filter(|method| ids.contains(&method.service_id))
            .collect(),
        None => schema.iter_methods().collect(),
    };

    let mut query_methods = methods
        .iter()
        .filter(|method| method.graphql_operation_type(schema) == GraphQLOperationType::Query)
        .peekable();

    let mut mutation_methods = methods
        .iter()
        .filter(|method| method.graphql_operation_type(schema) == GraphQLOperationType::Mutation)
        .peekable();

    let mut subscription_methods = methods
        .iter()
        .filter(|method| method.graphql_operation_type(schema) == GraphQLOperationType::Subscription)
        .peekable();

    if query_methods.peek().is_some() {
        f.write_str("type Query {\n")?;
        for method in query_methods {
            render_method_field(schema, View::new(method.id, method.record), f)?;
        }

        f.write_str("}\n\n")?;
    }

    if mutation_methods.peek().is_some() {
        f.write_str("type Mutation {\n")?;
        for method in mutation_methods {
            render_method_field(schema, View::new(method.id, method.record), f)?;
        }

        f.write_str("}\n\n")?;
    }

    if subscription_methods.peek().is_some() {
        f.write_str("type Subscription {\n")?;
        for method in subscription_methods {
            render_method_field(schema, View::new(method.id, method.record), f)?;
        }

        f.write_str("}\n\n")?;
    }

    Ok(())
}

pub(super) fn collect_types_to_render_filtered(
    schema: &GrpcSchema,
    service_ids: Option<&[crate::schema::ProtoServiceId]>,
) -> TypesToRender {
    let mut types_to_render = TypesToRender::default();

    if schema.services.is_empty() {
        return types_to_render;
    }

    let methods_iter: Box<dyn Iterator<Item = _>> = match service_ids {
        Some(ids) => Box::new(
            schema
                .iter_methods()
                .filter(move |method| ids.contains(&method.service_id)),
        ),
        None => Box::new(schema.iter_methods()),
    };

    for method in methods_iter {
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

    if let Some(directives) = method.argument_directives.as_deref() {
        write!(f, " {directives}")?;
    }

    f.write_str("): ")?;

    render_output_field_type(schema, &method.output_type, false, true, f)?;

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
