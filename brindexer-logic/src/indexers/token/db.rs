use crate::indexers::runtime::IndexerJobError;
use blockscout_db::entity::tokens;
use sea_orm::{ActiveModelTrait, DatabaseConnection, IntoActiveModel, Set, TransactionTrait};

use super::types::TokenData;

pub async fn update_tokens_with_data_in_db(
    db: &DatabaseConnection,
    tokens: Vec<(tokens::Model, TokenData)>,
) -> Result<(), IndexerJobError> {
    let tx = db.begin().await?;
    for (token, data) in tokens {
        let mut active = token.into_active_model();
        active.cataloged = Set(Some(!data.is_empty()));
        active.name = Set(data.name);
        active.symbol = Set(data.symbol);
        active.decimals = Set(data.decimals);
        active.total_supply = Set(data.total_supply);
        active.skip_metadata = Set(Some(data.skip_metadata));
        active.update(&tx).await?;
    }
    tx.commit().await?;
    Ok(())
}
