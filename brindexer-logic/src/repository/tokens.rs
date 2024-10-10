use blockscout_db::entity::tokens::Model as Token;
use sea_orm::{prelude::*, Condition, QuerySelect};

pub async fn fetch_uncataloged_tokens<C: ConnectionTrait>(
    db: &C,
    limit: u64,
) -> Result<Vec<Token>, DbErr> {
    let tokens = blockscout_db::entity::tokens::Entity::find()
        .filter(
            Condition::any()
                .add(blockscout_db::entity::tokens::Column::Cataloged.eq(false))
                .add(blockscout_db::entity::tokens::Column::Cataloged.is_null()),
        )
        .filter(
            Condition::any()
                .add(blockscout_db::entity::tokens::Column::SkipMetadata.eq(false))
                .add(blockscout_db::entity::tokens::Column::SkipMetadata.is_null()),
        )
        .limit(limit)
        .all(db)
        .await?;
    Ok(tokens)
}
