#![allow(clippy::blocks_in_conditions)]
use crate::{
    indexers::runtime::{IndexerJob, IndexerJobContext, IndexerJobError},
    repository,
};
use derive_new::new;
use tracing::instrument;

#[derive(Debug, Clone, new)]
pub struct TokenDataJob {
    batch_size: u64,
}

#[async_trait::async_trait]
impl IndexerJob for TokenDataJob {
    fn name(&self) -> &'static str {
        "token_data_job"
    }

    fn schedule(&self) -> &'static str {
        "every 5 seconds"
    }

    #[instrument(
        name = "token_data_job",
        level = "DEBUG",
        skip_all,
        fields(batch_size = self.batch_size),
        err,
    )]
    async fn execute(&self, ctx: &IndexerJobContext) -> Result<(), IndexerJobError> {
        let latest_block = repository::blocks::get_latest_block(ctx.db.as_ref()).await?;
        let tokens =
            repository::tokens::fetch_uncataloged_tokens(ctx.db.as_ref(), self.batch_size).await?;
        if tokens.is_empty() {
            tracing::debug!("no uncataloged tokens found");
            return Ok(());
        } else {
            tracing::info!(len = tokens.len(), "found uncataloged tokens");
        }
        let tokens_with_actual_data =
            super::rpc::get_all_token_data_from_rpc(ctx.rpc.as_ref(), tokens).await?;
        super::db::update_tokens_with_data_in_db(
            ctx.db.as_ref(),
            tokens_with_actual_data,
            latest_block,
        )
        .await?;
        Ok(())
    }
}
