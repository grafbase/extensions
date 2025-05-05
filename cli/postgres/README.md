# Grafbase PostgreSQL CLI

A command-line interface for introspecting PostgreSQL databases and generating GraphQL SDL compatible with the [postgres extension](https://grafbase.com/extensions/postgres).

## Overview

This CLI tool connects to a PostgreSQL database, introspects its schema (tables, columns, relationships, and types), and generates a GraphQL SDL representation that can be used with the Grafbase [postgres extension](https://grafbase.com/extensions/postgres).

## Installation

```bash
# From source
cargo install --path .
```

## Usage

### Environment Variables

- `DATABASE_URL` - Connection string to your PostgreSQL database

### Basic Command

```bash
grafbase-postgres --database-url "postgres://username:password@localhost:5432/mydatabase" introspect --extension-version 1.0.0 --database-name mydb
```

### Command Options

#### Global Options

- `-d, --database-url <DATABASE_URL>` - Connection string to the PostgreSQL database. Note that this argument must come before the subcommand (`introspect`) if you provide it.

#### Introspect Command

- `-o, --output-file <PATH>` - Write the SDL output to a file instead of stdout
- `-d, --database-name <NAME>` - Name for the database in the GraphQL SDL (default: "default")
- `-s, --default-schema <SCHEMA>` - Default schema to use (will be omitted from definitions) (default: "public")
- `-u, --extension-url <URL>` - URL to the Grafbase PostgreSQL extension
- `-v, --extension-version <VERSION>` - Version of the Grafbase PostgreSQL extension (semver)

**Note**: Either `--extension-url` or `--extension-version` must be provided.

## Examples

### Output SDL to Terminal

```bash
grafbase-postgres --database-url "postgres://user:pass@localhost:5432/mydb" introspect --extension-version 1.0.0
```

### Save SDL to a File

```bash
grafbase-postgres --database-url "postgres://user:pass@localhost:5432/mydb" introspect --extension-version 1.0.0 --output-file schema.graphql
```

### Use a Custom Database Name and Schema

```bash
grafbase-postgres --database-url "postgres://user:pass@localhost:5432/mydb" introspect --extension-version 1.0.0 --database-name production --default-schema app
```

## What Gets Introspected

The following database objects are introspected:
- Schemas
- Tables
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

## Advanced Usage

### Using a Custom Extension URL

If you need to use a custom extension URL instead of the official version:

```bash
grafbase-postgres --database-url "postgres://user:pass@localhost:5432/mydb" introspect --extension-url "https://example.com/my-postgres-extension"
```

## License

Apache-2.0
