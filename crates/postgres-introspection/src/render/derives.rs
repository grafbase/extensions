use std::collections::{BTreeMap, btree_map::Entry};

use anyhow::bail;
use grafbase_database_definition::{DatabaseDefinition, TableWalker};
use itertools::Itertools;

use crate::{
    config::{Config, DeriveConfig},
    render::ast::r#type::Type,
};

use super::ast::{
    directive::{Argument, Directive},
    field::Field,
};

#[derive(Default)]
pub struct Derives<'a> {
    pub types: BTreeMap<&'a str, Type<'a>>,
    pub fields: BTreeMap<&'a str, Vec<Field<'a>>>,
}

/// Validates the derives in the configuration, ensuring the types they would create are not already
/// defined. Returns the needed fields and type definitions based on the config.
pub(super) fn generate<'a>(
    database_definition: &'a DatabaseDefinition,
    config: &'a Config,
) -> anyhow::Result<Derives<'a>> {
    let mut derives = Derives::default();

    for (schema_name, schema) in config.schemas.iter() {
        for (table_name, table) in schema.tables.iter() {
            for (field_name, derive) in table.derives.iter() {
                if database_definition
                    .find_table_for_client_type(&derive.referenced_type)
                    .is_some()
                {
                    bail!("The type {} is already defined as a table.", derive.referenced_type);
                }

                let Some(referencing_table) = database_definition.get_table(schema_name, table_name) else {
                    bail!("Table {table_name} not found");
                };

                let required_field = match derives.types.entry(&derive.referenced_type) {
                    Entry::Occupied(entry) => {
                        let r#type = entry.into_mut();
                        add_field_to_type(derive, referencing_table, r#type)?
                    }
                    Entry::Vacant(entry) => {
                        let mut r#type = Type::new(&derive.referenced_type);
                        let required_field = add_field_to_type(derive, referencing_table, &mut r#type)?;

                        entry.insert(r#type);

                        required_field
                    }
                };

                let r#type = if required_field {
                    format!("{}!", derive.referenced_type)
                } else {
                    derive.referenced_type.clone()
                };

                let mut field = Field::new(field_name, r#type);
                field.push_directive(Directive::new("derive"));

                let fields = derive
                    .fields
                    .iter()
                    .map(|(other_side, this_side)| format!("{other_side}: {this_side}"))
                    .join(" ");

                let fields = format!("{{ {fields} }}");

                let mut is = Directive::new("is");
                is.push_argument(Argument::string("field", fields));
                field.push_directive(is);

                match derives.fields.entry(referencing_table.client_name()) {
                    Entry::Vacant(vacant_entry) => {
                        vacant_entry.insert(vec![field]);
                    }
                    Entry::Occupied(mut occupied_entry) => {
                        occupied_entry.get_mut().push(field);
                    }
                }
            }
        }
    }

    Ok(derives)
}

fn add_field_to_type<'a>(
    derive: &'a DeriveConfig,
    referencing_table: TableWalker<'a>,
    r#type: &mut Type<'a>,
) -> anyhow::Result<bool> {
    let mut field_is_required = false;
    let mut fields = Vec::new();

    for (referencing, referenced) in derive.fields.iter() {
        let Some(column) = referencing_table.find_database_column_for_field(referencing) else {
            bail!(
                "Field {referencing} not found in type {}",
                referencing_table.client_name()
            );
        };

        let field = Field::new(referenced, column.client_type(None).unwrap());
        r#type.push_field(field);

        if !field_is_required && !column.is_nullable() {
            field_is_required = true;
        }

        fields.push(referenced.as_str());
    }

    let fields = fields.join(" ");
    let mut directive = Directive::new("key");

    directive.push_argument(Argument::string("fields", fields));
    r#type.push_directive(directive);

    Ok(field_is_required)
}
