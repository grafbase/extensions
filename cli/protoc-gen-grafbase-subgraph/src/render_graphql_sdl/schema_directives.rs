use super::*;

pub(super) fn render_schema_directives(
    schema: &GrpcSchema,
    types_to_render: &services::TypesToRender,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    f.write_str("extend schema\n  @link(url: \"https://grafbase.com/extensions/grpc/0.1.2\", import: [\"@protoServices\", \"@protoEnums\", \"@protoMessages\", \"@grpcMethod\"])\n")?;

    render_proto_services(schema, f)?;
    render_proto_messages(schema, types_to_render, f)?;
    render_proto_enums(schema, types_to_render, f)?;

    f.write_str("\n")
}

fn render_proto_services(schema: &GrpcSchema, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if schema.services.is_empty() {
        return Ok(());
    }

    f.write_str(INDENT)?;
    f.write_str("@protoServices(\n")?;
    f.write_str(INDENT)?;
    f.write_str(INDENT)?;
    f.write_str("definitions: [\n")?;

    for service in schema.iter_services() {
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
