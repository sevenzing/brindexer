use crate::IndexerJobError;
use blockscout_db::entity::blocks;
use sea_orm::{ConnectionTrait, EntityTrait, QueryOrder};

pub async fn get_latest_block<C: ConnectionTrait>(db: &C) -> Result<i64, IndexerJobError> {
    let block = blocks::Entity::find()
        .order_by_desc(blocks::Column::Number)
        .one(db)
        .await?;
    Ok(block.map(|b| b.number).unwrap_or(0))
}
