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

### Adding directives on RPC method input arguments

You can add GraphQL directives to the input argument of RPC methods using the `argument_directives` option:

```protobuf
import "grafbase/options.proto";

service UserService {
  rpc CreateUser(CreateUserRequest) returns (User) {
    option (grafbase.graphql.argument_directives) = "@constraint(minLength: 1)";
  }

  rpc UpdateUser(UpdateUserRequest) returns (User) {
    option (grafbase.graphql.argument_directives) = "@constraint(minLength: 1) @auth(rules: [{allow: owner}])";
  }

  rpc SearchUsers(SearchUsersRequest) returns (SearchUsersResponse) {
    option (grafbase.graphql.is_query) = true;
    option (grafbase.graphql.argument_directives) = "@constraint(maxLength: 100)";
  }
}
```

This will generate GraphQL with directives on the input arguments:

```graphql
type Query {
  UserService_SearchUsers(input: SearchUsersRequestInput @constraint(maxLength: 100)): SearchUsersResponse @grpcMethod(...)
}

type Mutation {
  UserService_CreateUser(input: CreateUserRequestInput @constraint(minLength: 1)): User @grpcMethod(...)
  UserService_UpdateUser(input: UpdateUserRequestInput @constraint(minLength: 1) @auth(rules: [{allow: owner}])): User @grpcMethod(...)
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

## Options for composite schemas directives

The following options build upon the previous ones to make annotating your schemas with [Composite Schemas spec](https://github.com/graphql/composite-schemas-spec/) directives more convenient.

### Composite schema entity references

You can create federation-style entity references using the `derive_field` option on messages. This allows you to reference entities from other subgraphs:

```proto
import "grafbase/options.proto";

message Product {
  // Basic usage: creates a user field that references User entity by id
  option (grafbase.graphql.derive_field) = {
    entity: "User",
    is: "{ id: user_id }"
  };

  // Custom relation field name: creates an owner field instead of user
  option (grafbase.graphql.derive_field) = {
    entity: "User",
    name: "owner"
    is: "{ id: owner_id }"
  };

  // Reference by non-id field
  option (grafbase.graphql.derive_field) = {
    entity: "Shop",
    is: "{ slug: shop_slug }"
  };

  string id = 1;
  string name = 2;

  string user_id = 3;
  string owner_id = 4;

  string shop_slug = 6;
}
```

This generates GraphQL with entity references:

```graphql
type Product {
  id: String
  name: String
  user_id: String
  user: User @derive @is(field: "{ id: user_id }")  # Automatically added reference field
  owner_id: String
  owner: User @derive @is(field: "{ id: owner_id }")  # Custom name for the reference
  shop_slug: String
  shop: Shop @derive @is(field: "{ slug: shop_slug }")
}

# Stub entities are automatically created
type User @key(fields: "id") {
  id: String
}

type Shop @key(fields: "slug") {
  slug: String
}
```

Composite (multiple fields) and list derives are also supported.

### Key

This is a shortcut for `output_field_directives` for `@key`, as in `option (grafbase.graphql.output_field_directives) = "@key(fields: \"id\")"`:

```proto
message User {
  option (grafbase.graphql.key) = { fields: "id" };
  option (grafbase.graphql.key) = { fields: "alias email" };

  string id = 1;
  string alias = 2;
  string email = 3;
}
```

Will generate:

```graphql
extend schema @link(url: "https://specs.grafbase.com/composite-schemas/v1", import: ["@key"])

# ...

type User @key(fields: "id") @key(fields: "alias email") {
  id: String!
  alias: String!
  email: String!
}
```

### Lookup

This is a shortcut for the `@link` and `@lookup` directives needed to define a lookup field:

```proto
service MyService {
  option (grafbase.graphql.default_to_query_fields) = true;

  rpc GetUser (GetUserRequest) returns (GetUserResponse) {
    option (grafbase.graphql.lookup) = {};
  }
}
```

Will generate:

```graphql
extend schema @link(url: "https://specs.grafbase.com/composite-schemas/v1", import: ["@lookup"])

# ...

type Query {
  GetUser(input: GetUserRequest) @lookup
}
```

### Argument @is directive

The `argument_is` option is a shortcut for adding an `@is` directive to RPC method input arguments. It's equivalent to using `argument_directives` with `@is(field: "...")`:

```proto
service MyService {
  option (grafbase.graphql.default_to_query_fields) = true;

  rpc GetUser (GetUserRequest) returns (GetUserResponse) {
    option (grafbase.graphql.lookup) = {};
    option (grafbase.graphql.argument_is) = "{ user_id: user.id }";
  }
}
```

Will generate:

```graphql
extend schema @link(url: "https://specs.grafbase.com/composite-schemas/v1", import: ["@is", "@lookup"])

# ...

type Query {
  GetUser(input: GetUserRequest @is(field: "{ user_id: user.id }")) @lookup
}
```

This is a shortcut for:
```proto
option (grafbase.graphql.argument_directives) = "@is(field: \"{ user_id: user.id }\")";
```

### Join fields

This option corresponds to a join with a gRPC method that isn't a lookup. It will define a field that will be resolved by calling the gRPC method, with data from the parent object used to populate the input message, through an `@require` directive.

```proto
message Product {
    option (grafbase.graphql.join_field) = {
        name: "parts",
        service: "products.ProductService",
        method: "GetProductParts",
        require: "{ product_id: id }"
    };

    string id = 1;For
    string name = 2;
    string sku = 3;
}

rpc ProductService {
    rpc GetProductParts(GetProductPartsRequest) returns (stream GetProductPartsResponse);
}

message GetProductPartsRequest {
    string product_id = 1;
}

message GetProductPartsResponse {
    repeated Part part = 1;
}

message Part {
    string id = 1;
    string name = 2;
    uint32 quantity_in_stock = 3;
}
```

Will generate:

```graphql
extend schema @link(url: "https://specs.grafbase.com/composite-schemas/v1", imports: ["@require"])

# ...

type Product {
    id: String!
    name: String!
    sku: String!
    parts(input: products_GetProductPartsRequest @require(field: "{ product_id: id }")): product_GetProductPartsResponse @grpcMethod(service: "products.ProductService", method: "GetProductParts")
}

type Part {
    id: ID!
    name: String!
    quantityInStock: Int!
}

type Mutation {
    product_GetProductParts(input: products_GetProductPartsRequest): product_GetProductPartsResponse
}
```

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
