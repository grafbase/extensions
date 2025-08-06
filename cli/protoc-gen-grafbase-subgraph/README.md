# protoc-gen-grafbase-subgraph

This binary crate is a protoc plugin that generates a GraphQL subgraph to be used in concert with the Grafbase gRPC extension.

## Installation

Download the relevant binary from your platform from the [GitHub releases](https://github.com/grafbase/extensions/releases?q=protoc-gen-grafbase-subgraph&expanded=true).

## Usage with buf

Make sure the binary is in your PATH, then configure it in your `buf.gen.yaml` file:

```yaml
version: v2
managed:
  enabled: true
plugins:
  - local: protoc-gen-grafbase-subgraph
    out: .
inputs:
  - directory: proto
```

## Usage with protoc

Make sure the binary is in your PATH, then run protoc with the `--grafbase-subgraph_out` flag. For example:

```
protoc --grafbase-subgraph_out=. proto/*.proto
```

## Custom options

To make use of custom options, you will need to import the option definitions. They are defined in the `grafbase.options` package, available in the "options.proto" file in this project. Also note that since this module imports ["google/protobuf/descriptor.proto"](https://github.com/protocolbuffers/protobuf/blob/8228ee42b512cc330971e61bc9b86935a59f3477/src/google/protobuf/descriptor.proto), that one has to be present in your project as well.

With `buf`, you can make use of the [inputs.git_repo](https://buf.build/docs/configuration/v2/buf-gen-yaml/#git_repo) option.

### Mapping RPC methods to Query fields

By default, RPC methods are mapped to fields on Mutation. But you can also map them to fields on Query:

```protobuf
import "grafbase/options.proto";

service SearchService {
  rpc Search(SearchRequest) returns (SearchResponse) {
    option (grafbase.graphql.is_query_field) = true;
  }
}
```

### Default all service methods to Query fields

```protobuf
import "grafbase/options.proto";

service SearchService {
  option (grafbase.graphql.default_to_query_fields) = true;

  rpc Search(SearchRequest) returns (SearchResponse) {
    option (grafbase.graphql.directives) = "@lookup";
  }
}
```

Analogous to the "is_query_field" option above, there is also a "is_mutation_field" option to map RPC methods to fields on Mutation when your service defaults to Query:

```protobuf
import "grafbase/options.proto";

service SearchService {
  option (grafbase.graphql.default_to_query_fields) = true;

  rpc Search(SearchRequest) returns (SearchResponse) {
    option (grafbase.graphql.is_mutation_field) = true;
  }
}
```

### Adding GraphQL directives on types, fields and enum values

```protobuf
import "grafbase/options.proto";

message MyMessage {
  option (grafbase.graphql.object_directives) = "@key(fields: \"id\")";

  string id = 1 [(grafbase.graphql.output_field_directives) = "@deprecated"];
}

enum Color {
  option (grafbase.graphql.enum_directives) = "@deprecated";

  RED = 0 [(grafbase.graphql.enum_value_directives) = "@deprecated @tag(name: \"private\")"];
  GREEN = 1 [(grafbase.graphql.enum_value_directives) = "@deprecated"];
  BLUE = 2 [(grafbase.graphql.enum_value_directives) = "@deprecated"];
}
```

### Adding schema directives

You can add directives to the GraphQL schema extension using the `schema_directives` option at the service level:

```protobuf
import "grafbase/options.proto";

service MyService {
  option (grafbase.graphql.schema_directives) = "@contact(name: \"API Support\", url: \"https://api.example.com/support\")";
  
  rpc GetItem(GetItemRequest) returns (Item);
}

service AnotherService {
  option (grafbase.graphql.schema_directives) = "@tag(name: \"backend\") @auth(rules: [{allow: \"admin\"}])";
  
  rpc UpdateItem(UpdateItemRequest) returns (Item);
}
```

This will generate:

```graphql
extend schema
  @link(url: "https://grafbase.com/extensions/grpc/0.2.0", import: ["@protoServices", "@protoEnums", "@protoMessages", "@grpcMethod"])
  @contact(name: "API Support", url: "https://api.example.com/support")
  @tag(name: "backend") @auth(rules: [{allow: "admin"}])
  ...
```

#### Schema directive behavior

- **Deduplication**: If multiple services specify the same directive, it will only appear once in the schema extension
- **Multi-file mode**: When using `subgraph_name`, only directives from services in that subgraph are included
- **Multiple directives**: You can include multiple directives in a single string, separated by spaces

### Mapping specific services to different subgraphs

By default, the plugin generates a single `schema.graphql` file containing all services. However, you can map different services to different subgraph files using the `subgraph_name` option:

```protobuf
import "grafbase/options.proto";

service UserService {
  option (grafbase.graphql.subgraph_name) = "users";

  rpc GetUser(GetUserRequest) returns (User);
  rpc CreateUser(CreateUserRequest) returns (User);
}

service ProductService {
  option (grafbase.graphql.subgraph_name) = "products";

  rpc GetProduct(GetProductRequest) returns (Product);
  rpc ListProducts(ListProductsRequest) returns (ListProductsResponse);
}

service OrderService {
  option (grafbase.graphql.subgraph_name) = "orders";

  rpc CreateOrder(CreateOrderRequest) returns (Order);
}
```

This will generate three files:
- `users.graphql` - Contains only the UserService and its related types
- `products.graphql` - Contains only the ProductService and its related types
- `orders.graphql` - Contains only the OrderService and its related types

#### Multi-file mode behavior

- **Automatic mode detection**: As soon as any service has a `subgraph_name` option, the plugin switches to multi-file mode
- **Service filtering**: Each generated file only includes the services mapped to that subgraph
- **Type filtering**: Only types actually used by the services in a subgraph are included in that file
- **Shared subgraphs**: Multiple services can map to the same subgraph by using the same `subgraph_name`
- **Validation**: Subgraph names must match the pattern `[a-zA-Z][a-zA-Z0-9-]*`
- **Backward compatibility**: If no services have `subgraph_name`, the default single `schema.graphql` is generated
- **Services without subgraph_name**: In multi-file mode, services without a `subgraph_name` are ignored

## Limitations

- Methods with client streaming are supported, but only one message can be sent from the client side.

## Contributing

### Releasing

To release a new version of the binary:

1. Update the version number in `Cargo.toml`
2. Create a tag with the format `protoc-gen-grafbase-subgraph-X.Y.Z` (e.g., `protoc-gen-grafbase-subgraph-0.2.0`)
3. Push the tag to GitHub:
   ```
   git tag protoc-gen-grafbase-subgraph-X.Y.Z
   git push origin protoc-gen-grafbase-subgraph-X.Y.Z
   ```
4. The GitHub Actions workflow will automatically build the binary for multiple platforms and create a release with the artifacts

## Prior art

- https://github.com/ysugimoto/grpc-graphql-gateway generates a graphql-go based GraphQL server that proxies to a gRPC server.
- https://github.com/danielvladco/go-proto-gql another project that does the same. Unmaintained.
