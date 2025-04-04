# Grafbase Postgres Extension

This Postgres extension for the Grafbase Gateway acts as a complete virtual subgraph that you can compose into your federated schema.

Use the [`grafbase-postgres`](https://github.com/grafbase/extensions/tree/julius/ywmxqwoumunr/cli/postgres) introspection tool to generate a subgraph schema from your Postgres database. You can then compose this subgraph into your federated schema. Commit the generated schema to your repository and re-introspect the database periodically to keep the schema up-to-date.

The introspection process generates a complete set of types representing the data in your Postgres database, including entities, relationships, and scalar fields. Use these types to query and mutate data in your Postgres database via the Grafbase Gateway.

Generally, every operation executed through the extension generates exactly one SQL statement. This includes relationships, which the extension represents as lateral joins. The database handles JSON rendering; the extension manages query generation and data retrieval.

**Note:** This extension is currently under development. Known missing features include:

*   TLS support for database connections
*   Support for types from Postgres extensions (e.g., PostGIS)
*   Real pagination cursors and page info (currently returns dummy values)

## Getting Started

### Introspection Tool (`grafbase-postgres`)

Currently, you must compile the `grafbase-postgres` introspection tool from source (refer to its [repository](https://github.com/grafbase/extensions/tree/julius/ywmxqwoumunr/cli/postgres) for instructions). After successful compilation, copy the resulting binary to a directory in your system's `PATH`.

*Future versions will offer easier installation, likely integrated with the Grafbase CLI.*

### Building the Extension (Wasm)

To use the extension with the Grafbase Gateway, you first need to build the Wasm component.

1.  Build the extension:
    ```bash
    grafbase extension build
    ```
2.  This command creates a `build` directory containing the Wasm module and its manifest:
    ```
    build/
    ├── extension.wasm
    └── manifest.json
    ```

### Gateway Configuration

Configure your Grafbase Gateway to use the extension in your `grafbase.config.toml`.

**Using a Published Version (Recommended when available):**

Specify the desired version (replace `0.1` when newer versions are released):

```toml
# grafbase.config.toml
[extensions.postgres]
version = "0.1"
```

**Using a Local Build:**

If you built the extension manually, point the gateway to the build directory:

```toml
# grafbase.config.toml
[extensions.postgres]
path = "/path/to/your/build" # Update this path
```

## Building From Source

As mentioned above, you currently need to build the Wasm module locally.

1.  Clone the repository containing this extension.
2.  Navigate to the extension's directory.
3.  Run the build command:
    ```bash
    grafbase extension build
    ```
4.  Configure your gateway to use the generated artifacts in the `build` directory as shown in the [Gateway Configuration](#gateway-configuration) section.

## Testing

To run the tests:

1.  Start the test database using Docker Compose:
    ```bash
    docker compose up -d
    ```
2.  Execute the test suite using Cargo:
    ```bash
    cargo test
    ```

### Faster Test Execution

To speed up a full test run, you can pre-compile the extension:

```bash
grafbase extension build
```

Then, run the tests with the `PREBUILT_EXTENSION` environment variable set:

```bash
PREBUILT_EXTENSION=1 cargo test
```

## Configuration

Configure the Postgres extension within your `grafbase.config.toml` under the `[extensions.postgres.config]` section. See the [Grafbase Gateway configuration documentation](https://grafbase.com/docs/reference/gateway/configuration/extensions) for general extension configuration details.

```toml
# Example configuration within grafbase.config.toml
[extensions.postgres]
  # version = "0.1" # Or path = "..."

[extensions.postgres.config]
[[extensions.postgres.config.databases]]
# Optional: Specify a name if connecting multiple databases.
# This name links the database connection to a specific subgraph.
name = "default"
# Use environment variables for sensitive parts like passwords
url = "postgres://user:password@host:port/database"

[extensions.postgres.config.databases.pool]
# Maximum number of connections (default: 10)
max_connections = 10
# Minimum number of idle connections maintained (default: 0)
min_connections = 0
# Maximum idle time before closing a connection (ms, default: 600000 / 10 min)
idle_timeout_ms = 600000
# Maximum time to wait for a connection from the pool (ms, default: 30000 / 30 sec)
acquire_timeout_ms = 30000
# Maximum lifetime of a connection (ms, default: 1800000 / 30 min)
max_lifetime_ms = 1800000
```

## Introspection

Use the `grafbase-postgres introspect` command to generate a GraphQL schema definition (SDL) from your database:

```bash
# Ensure grafbase-postgres is in your PATH
# Replace connection details and version as needed
grafbase-postgres \
    -d "postgres://postgres:grafbase@localhost:5432/postgres" \
    introspect \
    -v 0.1.0 > subgraph.graphql
```

This command generates a GraphQL schema (`subgraph.graphql` in this example). You can then publish this schema as a virtual subgraph to your federated graph using the [Grafbase CLI `publish` command](https://grafbase.com/docs/reference/grafbase-cli/publish):

```bash
# Replace with your graph details
grafbase publish \
    --name postgres \
    my-org/my-graph@main \
    -m "Add postgres subgraph" \
    --virtual \
    --schema subgraph.graphql
```

## Generated Schema (Queries and Types)

The introspection process reads tables, views (TODO: Add view support), and enums from the specified database schema to generate a GraphQL SDL.

Consider this example PostgreSQL schema:

```sql
CREATE TABLE "users" (
  id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  username VARCHAR(255) NOT NULL,
  email VARCHAR(255) NOT NULL,
  metadata JSONB DEFAULT '{}'
);

CREATE TABLE profiles (
  id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  user_id BIGINT NOT NULL REFERENCES "users"(id) ON DELETE CASCADE,
  first_name VARCHAR(100),
  last_name VARCHAR(100)
);
```

The introspection tool generates the following GraphQL types based on the tables above:

```graphql
type Profile
  @pgTable(name: "profiles")
  @pgKey(fields: ["id"], type: PRIMARY)
{
  id: BigInt! @pgColumn(name: "id", type: BIGINT)
  userId: BigInt! @pgColumn(name: "user_id", type: BIGINT)
  firstName: String @pgColumn(name: "first_name", type: VARCHAR)
  lastName: String @pgColumn(name: "last_name", type: VARCHAR)
  user: User! @pgRelation(name: "profiles_user_id_fkey", fields: ["userId"], references: ["id"])
}

type User
  @pgTable(name: "users")
  @pgKey(fields: ["id"], type: PRIMARY)
{
  id: BigInt! @pgColumn(name: "id", type: BIGINT)
  username: String! @pgColumn(name: "username", type: VARCHAR)
  email: String! @pgColumn(name: "email", type: VARCHAR)
  metadata: JSON @pgColumn(name: "metadata", type: JSONB)
  profiles(
    filter: ProfileFilterInput,
    first: Int,
    last: Int,
    before: String,
    after: String,
    orderBy: [ProfileOrderByInput!],
  ): ProfileConnection! @pgRelation(name: "profiles_user_id_fkey")
}
```

**Key Generation Principles:**

*   **Naming:** Field names default to camelCase, and type names default to PascalCase. The original database names are preserved in the `@pgTable` and `@pgColumn` directives.
*   **Schemas:** If your database uses multiple PostgreSQL schemas, the directives (e.g., `@pgTable(name: "users", schema: "public")`) will include the schema name.
*   **Relationships:** The tool generates fields for foreign key relationships using the `@pgRelation` directive. The side defining the foreign key constraint includes `fields` and `references` arguments; the other side represents the inverse relationship.
*   **JSON Types:** Columns with `JSON` or `JSONB` types map to the `JSON` scalar type in the SDL.
    *   If your JSON data has a consistent structure, you can replace the `JSON` scalar with a custom GraphQL object type *after* introspection. **Note:** Queries will fail if the database returns JSON that doesn't match your custom type definition.
*   **Customization:** You can rename generated types and fields after introspection. However, you **must** keep the original database object names within the `@pgTable`, `@pgColumn`, and `@pgRelation` directives. Ensure you also update any corresponding input types if you rename elements.
*   **Pruning:** You can safely remove unused queries, mutations, and their associated input/output types from the generated schema if they are not needed in your API.

### Queries

The introspection generates queries for fetching single records and collections.

```graphql
# Example generated queries
type Query {
  # Fetch a single user by primary/unique key
  user(
    lookup: UserLookupInput!,
  ): User @pgSelectOne

  # Fetch a collection of users with filtering, ordering, and pagination
  users(
    filter: UserFilterInput,
    first: Int,
    last: Int,
    before: String,
    after: String,
    orderBy: [UserOrderByInput!],
  ): UserConnection! @pgSelectMany
}
```

*   **Single Record (`@pgSelectOne`):** Fetches a unique row (e.g., `user`). Its `lookup` argument accepts fields corresponding to the table's primary key or unique constraints. For composite keys, the tool generates specific input types.
*   **Collections (`@pgSelectMany`):** Fetches multiple rows (e.g., `users`). It supports filtering (`filter`), ordering (`orderBy`), and cursor-based pagination (`first`, `last`, `before`, `after`).
*   **Performance:** When you query fields representing relationships, the extension generates efficient SQL joins (specifically lateral joins). The extension guarantees execution of exactly one SQL query per incoming GraphQL request, preventing the N+1 query problem.
*   **Pagination:** Queries returning multiple items (including nested one-to-many relations) expose standard GraphQL connection types with pagination arguments (`first`, `last`, `before`, `after`) and `pageInfo`. (**Note:** Cursors and `pageInfo` currently return dummy values, see Missing Features).

### Mutations

The introspection also generates standard CRUD mutations.

```graphql
# Example generated mutations
type Mutation {
  # Create a single user
  userCreate(
    input: UserCreateInput!,
  ): UserCreatePayload! @pgInsertOne

  # Create multiple users
  userCreateMany(
    input: [UserCreateInput!]!,
  ): UserCreateManyPayload! @pgInsertMany

  # Update a single user (identified by lookup)
  userUpdate(
    lookup: UserLookupInput!,
    input: UserUpdateInput!,
  ): UserUpdatePayload! @pgUpdateOne

  # Update multiple users (identified by filter)
  userUpdateMany(
    filter: UserFilterInput,
    input: UserUpdateInput!,
  ): UserUpdateManyPayload! @pgUpdateMany

  # Delete a single user (identified by lookup)
  userDelete(
    lookup: UserLookupInput!,
  ): UserDeletePayload! @pgDeleteOne

  # Delete multiple users (identified by filter)
  userDeleteMany(
    filter: UserFilterInput,
  ): UserDeleteManyPayload! @pgDeleteMany
}
```

*   **Operations:** The tool generates mutations for single-row (`@pgInsertOne`, `@pgUpdateOne`, `@pgDeleteOne`) and multi-row (`@pgInsertMany`, `@pgUpdateMany`, `@pgDeleteMany`) operations.
*   **Returning Data:** All mutations support a `returning` selection set, allowing you to fetch data about the affected rows within the same database transaction.
*   **Performance:** Each mutation executes as a single SQL statement.

### Supported PostgreSQL Versions

We primarily test against the latest stable PostgreSQL version. The extension relies on SQL features, particularly JSON/JSONB functions, available in PostgreSQL. Therefore, the minimum supported version is **PostgreSQL 9.4**.
