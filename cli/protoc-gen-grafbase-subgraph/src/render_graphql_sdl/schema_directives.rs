use super::*;

pub(super) fn render_schema_directives_filtered(
    schema: &GrpcSchema,
    service_ids: Option<&[crate::schema::ProtoServiceId]>,
    types_to_render: &services::TypesToRender,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    let mut all_schema_directives = Vec::new();

    let services = if let Some(ids) = service_ids {
        schema
            .iter_services()
            .filter(|service| ids.contains(&service.id))
            .collect::<Vec<_>>()
    } else {
        schema.iter_services().collect::<Vec<_>>()
    };

    for service in &services {
        if let Some(directives) = &service.schema_directives {
            all_schema_directives.push(directives.as_str());
        }
    }

    let mut seen = std::collections::HashSet::new();
    let unique_directives: Vec<_> = all_schema_directives.into_iter().filter(|d| seen.insert(*d)).collect();

    f.write_str("extend schema\n  @link(url: \"https://grafbase.com/extensions/grpc/0.2.0\", import: [\"@protoServices\", \"@protoEnums\", \"@protoMessages\", \"@grpcMethod\"])\n")?;

    for directive in unique_directives {
        f.write_str("  ")?;
        f.write_str(directive)?;
        f.write_str("\n")?;
    }

    render_proto_services_filtered(schema, service_ids, f)?;
    render_proto_messages(schema, types_to_render, f)?;
    render_proto_enums(schema, types_to_render, f)?;

    f.write_str("\n")
}

fn render_proto_services_filtered(
    schema: &GrpcSchema,
    service_ids: Option<&[crate::schema::ProtoServiceId]>,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    let services_to_render: Vec<_> = if let Some(ids) = service_ids {
        schema
            .iter_services()
            .filter(|service| ids.contains(&service.id))
            .collect()
    } else {
        schema.iter_services().collect()
    };

    if services_to_render.is_empty() {
        return Ok(());
    }

    f.write_str(INDENT)?;
    f.write_str("@protoServices(\n")?;
    f.write_str(INDENT)?;
    f.write_str(INDENT)?;
    f.write_str("definitions: [\n")?;

    for service in services_to_render {
        writeln!(f, "{INDENT}{INDENT}{INDENT}{{")?;
        writeln!(f, "{INDENT}{INDENT}{INDENT}{INDENT}name: \"{}\"", service.name)?;
        writeln!(f, "{INDENT}{INDENT}{INDENT}{INDENT}methods: [")?;

        for method in service.id.methods(schema) {
            writeln!(f, "{INDENT}{INDENT}{INDENT}{INDENT}{INDENT}{{")?;
            writeln!(
                f,
                "{INDENT}{INDENT}{INDENT}{INDENT}{INDENT}{INDENT}name: \"{}\"",
                method.name
            )?;
            writeln!(
                f,
                "{INDENT}{INDENT}{INDENT}{INDENT}{INDENT}{INDENT}inputType: \"{}\"",
                method.input_type.proto_name(schema)
            )?;
            writeln!(
                f,
                "{INDENT}{INDENT}{INDENT}{INDENT}{INDENT}{INDENT}outputType: \"{}\"",
                method.output_type.proto_name(schema)
            )?;

            if method.server_streaming {
                writeln!(
                    f,
                    "{INDENT}{INDENT}{INDENT}{INDENT}{INDENT}{INDENT}serverStreaming: true",
                )?;
            }
            writeln!(f, "{INDENT}{INDENT}{INDENT}{INDENT}{INDENT}}}")?;
        }

        writeln!(f, "{INDENT}{INDENT}{INDENT}{INDENT}]")?;
        writeln!(f, "{INDENT}{INDENT}{INDENT}}}")?;
    }

    f.write_str(INDENT)?;
    f.write_str(INDENT)?;
    f.write_str("]\n")?;

    f.write_str(INDENT)?;
    f.write_str(")\n")?;

    Ok(())
}

