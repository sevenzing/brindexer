use derive_new::new;
use sea_orm::DatabaseConnection;
use std::{sync::Arc, time::Duration};
use thiserror::Error;

#[async_trait::async_trait]
pub trait IndexerJob: Send + Sync {
    fn retries(&self) -> usize {
        3
    }
    fn retry_interval(&self) -> Duration {
        Duration::from_secs(5)
    }
    fn name(&self) -> &'static str;
    fn schedule(&self) -> &'static str {
        "every 10 seconds"
    }
    async fn execute(&self, ctx: &IndexerJobContext) -> Result<(), IndexerJobError>;
}

#[derive(Debug, new)]
pub struct IndexerJobContext {
    pub db: Arc<DatabaseConnection>,
    pub retries: usize,
}

impl IndexerJobContext {
    pub fn from_db(db: Arc<DatabaseConnection>) -> Self {
        Self::new(db, 0)
    }

    pub fn with_retries(&mut self, retries: usize) -> &mut Self {
        self.retries = retries;
        self
    }
}

#[derive(Error, Debug)]
pub enum IndexerJobError {
    #[error("db error: {0}")]
    DatabaseError(#[from] sea_orm::DbErr),
    #[error("internal error: {0}")]
    Internal(#[from] anyhow::Error),
}
