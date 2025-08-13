## Unreleased

### Added

- **Composite schemas shortcuts** added. New protobuf options for simplified composite schemas directives:
  - `key` option generates `@key` directives on types
  - `lookup` option generates `@lookup` directive with optional `@is` mapping
  - `join_field` option generates fields with `@require` and `@grpcMethod` directives, with proper type resolution

- **Input argument directives** added. You can now add GraphQL directives to RPC method input argument, that corresponds to the input of the RPC method, using the `argument_directives` option on methods.

- **Proto3 optional field support** added. The generator now properly handles proto3 optional fields:
  - Non-optional scalar and enum fields in proto3 are rendered as non-nullable (`Type!`) in GraphQL output types
  - Optional scalar and enum fields are rendered as nullable (`Type`) in GraphQL output types
  - Message type fields are always nullable regardless of the optional flag
  - Input types remain nullable for all fields
  - The generator now declares support for `FEATURE_PROTO3_OPTIONAL` to work with proto3 files containing optional fields

- **Composite schema entity references with @derive** added. You can now create federation-style entity references using the `derive_field` option on messages:
  - Use `option (grafbase.graphql.derive_field) = {entity: "User", is: "{ id: user_id }"};` on fields
  - Automatically generates reference fields with `@derive` and `@is` directives
  - Creates stub entity types with `@key` directives if the type is not already defined
  - Supports custom relation field names with the `name` parameter
  - The `@is` directive uses the value from the `is` parameter directly
  - Enables cross-subgraph entity references in federated schemas

- **Input argument directives support** added. You can now add GraphQL directives to RPC method input arguments using the `argument_directives` option:
  - Use `option (grafbase.graphql.argument_directives) = "@constraint(minLength: 1)";` in method options
  - Supports multiple directives in a single string, separated by spaces
  - Works with both Query and Mutation fields
  - Can be combined with existing method directives

- **Multiple subgraphs support** added. Support for generating multiple GraphQL files based on service annotations:

  - Services can now have a `subgraph_name` option that maps them to different subgraph files
  - When any service has a `subgraph_name`, the tool automatically switches to multi-file mode
  - Generated files are named `<subgraph_name>.graphql` instead of the default `schema.graphql`
  - Each subgraph file only includes the services and types relevant to that specific subgraph
  - Multiple services can map to the same subgraph
  - Subgraph names must match the pattern `[a-zA-Z][a-zA-Z0-9-]*`
  - Services without `subgraph_name` in multi-file mode are ignored without warning

### Fixed

- In some scenarios, the plugin would panic because packages were not provided in alphabetical order. This is fixed. (https://github.com/grafbase/extensions/pull/144)

## 0.2.0 - 2025-07-24

### Added

- **GraphQL Directive Support**: Added support for all directive options defined in `options.proto`:
  - `object_directives` and `input_object_directives` for object-level directives
  - `input_field_directives` and `output_field_directives` for field-level directives
  - `enum_directives` for enum-level directives
  - `enum_value_directives` for enum value-level directives
- **Query Field Mapping**: Added support for mapping gRPC service methods to GraphQL Query fields instead of Mutations:
  - `is_graphql_query_field` and `is_graphql_mutation_field` options on individual methods
  - `graphql_default_to_query_fields` and `graphql_default_to_mutation_fields` options on services to make all methods default to Query (or Mutation) fields
- **Query Type Generation**: The generator now creates a `type Query` in addition to `type Mutation` and `type Subscription` based on method configurations

## 0.1.0 - 2025-04-15

- Initial release. The output matches the directives expected by version 0.1.0 of the grpc extension.
