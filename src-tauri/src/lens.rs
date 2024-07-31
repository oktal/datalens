use datafusion::{
    logical_expr::{DdlStatement, LogicalPlan},
    prelude::*,
    sql::{parser::Statement, TableReference},
};
use tokio::sync::mpsc;

use crate::{
    model::{self, StreamId},
    stream::{QueryStreamRequest, QueryStreamer},
};

pub struct LensError(anyhow::Error);
pub type LensResult<T, E = LensError> = std::result::Result<T, E>;

pub struct Lens {
    ctx: SessionContext,

    stream_tx: mpsc::Sender<QueryStreamRequest>,
}

impl<E> From<E> for LensError
where
    E: Into<anyhow::Error>,
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
    pub fn new() -> (Self, QueryStreamer) {
        let config = SessionConfig::new()
            .with_information_schema(false)
            .with_create_default_catalog_and_schema(false);
        let ctx = SessionContext::new_with_config(config);
        let (query_exec, query_tx) = QueryStreamer::new(ctx.clone());
        (
            Self {
                ctx,
                stream_tx: query_tx,
            },
            query_exec,
        )
    }

    pub async fn sql(&self, query: &str) -> LensResult<DataFrame> {
        Ok(self
            .ctx
            .execute_logical_plan(self.create_logical_plan(query).await?)
            .await?)
    }

    pub async fn stream(&self, query: &str) -> LensResult<StreamId> {
        let (req, rx) = QueryStreamRequest::create(query);
        self.stream_tx.send(req).await?;
        let stream_id = rx.await?;
        stream_id
    }

    pub async fn stream_next(&self, stream_id: StreamId) -> LensResult<Option<Vec<model::Row>>> {
        let (req, rx) = QueryStreamRequest::next(stream_id);
        self.stream_tx.send(req).await?;
        let rows = rx.await?;
        rows
    }

    pub fn context(&self) -> &SessionContext {
        &self.ctx
    }

    async fn create_logical_plan(&self, query: &str) -> LensResult<LogicalPlan> {
        // We need to create (and rewrite) our own logical plan instead of directly using `sql`
        // from DataFusion `SessionContext` because unfortunately, DataFusion does not properly
        // interpret table identifier from the "CREATE EXTERNAL TABLE <database>.<schema>.<table>"
        // statement.
        // When transforming a SQL statement to its LogicalPlan, DataFusion will convert
        // "<database>.<schema>.<table>" identifier to a TableReference::Bare { "database.schema.table" }
        // reference instead of TableReference::Full { "database", "schema", "table" }
        // We thus "rewrite" the logical plan prior to executing it with the TableReference that we
        // parsed from the initial statement
        let state = self.ctx.state();
        let dialect = state.config().options().sql_parser.dialect.as_str();

        let statement = state.sql_to_statement(query, dialect)?;

        let create_table_ref = if let Statement::CreateExternalTable(cet) = &statement {
            Some(TableReference::parse_str(&cet.name))
        } else {
            None
        };

        let plan = state.statement_to_plan(statement).await?;
        let plan = if let LogicalPlan::Ddl(DdlStatement::CreateExternalTable(mut cet)) = plan {
            if let Some(table_ref) = create_table_ref {
                cet.name = table_ref;
            }
            LogicalPlan::Ddl(DdlStatement::CreateExternalTable(cet))
        } else {
            plan
        };

        Ok(plan)
    }
}
