use crate::{
    lens::{Lens, LensResult},
    model::{self, StreamId},
};

#[tauri::command]
pub async fn sql(lens: tauri::State<'_, Lens>, query: String) -> LensResult<()> {
    lens.sql(&query).await?;
    Ok(())
}

#[tauri::command]
pub async fn sql_stream(lens: tauri::State<'_, Lens>, query: String) -> LensResult<StreamId> {
    lens.stream(&query).await
}

#[tauri::command]
pub async fn stream_next(
    lens: tauri::State<'_, Lens>,
    stream_id: StreamId,
) -> LensResult<Option<Vec<model::Row>>> {
    lens.stream_next(stream_id).await
}

#[tauri::command]
pub async fn list_databases(lens: tauri::State<'_, Lens>) -> LensResult<Vec<model::Database>> {
    let context = lens.context();
    let mut catalogs = Vec::new();

    for catalog_name in context.catalog_names() {
        let Some(cat) = context.catalog(&catalog_name) else {
            continue;
        };

        let mut schemas = Vec::new();

        for schema_name in cat.schema_names() {
            let Some(schema) = cat.schema(&schema_name) else {
                continue;
            };

            let mut tables = Vec::new();

            for table_name in schema.table_names() {
                let Some(table) = schema.table(&table_name).await? else {
                    continue;
                };

                let schema = table.schema();

                tables.push(model::Table {
                    name: table_name.clone(),
                    schema,
                });
            }

            schemas.push(model::Schema {
                name: schema_name.clone(),
                tables,
            });
        }

        catalogs.push(model::Database {
            name: catalog_name.clone(),
            schemas,
        });
    }

    Ok(catalogs)
}
