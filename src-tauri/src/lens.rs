use datafusion::prelude::*;
use tokio::sync::mpsc;

use crate::{
    model::{self, StreamId},
    stream::{QueryStreamRequest, QueryStreamer},
};

pub struct LensError(pub(crate) Box<dyn std::error::Error + Send + Sync + 'static>);
pub type LensResult<T, E = LensError> = std::result::Result<T, E>;

pub struct Lens {
    ctx: SessionContext,

    stream_tx: mpsc::Sender<QueryStreamRequest>,
}

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
    pub fn new() -> (Self, QueryStreamer) {
        let ctx = SessionContext::new();
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
        let df = self.ctx.sql(query).await?;
        Ok(df)
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
}
