# gRPC extension

This extension allows exposing gRPC services as part of your federated GraphQL graph. The extension assumes protocol buffer payloads for the communication between the Gateway and the gRPC services.

The core directive is `@grpcMethod`. It can be used on any output field, like so:

```graphql
type Query {
    getFeature(input: PointInput!): Feature @grpcMethod(service: "routeguide.RouteGuide", method: "GetFeature", input: "*")
}
```

It takes the following arguments:

- `service`: The name of the gRPC service.
- `method`: The name of the gRPC method.
- `input`: The input type of the gRPC method. It can be `"*"` to use the whole input, or a specific transformation. See the docs for [InputValueSet](https://grafbase.com/docs/reference/extensions/grafbase-spec/v1.0#inputvalueset). It defaults to `"*"`.

The service must also be defined on your GraphQL schema's schema definition, along with the required types:

```graphql
extend schema
    @link(url: "{path_str}", import: ["@grpcMethod", "@protoMessages", "@protoServices"])
    @protoMessages(definitions: [
      {
        name: "Point"
        fields: [
          { name: "latitude", type: "int32", number: 1 }
          { name: "longitude", type: "int32", number: 2 }
        ]
      },
      {
        name: "Feature"
        fields: [
          { name: "name", type: "string", number: 1 }
          { name: "location", type: "Point", number: 2 }
        ]
      }
    ])
    @protoServices(definitions: [
      {
        name: "routeguide.RouteGuide"
        methods: [
        { name: "GetFeature", inputType: "Point", outputType: "Feature" }
        ]
      }}
    ])
```

As you can imagine, these definitions can get verbose over time. See the following section for the recommended approach to generating them.

Once your services are defined in your GraphQL schema, you must define how to connect to them in your Gateway configuration file. For example:

```toml
[[extensions.grpc.config.services]]
name = "routeguide.RouteGuide"
address = "http://routeguide.mydomain.local"

[[extensions.grpc.config.services]]
name = "internal.Pricing"
address = "{{ env.PRICING_SERVICE_URL }}"
```

## Generation of virtual subgraph schemas from Protobuf definitions

Use the protobuf compiler plugin: `protoc-gen-grafbase-subgraph`. TODO.

## Conventions

The conversion between JSON and protobuf types follows the conventions defined in the [ProtoJSON format](https://protobuf.dev/programming-guides/json/) documentation.

## Features

- Server streaming for methods on subscription fields.
- Client streaming methods are supported, but only one message can be sent. Please reach out if you are interested in more extensive support.
- Well-known types support, with the ProtoJSON mapping:
  - [x] Value types (`BoolValue`, `BytesValue`, etc.)
  - [x] `Duration`
  - [x] `Empty`
  - [x] `FieldMask`
  - [ ] `Struct`, `ListValue`, `Value`...
  - [ ] `Any`
  - [ ] Reflection
