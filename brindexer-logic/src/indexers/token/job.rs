use crate::{
    indexers::runtime::{IndexerJob, IndexerJobContext, IndexerJobError},
    repository,
};
use derive_new::new;

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
        "every 10 seconds"
    }

    async fn execute(&self, ctx: &IndexerJobContext) -> Result<(), IndexerJobError> {
        let tokens =
            repository::tokens::fetch_uncataloged_tokens(ctx.db.as_ref(), self.batch_size).await?;
        let tokens_with_actual_data =
            super::rpc::get_all_token_data_from_rpc(ctx.rpc.as_ref(), tokens).await?;
        super::db::update_tokens_with_data_in_db(ctx.db.as_ref(), tokens_with_actual_data).await?;
        Ok(())
    }
}
