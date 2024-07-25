use serde::{Deserialize, Serialize};
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

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct StreamId(uuid::Uuid);

impl std::fmt::Display for StreamId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl StreamId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

/// Represent a row read from a record batch
#[derive(Debug, Serialize)]
pub struct Row {
    /// Names of the columns present in this row
    pub columns: Vec<String>,

    /// Values of the corresponding columns
    pub values: Vec<String>,
}
