# Grafbase PostgreSQL CLI

A command-line interface for introspecting PostgreSQL databases and generating GraphQL SDL compatible with the [postgres extension](https://grafbase.com/extensions/postgres).

## Overview

This CLI tool connects to a PostgreSQL database, introspects its schema (tables, columns, relationships, and types), and generates a GraphQL SDL representation that can be used with the Grafbase [postgres extension](https://grafbase.com/extensions/postgres).

## Installation

Download a binary for your platform from the [releases page](https://github.com/grafbase/extensions/releases?q=grafbase-postgres&expanded=true).

Or build from source:

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

## License

Apache-2.0
