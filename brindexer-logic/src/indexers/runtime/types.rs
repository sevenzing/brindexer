use derive_new::new;
use sea_orm::DatabaseConnection;
use std::{sync::Arc, time::Duration};
use thiserror::Error;

use crate::{rpc::RpcClient, RpcClientError};

#[async_trait::async_trait]
pub trait IndexerJob: Send + Sync {
    fn retries(&self) -> usize {
        3
    }
    fn retry_interval(&self) -> Duration {
        Duration::from_secs(5)
    }
    fn name(&self) -> String;
    fn schedule(&self) -> String {
        "every 10 seconds".to_string()
    }
    async fn execute(&self, ctx: &IndexerJobContext) -> Result<(), IndexerJobError>;
}

#[derive(Debug, new)]
pub struct IndexerJobContext {
    pub db: Arc<DatabaseConnection>,
    pub rpc: Arc<RpcClient>,
    pub retries: usize,
}

impl IndexerJobContext {
    pub fn from_db_rpc(db: Arc<DatabaseConnection>, rpc: Arc<RpcClient>) -> Self {
        Self::new(db, rpc, 0)
    }

    pub fn with_retries(&mut self, retries: usize) -> &mut Self {
        self.retries = retries;
        self
    }
}

#[derive(Error, Debug)]
pub enum IndexerJobError {
    #[error("rpc error: {0}")]
    RpcError(#[from] RpcClientError),
    #[error("db error: {0}")]
    DatabaseError(#[from] sea_orm::DbErr),
    #[error("internal error: {0}")]
    Internal(#[from] anyhow::Error),
}
