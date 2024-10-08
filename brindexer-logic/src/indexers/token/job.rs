use derive_new::new;

use crate::indexers::runtime::{IndexerJob, IndexerJobContext, IndexerJobError};

#[derive(Debug, Clone, new)]
pub struct TokenDataJob;

#[async_trait::async_trait]
impl IndexerJob for TokenDataJob {
    fn name(&self) -> &'static str {
        "token_data_job"
    }

    fn schedule(&self) -> &'static str {
        "every 10 seconds"
    }

    async fn execute(&self, ctx: &IndexerJobContext) -> Result<(), IndexerJobError> {
        tracing::info!("run token data job with retries: {}", ctx.retries);
        Ok(())
    }
}
