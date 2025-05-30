# Grafbase PostgreSQL CLI

A command-line interface for introspecting PostgreSQL databases and generating GraphQL SDL compatible with the [postgres extension](https://grafbase.com/extensions/postgres).

## Overview

This CLI tool connects to a PostgreSQL database, introspects its schema (tables, columns, relationships, and types), and generates a GraphQL SDL representation that can be used with the Grafbase [postgres extension](https://grafbase.com/extensions/postgres).

## Installation

### Quick Install Script

You can install the latest version with a single command:

```bash
curl -fsSL https://raw.githubusercontent.com/grafbase/extensions/refs/heads/main/cli/postgres/install.sh | bash
```

This will automatically detect your system, download the appropriate binary, and install it to `~/.grafbase/bin`.

### Manual Installation

Download a binary for your platform from the [releases page](https://github.com/grafbase/extensions/releases?q=grafbase-postgres&expanded=true).

### From Source

```bash
# From source
cargo install --path .
```

## Usage

### Environment Variables

- `DATABASE_URL` - Connection string to your PostgreSQL database

If the current directory has a `.env` file stored with the `DATABASE_URL` environment variable, it will be used as the default value for the `--database-url` option.

Provide all required TLS parameters directly in the connection string. For TLS connections, add parameters like:

- `sslmode=verify-full`
- `sslrootcert=/path/to/ca.crt`
- `sslcert=/path/to/client.crt`
- `sslkey=/path/to/client.key`

Find example certificate files in our [test setup repository](https://github.com/grafbase/extensions/tree/main/docker/postgres-mtls/certs).

### Basic Command

```bash
grafbase-postgres --database-url "postgres://username:password@localhost:5432/mydatabase" introspect --config grafbase-postgres.toml
```

### Command Options

#### Global Options

- `-d, --database-url <DATABASE_URL>` - Connection string to the PostgreSQL database. Note that this argument must come before the subcommand (`introspect`) if you provide it.

#### Introspect Command

- `-c, --config <PATH>` - Specify configuration file for introspection. Defaults to `./grafbase-postgres.toml` if not provided.

## Examples

### Output SDL to Terminal

```bash
grafbase-postgres \
    --database-url "postgres://user:pass@localhost:5432/mydb" \
    introspect \
    --config grafbase-postgres.toml
```

### Save SDL to a File

```bash
grafbase-postgres \
    --database-url "postgres://user:pass@localhost:5432/mydb" \
    introspect \
    --config grafbase-postgres.toml > schema.graphql
```

## What Gets Introspected

The following database objects are introspected:
- Schemas
- Tables
- Views (normal and materialized)
- Columns (including data types and constraints)
- Primary keys and unique constraints
- Foreign keys (relationships between tables)
- Enums (custom enum types)

## How It Works

The tool:
1. Connects to the specified PostgreSQL database
2. Introspects the schema structure
3. Builds a database definition object
4. Renders the definition as GraphQL SDL
5. Outputs the SDL to stdout or a file

## Configuration

Configure the introspection command using a TOML configuration file. Include these essential settings:

```toml
# The URL of the extension, which appears at the top of the GraphQL SDL.
extension_url = "https://grafbase.com/extensions/postgres/0.3.0"

# The default schema, which we'll omit from the SDL output.
# Defaults to "public" if you don't specify it
default_schema = "public"

# The name of the database the virtual subgraph should use. This
# maps to a Postgres database name in your gateway configuration.
# Defaults to "default" if you don't specify it
database_name = "default"

# Enable mutations (write operations) globally for the whole database.
# Defaults to true if you omit this setting.
enable_mutations = true

# Enable queries (read operations) globally for the whole database.
# Defaults to true if you omit this setting.
enable_queries = true

# Schema allowlist: An array of schema names to include in the introspection.
# If provided, only schemas in this list will be included.
# If not defined or null, all schemas will be included (unless in the denylist).
# If empty, no schemas will be included.
schema_allowlist = null

# Schema denylist: An array of schema names to exclude from the introspection.
# Schemas in this list will be excluded even if they appear in the allowlist.
# This takes precedence over the schema_allowlist.
schema_denylist = []

# Configure schemas for this database. Key-value from schema name to configuration.
schemas = {}
```

### Schema Configuration

```toml
[schemas.public]

# Enable mutations (write operations) globally for the whole schema.
# Takes precedence over the global setting. Defaults to true if you omit this setting.
enable_mutations = true

# Enable queries (read operations) globally for the whole database.
# Takes precedence over the global setting. Defaults to true if you omit this setting.
enable_queries = true

# Table allowlist: An array of table names to include in the introspection.
# If provided, only tables in this list will be included for this schema.
# If not defined or null, all tables will be included (unless in the denylist).
# If empty, no tables will be included for the schema.
table_allowlist = null

# Table denylist: An array of table names to exclude from the introspection.
# Tables in this list will be excluded even if they appear in the allowlist.
# This takes precedence over the table_allowlist.
table_denylist = []

# Configure views for this schema.
views = {}

# Configure tables for this schema. Key-value from table name to configuration.
tables = {}
```

### Table Configuration

```toml
[schemas.public.tables.users]

# Enable mutations (write operations) for the table.
# Takes precedence over the global and schema settings.
# Defaults to true if you omit this setting.
enable_mutations = true

# Enable queries (read operations) for the table.
# Takes precedence over the global and schema settings.
# Defaults to true if you omit this setting.
enable_queries = true

# Table relations are always calculated from the database foreign keys.
# In cases like with table to view relations this is not possible, and
# you can define them manually from this map. Key/value from relation name
# to config.
relations = {}

# Configure derives for cross-database joins.
derives = {}
```

### View Configuration

PostgreSQL views require additional configuration because the information schema doesn't provide details about unique constraints, nullability, or relations. To make a view visible in your GraphQL SDL, you must define at least one unique key.

```toml
[schemas.public.views.restricted_users]

# Enable queries (read operations) for the the view.
# Takes precedence over the global and schema settings.
# Defaults to true if you omit this setting.
enable_queries = true

# Even if the underlying table has unique constraints, the database does not
# show them for the view presenting the data. Single column keys you can configure
# better through the columns map, but use this for compound keys.
# An array of arrays. Each array is a collection of columns forming the key. Order
# of the columns matter.
unique_keys = []

# The database does not have information on the nullability, or uniqueness
# of view columns. You can define column settings manually from this map.
# Key/value from column name to config.
columns = {}

# Views do not have foreign keys mapped to them, as tables do. You
# can define relations manualy from this map.
# Key/value from relation name to config.
relations = {}

# Configure derives for cross-database joins.
derives = {}
```

#### Unique Key Definitions

```toml
[schemas.public.views.my_view]

# The order of columns matters - match the order in the underlying query/table.
# Define compound keys like this:
unique_keys = [["user_name", "user_id"]]

# structure: schemas.<schema_name>.views.<view_name>.columns.<column_name>
[schemas.public.views.my_view.columns.user_name]

# Defaults to true if you omit this setting
nullable = false
# Define a single-column unique key here. Defaults to false if omitted.
unique = false
# Customize the GraphQL field name:
rename = "name_user"
# Add a description that appears as a comment in the GraphQL schema:
description = "The name of the user"

[schemas.public.views.my_view.columns.user_id]
nullable = false
```

The introspection will fail if you reference any non-existent schemas, views, or columns.

#### Relation Definitions

```toml
# structure: schemas.<schema_name>.views.<view_name>.relations.<relation_name>
[schemas.public.views.my_view.relations.my_view_to_my_table]

# The schema containing the referenced table or view.
# Defaults to "public" if omitted. Must exist.
referenced_schema = "public"

# Specify either a table or view. Must exist.
referenced_table = "my_table"

# List columns in the view that reference columns in the target
# table or view.
#
# Introspection fails if these columns don't exist.
referencing_columns = ["user_id", "user_name"]

# List columns in the target table or view that your view
# references.
#
# Introspection fails if these columns don't exist.
referenced_columns = ["id", "name"]
```

Define these relations in your config file to enable joins to and from your views.

### Derive Definitions

Our derives setup offers a powerful way to join data efficiently between multiple Postgres databases. You can use several approaches to enable joins across two or more Postgres databases.

The Grafbase Gateway prevents GraphQL N+1 problems in all cross-database joins by minimizing the number of queries needed to load data.

Let's explore some examples using these SQL schemas:

Database A:

```sql
CREATE TABLE "posts" (
  id INT PRIMARY KEY,
  title VARCHAR(255) NOT NULL,
  author_id INT NOT NULL
);
```

Database B:

```sql
CREATE TABLE "users" (
  id INT PRIMARY KEY,
  name VARCHAR(255) NOT NULL
)
```

We want to create a federated GraphQL schema that joins posts and users:

```graphql
type Post @key(fields: "id") {
  id: Int!
  title: String!
  authorId: Int!
  author: User
}

type User @key(fields: "id") {
  id: Int!
  name: String!
}
```

#### Entity Joins

The first approach uses GraphQL federation spec entity joins. This method requires both subgraph schemas to define the same type. Database A needs a view that represents unique users, which we create with:

```sql
CREATE VIEW "users" AS SELECT DISTINCT(author_id) AS id FROM posts ORDER BY author_id;
```

Add this configuration for the grafbase-postgres CLI extension before introspection:

```toml
[schemas.public.views.users.columns.id]
nullable = false
unique = true
```

This gives subgraph A the following types:

```graphql
type Post @key(fields: "id") {
  id: Int!
  title: String!
  authorId: Int!
  author: User!
}

type User @key(fields: "id") {
  id: Int!
}
```

When you compose this with subgraph B, you can efficiently join posts and users.

#### Deriving

Creating an extra view to join data between databases often adds maintenance overhead and can slow down performance. Instead, use the composite spec `@is` and `@derive` directives. Define derives for subgraph A:

```toml
[schemas.public.tables.posts.derives.author]
referenced_type = "User"
fields = { id = "authorId" }
```

This tells the system: for the `posts` table, create a derived field `author` that loads a single `User` entity. The `id` field of the User type maps to the `authorId` field of the `Post` type.

Introspection then produces these types:

```graphql
type Post @key(fields: "id") {
  id: Int!
  title: String!
  authorId: Int!
  author: User! @derive @is(field: "{ id: authorId }")
}

type User @key(fields: "id") {
  id: Int!
}
```

The introspection creates a User type with fields needed for joining and a derived relation field with the corresponding directives. Compose this with the other subgraph to create a federated graph that joins posts to users across databases.

#### One to Many

We're currently working on support for one-to-many joins with deriving. For now, use the entity join approach if you need to fetch many entities from another graph.

Add a view to the database with the posts table:

```sql
CREATE VIEW "users" AS SELECT DISTINCT(author_id) AS id FROM posts ORDER BY author_id;
```

Then add this configuration to create a relation between the users view and posts table:

```toml
[schemas.public.views.users.relations.users_to_posts]
referenced_schema = "public"
referenced_table = "posts"
referencing_columns = ["id"]
referenced_columns = ["author_id"]
```

Introspection produces this SDL:

```graphql
type Post @key(fields: "id") {
  id: Int!
  title: String!
  authorId: Int!
  author: User!
}

type User @key(fields: "id") {
  id: Int!
  posts: [Post!]!
}
```

When you compose this with the other subgraph:

```graphql
type User @key(fields: "id") {
  id: Int!
  name: String!
}
```

You get this federated graph:

```graphql
type Post @key(fields: "id") {
  id: Int!
  title: String!
  authorId: Int!
  author: User!
}

type User @key(fields: "id") {
  id: Int!
  name: String!
  posts: [Post!]!
}
```

### Schema and Table Filtering

You can control which database schemas and tables are included in the introspection process using allowlist and denylist options.

#### Schema Filtering

You can include or exclude specific database schemas using the following options:

- `schema_allowlist`: An array of schema names to include. If provided, only schemas in this list will be included in the introspection. If empty, no schemas will be included. If not defined, all schemas will be included.
- `schema_denylist`: An array of schema names to exclude. Schemas in this list will be excluded from introspection, even if they appear in the allowlist.

```toml
# Example of schema filtering in config.toml
extension_url = "https://grafbase.com/extensions/postgres/0.4.7"
schema_allowlist = ["public", "app"]
schema_denylist = ["internal"]
```

#### Table Filtering

Within each schema, you can include or exclude specific tables using these options:

- `table_allowlist`: An array of table names to include. If provided, only tables in this list will be included for that schema. If empty, no tables will be included for the schema. If not defined, all tables will be included.
- `table_denylist`: An array of table names to exclude. Tables in this list will be excluded, even if they appear in the allowlist.

```toml
# Example of table filtering in config.toml
extension_url = "https://grafbase.com/extensions/postgres/0.4.9"

[schemas.public]
table_allowlist = ["users", "posts"]

[schemas.internal]
table_denylist = ["audit_logs", "system_metrics"]
```

When both allowlist and denylist are specified, the denylist takes precedence. For example, if a table is included in both `table_allowlist` and `table_denylist`, it will be excluded from the introspection.

## License

Apache-2.0
