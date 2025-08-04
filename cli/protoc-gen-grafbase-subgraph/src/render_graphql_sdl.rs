mod graphql_types;
mod schema_directives;
mod services;

use self::graphql_types::{render_input_field_type, render_output_field_type};
use crate::schema::{FieldType, GrpcSchema, ProtoEnumId, ProtoMessageId};
use std::{
    collections::BTreeSet,
    fmt::{self, Display as _},
};

const INDENT: &str = "  ";

pub(crate) fn render_graphql_sdl(schema: &GrpcSchema, out: impl fmt::Write) -> fmt::Result {
    render_graphql_sdl_filtered(schema, None, out)
}

pub(crate) fn render_graphql_sdl_filtered(
    schema: &GrpcSchema,
    service_ids: Option<&[crate::schema::ProtoServiceId]>,
    mut out: impl fmt::Write,
) -> fmt::Result {
    out.write_fmt(format_args!(
        "{}",
        crate::display_utils::display_fn(|f| {
            let types_to_render = services::collect_types_to_render_filtered(schema, service_ids);

            schema_directives::render_schema_directives_filtered(schema, service_ids, &types_to_render, f)?;

            services::render_services_filtered(schema, service_ids, f)?;

            graphql_types::render_graphql_types(schema, &types_to_render, f)
        })
    ))
}
