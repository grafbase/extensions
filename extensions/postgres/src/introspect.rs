mod enums;
mod field_mapping;
mod foreign_keys;
mod keys;
mod schemas;
mod tables;

use std::collections::HashMap;

use grafbase_database_definition::{DatabaseDefinition, KeyType, RelationKind, ScalarKind};
use grafbase_sdk::types::SubgraphSchema;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PgDatabase {
    pub name: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PgTable {
    pub name: String,
    pub schema: String,
    #[serde(default)]
    pub kind: RelationKind,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PgColumn {
    pub name: String,
    pub r#type: ScalarKind,
    pub enum_schema: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PgEnum {
    pub name: String,
    pub schema: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PgEnumVariant {
    pub name: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PgRelation {
    pub name: String,
    #[serde(default)]
    pub fields: Vec<String>,
    #[serde(default)]
    pub references: Vec<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PgKey {
    pub fields: Vec<String>,
    pub r#type: KeyType,
}

pub fn from_sdl(subgraph_schemas: Vec<SubgraphSchema<'_>>) -> HashMap<String, DatabaseDefinition> {
    let mut result = HashMap::new();

    for schema in subgraph_schemas {
        let Some(pg_database) = schema
            .directives()
            .find(|d| d.name() == "pgDatabase")
            .and_then(|d| d.arguments::<PgDatabase>().ok())
        else {
            continue;
        };

        let mut database_definition = DatabaseDefinition::new(pg_database.name);

        schemas::introspect_sdl(&schema, &mut database_definition);
        enums::introspect_sdl(&schema, &mut database_definition);
        tables::introspect_sdl(&schema, &mut database_definition);
        foreign_keys::introspect_sdl(&schema, &mut database_definition);
        keys::introspect_sdl(&schema, &mut database_definition);
        field_mapping::introspect(&schema, &mut database_definition);

        result.insert(schema.subgraph_name().to_string(), database_definition);
    }

    result
}