fn render_proto_messages(
    schema: &GrpcSchema,
    types_to_render: &services::TypesToRender,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    let mut messages_to_render = schema
        .iter_messages()
        .filter(|message| {
            types_to_render.messages_to_render_as_input.contains(&message.id)
                || types_to_render.messages_to_render_as_output.contains(&message.id)
        })
        .peekable();

    if messages_to_render.peek().is_none() {
        return Ok(());
    }

    f.write_str(INDENT)?;
    f.write_str("@protoMessages(\n")?;
    f.write_str(INDENT)?;
    f.write_str(INDENT)?;
    f.write_str("definitions: [\n")?;

    for message in messages_to_render {
        writeln!(f, "{INDENT}{INDENT}{INDENT}{{")?;
        writeln!(f, "{INDENT}{INDENT}{INDENT}{INDENT}name: \"{}\"", message.name)?;
        writeln!(f, "{INDENT}{INDENT}{INDENT}{INDENT}fields: [")?;

        for field in message.id.fields(schema) {
            writeln!(f, "{INDENT}{INDENT}{INDENT}{INDENT}{INDENT}{{")?;
            writeln!(
                f,
                "{INDENT}{INDENT}{INDENT}{INDENT}{INDENT}{INDENT}name: \"{}\"",
                field.name
            )?;
            writeln!(
                f,
                "{INDENT}{INDENT}{INDENT}{INDENT}{INDENT}{INDENT}number: {}",
                field.number,
            )?;
            writeln!(
                f,
                "{INDENT}{INDENT}{INDENT}{INDENT}{INDENT}{INDENT}repeated: {}",
                field.repeated,
            )?;
            writeln!(
                f,
                "{INDENT}{INDENT}{INDENT}{INDENT}{INDENT}{INDENT}type: \"{}\"",
                field.r#type.proto_name(schema)
            )?;

            writeln!(f, "{INDENT}{INDENT}{INDENT}{INDENT}{INDENT}}}")?;
        }

        writeln!(f, "{INDENT}{INDENT}{INDENT}{INDENT}]")?;
        writeln!(f, "{INDENT}{INDENT}{INDENT}}}")?;
    }

    f.write_str(INDENT)?;
    f.write_str(INDENT)?;
    f.write_str("]\n")?;

    f.write_str(INDENT)?;
    f.write_str(")\n")?;

    Ok(())
}

fn render_proto_enums(
    schema: &GrpcSchema,
    types_to_render: &services::TypesToRender,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    let mut enums_to_render = schema
        .iter_enums()
        .filter(|r#enum| types_to_render.enums_to_render.contains(&r#enum.id))
        .peekable();

    if enums_to_render.peek().is_none() {
        return Ok(());
    }

    f.write_str(INDENT)?;
    f.write_str("@protoEnums(\n")?;
    f.write_str(INDENT)?;
    f.write_str(INDENT)?;
    f.write_str("definitions: [\n")?;

    for r#enum in enums_to_render {
        writeln!(f, "{INDENT}{INDENT}{INDENT}{{")?;
        writeln!(f, "{INDENT}{INDENT}{INDENT}{INDENT}name: \"{}\"", r#enum.name)?;
        writeln!(f, "{INDENT}{INDENT}{INDENT}{INDENT}values: [")?;

        for value in &r#enum.values {
            writeln!(f, "{INDENT}{INDENT}{INDENT}{INDENT}{INDENT}{{")?;
            writeln!(
                f,
                "{INDENT}{INDENT}{INDENT}{INDENT}{INDENT}{INDENT}name: \"{}\"",
                value.name
            )?;
            writeln!(
                f,
                "{INDENT}{INDENT}{INDENT}{INDENT}{INDENT}{INDENT}number: {}",
                value.number,
            )?;

            writeln!(f, "{INDENT}{INDENT}{INDENT}{INDENT}{INDENT}}}")?;
        }

        writeln!(f, "{INDENT}{INDENT}{INDENT}{INDENT}]")?;
        writeln!(f, "{INDENT}{INDENT}{INDENT}}}")?;
    }

    f.write_str(INDENT)?;
    f.write_str(INDENT)?;
    f.write_str("]\n")?;

    f.write_str(INDENT)?;
    f.write_str(")\n")?;

    Ok(())
}
