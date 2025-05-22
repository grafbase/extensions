use grafbase_database_definition::TableWalker;
use indexmap::IndexMap;
use serde::Deserialize;
use std::collections::BTreeMap;

/// Returns the default value for enable_queries configuration.
fn default_enable_queries() -> bool {
    true
}

/// Returns the default value for enable_mutations configuration.
fn default_enable_mutations() -> bool {
    true
}

/// Represents the overall configuration for the application.
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// Determines whether mutations (write operations) are enabled for this configuration.
    #[serde(default = "default_enable_mutations")]
    pub enable_mutations: bool,
    /// Determines whether queries (read operations) are enabled for this configuration.
    #[serde(default = "default_enable_queries")]
    pub enable_queries: bool,
    /// The name of the database this virtual subgraph uses (the name from grafbase.toml).
    #[serde(default = "default_database_name")]
    pub database_name: String,
    /// The default schema name to use, e.g. objects in this schema will not have an
    /// explicit mention of the schema in their SDL counterparts.
    #[serde(default = "default_default_schema")]
    pub default_schema: String,
    /// The URL of the extension printed in the SDL file.
    pub extension_url: String,
    /// Configuration details for each schema within the database, keyed by schema name.
    #[serde(default)]
    pub schemas: BTreeMap<String, SchemaConfig>,
    /// Optional list of schemas to include in the GraphQL schema.
    /// If this list is populated, only schemas in this list will be included.
    /// If empty, all schemas will be included.
    #[serde(default)]
    pub schema_allowlist: Vec<String>,
    /// Optional list of schemas to exclude from the GraphQL schema.
    /// If this list is populated, schemas in this list will be excluded even if they are in the allowlist.
    /// This takes precedence over the allowlist.
    #[serde(default)]
    pub schema_denylist: Vec<String>,
}

impl Config {
    /// Determines whether mutations (write operations) are allowed for the specified table.
    pub fn mutations_allowed(&self, table: TableWalker<'_>) -> bool {
        if !table.mutations_allowed() {
            return false;
        }

        let Some(schema_config) = self.schemas.get(table.schema()) else {
            return self.enable_mutations;
        };

        let Some(table_config) = schema_config.tables.get(table.database_name()) else {
            return schema_config.enable_mutations.unwrap_or(self.enable_mutations);
        };

        table_config
            .enable_mutations
            .or(schema_config.enable_mutations)
            .unwrap_or(self.enable_mutations)
    }

    /// Determines whether queries (read operations) are allowed for the specified table.
    pub fn queries_allowed(&self, table: TableWalker<'_>) -> bool {
        let Some(schema_config) = self.schemas.get(table.schema()) else {
            return self.enable_queries;
        };

        if table.relation_kind().is_view() {
            let Some(view_config) = schema_config.views.get(table.database_name()) else {
                return schema_config.enable_queries.unwrap_or(self.enable_queries);
            };

            view_config
                .enable_queries
                .or(schema_config.enable_queries)
                .unwrap_or(self.enable_queries)
        } else {
            let Some(table_config) = schema_config.tables.get(table.database_name()) else {
                return schema_config.enable_queries.unwrap_or(self.enable_queries);
            };

            table_config
                .enable_queries
                .or(schema_config.enable_queries)
                .unwrap_or(self.enable_queries)
        }
    }

    pub fn is_schema_included(&self, schema: &str) -> bool {
        !self.schema_denylist.contains(&schema.to_string())
            && (self.schema_allowlist.is_empty() || self.schema_allowlist.contains(&schema.to_string()))
    }

    /// Determines whether a table is included in the GraphQL schema based on the configuration.
    /// A table is included if:
    /// 1. It's not in the schema's table_denylist
    /// 2. Either the schema's table_allowlist is empty or the table is in the allowlist
    pub fn is_table_included(&self, schema: &str, table: &str) -> bool {
        let Some(schema_config) = self.schemas.get(schema) else {
            return true;
        };

        !schema_config.table_denylist.contains(&table.to_string())
            && (schema_config.table_allowlist.is_empty() || schema_config.table_allowlist.contains(&table.to_string()))
    }
}

/// Returns the default database name.
fn default_database_name() -> String {
    "default".to_string()
}

/// Returns the default schema name.
fn default_default_schema() -> String {
    "public".to_string()
}

