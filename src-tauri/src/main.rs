// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use datafusion::prelude::*;

struct Lens {
    ctx: SessionContext,
}

mod model {
    use serde::Serialize;
    use std::sync::Arc;

    /// A database (or catalog) registered in DataFusion' context
    #[derive(Debug, Serialize)]
    pub struct Database {
        /// Name of the database
        pub name: String,

        /// List of [`Schema`] schemas for this database
        pub schemas: Vec<Schema>,
    }

    /// A schema registered in DataFusion' context
    #[derive(Debug, Serialize)]
    pub struct Schema {
        /// Name of the schema
        pub name: String,

        /// List [`Table`] tables for this schema
        pub tables: Vec<Table>,
    }

    /// Represents a table registered in DataFusion' context
    #[derive(Debug, Serialize)]
    pub struct Table {
        /// Name of the table
        pub name: String,

        /// Associated DataFusion [`Schema`] of this table
        pub schema: Arc<datafusion::arrow::datatypes::Schema>,
    }
}

pub struct LensError(pub(crate) Box<dyn std::error::Error>);
pub type LensResult<T, E = LensError> = std::result::Result<T, E>;

impl<E> From<E> for LensError
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn from(value: E) -> Self {
        Self(value.into())
    }
}

impl serde::Serialize for LensError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.0.to_string().as_ref())
    }
}

impl Lens {
    pub fn new() -> Self {
        let ctx = SessionContext::new();
        Self { ctx }
    }

    pub async fn sql(&self, query: &str) -> LensResult<DataFrame> {
        let df = self.ctx.sql(query).await?;
        Ok(df)
    }

    pub fn context(&self) -> &SessionContext {
        &self.ctx
    }
}

#[tauri::command]
async fn sql(lens: tauri::State<'_, Lens>, query: String) -> LensResult<()> {
    lens.sql(&query).await?;
    Ok(())
}

#[tauri::command]
async fn list_databases(lens: tauri::State<'_, Lens>) -> LensResult<Vec<model::Database>> {
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
                println!("{table_name}");
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

fn main() {
    tauri::Builder::default()
        .manage(Lens::new())
        .invoke_handler(tauri::generate_handler![list_databases, sql])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
