use blockscout_db::entity::tokens::Model as Token;
use sea_orm::{prelude::*, QuerySelect};

pub async fn fetch_uncataloged_tokens<C: ConnectionTrait>(
    db: &C,
    limit: u64,
) -> Result<Vec<Token>, DbErr> {
    let tokens = blockscout_db::entity::tokens::Entity::find()
        .filter(
            blockscout_db::entity::tokens::Column::Cataloged
                .eq(true)
                .not(),
        )
        .filter(
            blockscout_db::entity::tokens::Column::SkipMetadata
                .eq(true)
                .not(),
        )
        .limit(limit)
        .all(db)
        .await?;
    Ok(tokens)
}
