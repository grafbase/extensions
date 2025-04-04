mod builder;
mod create_many;
mod create_one;
mod delete_many;
mod delete_one;
mod find_many;
mod find_one;
mod query;
mod update_many;
mod update_one;

use grafbase_database_definition::Operation;
use grafbase_sdk::{SdkError, types::Data};

pub(super) fn execute(ctx: super::Context<'_>) -> Result<Data, SdkError> {
    match ctx.operation() {
        Operation::FindOne(table_id) => find_one::execute(ctx, table_id),
        Operation::FindMany(table_id) => find_many::execute(ctx, table_id),
        Operation::DeleteOne(table_id) => delete_one::execute(ctx, table_id),
        Operation::DeleteMany(table_id) => delete_many::execute(ctx, table_id),
        Operation::CreateOne(table_id) => create_one::execute(ctx, table_id),
        Operation::CreateMany(table_id) => create_many::execute(ctx, table_id),
        Operation::UpdateOne(table_id) => update_one::execute(ctx, table_id),
        Operation::UpdateMany(table_id) => update_many::execute(ctx, table_id),
    }
}