/// Represents the overrides for a database schema.
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct SchemaConfig {
    /// Determines whether mutations (write operations) are enabled for this schema.
    pub enable_mutations: Option<bool>,
    /// Determines whether queries (read operations) are enabled for this schema.
    pub enable_queries: Option<bool>,
    /// Configuration details for each view within the database, keyed by view name.
    #[serde(default)]
    pub views: BTreeMap<String, ViewConfig>,
    /// Configuration overrides for each table within the schema, keyed by table name.
    #[serde(default)]
    pub tables: BTreeMap<String, TableConfig>,
    /// Optional list of tables to include in the GraphQL schema.
    /// If this list is populated, only tables in this list will be included.
    /// If empty, all tables will be included.
    #[serde(default)]
    pub table_allowlist: Vec<String>,
    /// Optional list of tables to exclude from the GraphQL schema.
    /// If this list is populated, tables in this list will be excluded even if they are in the allowlist.
    /// This takes precedence over the allowlist.
    #[serde(default)]
    pub table_denylist: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct TableConfig {
    /// Determines whether mutations (write operations) are enabled for this table.
    pub enable_mutations: Option<bool>,
    /// Determines whether queries (read operations) are enabled for this table.
    pub enable_queries: Option<bool>,
    /// Configuration details for relationships originating from this view, keyed by relationship name.
    #[serde(default)]
    pub relations: BTreeMap<String, RelationConfig>,
    /// Configuration for derived fields in this table, keyed by derive name.
    #[serde(default)]
    pub derives: BTreeMap<String, DeriveConfig>,
}

/// Represents the configuration settings for a specific database relation (e.g., a view).
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ViewConfig {
    /// Determines whether mutations (write operations) are enabled for this table.
    pub enable_mutations: Option<bool>,
    /// Determines whether queries (read operations) are enabled for this table.
    pub enable_queries: Option<bool>,
    /// Optional list of unique key constraints, where each constraint is a list of column names.
    pub unique_keys: Option<Vec<Vec<String>>>,
    /// Configuration details for each column within the relation, keyed by column name.
    #[serde(default)]
    pub columns: BTreeMap<String, ColumnConfig>,
    /// Configuration details for relationships originating from this view, keyed by relationship name.
    #[serde(default)]
    pub relations: BTreeMap<String, RelationConfig>,
    /// Configuration for derived fields in this table, keyed by derive name.
    #[serde(default)]
    pub derives: BTreeMap<String, DeriveConfig>,
}

/// Represents the configuration for a derived field within a table or view.
///
/// A derived field allows you to enable joins between subgraphs. By defining a derived field in
/// this database, the introspection generates a virtual type, which then composes with the full
/// federated schema.
///
/// For example:
///
/// ```toml
/// # In your config.toml file:
///
/// [schemas.public.tables.Post.derives.author]
/// referenced_type = "User"
/// field = { author_id = "id" }
/// ```
///
/// This then is reflected in the generated schema as follows:
///
/// ```graphql
/// type Post @key(fields: "id") {
///   id: ID!
///   title: String!
///   email: String!
///   # introspection will create this field
///   author: User! @derive @is(field: "{ author_id: id }")
/// }
///
/// """
/// Introspection will create this type.
/// """
/// type User @key(fields: "id") {
///   id: ID!
/// }
/// ```
///
/// Keep in mind that you do not need to define derives for types that are already defined in the
/// schema. The introspection process will detect the relationship between the types through
/// foreign keys. Also, if you have the same type in two different databases, automatically utilize the
/// Apollo federation keys to implement an entity join:
///
/// Subgraph A:
///
/// ```
/// type User @key(fields: "id") {
///   id: ID!
///   name: String!
/// }
/// ```
///
/// Subgraph B:
///
/// ```
/// type User @key(fields: "id") {
///   id: ID!
///   socialSecurityNumber: String!
/// }
/// ```
///
/// The composition will combine these User types into a single User type without extra
/// configuration.
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct DeriveConfig {
    /// The type the derived field points to.
    pub referenced_type: String,
    /// A map of referencing field to referenced field.
    pub fields: IndexMap<String, String>,
}

/// Represents the configuration for a specific column within a view.
#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct ColumnConfig {
    /// Specifies whether the column can contain null values. Defaults to `true`.
    #[serde(default = "default_nullable")]
    pub nullable: bool,
    /// Specifies whether the column values must be unique. Defaults to `false`.
    #[serde(default = "default_unique")]
    pub unique: bool,
    /// An optional new name for the column. If `None`, the original name is used.
    pub rename: Option<String>,
    /// An optional description for the column.
    pub description: Option<String>,
}

/// Represents the configuration for a relationship defined within a view.
#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct RelationConfig {
    /// The name of the schema containing the referenced table. Defaults to the `public`.
    #[serde(default = "default_default_schema")]
    pub referenced_schema: String,
    /// The name of the table (or view) that this relationship points to.
    pub referenced_table: String,
    /// The list of column names in the current view that form the foreign key.
    pub referencing_columns: Vec<String>,
    /// The list of column names in the referenced table that form the primary/unique key.
    pub referenced_columns: Vec<String>,
}

// Helper function for default nullable value
fn default_nullable() -> bool {
    true
}

// Helper function for default unique value
fn default_unique() -> bool {
    false
}
