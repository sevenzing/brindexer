use super::types::TokenData;
use crate::indexers::runtime::IndexerJobError;
use blockscout_db::entity::tokens;
use sea_orm::{ActiveModelTrait, ConnectionTrait, IntoActiveModel, Set};

pub async fn update_tokens_with_data_in_db<C: ConnectionTrait>(
    db: &C,
    tokens: Vec<(tokens::Model, TokenData)>,
    latest_block: i64,
) -> Result<(), IndexerJobError> {
    for (token, data) in tokens {
        let mut active = token.into_active_model();
        active.cataloged = Set(Some(!data.is_empty()));
        active.skip_metadata = Set(Some(data.is_empty()));
        active.name = Set(data.name);
        active.symbol = Set(data.symbol);
        active.decimals = Set(data.decimals);
        if let Some(total_supply) = data.total_supply {
            active.total_supply = Set(Some(total_supply));
            active.total_supply_updated_at_block = Set(Some(latest_block));
        }
        active.update(db).await?;
    }
    Ok(())
}
