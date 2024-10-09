use sea_orm::prelude::*;
use blockscout_db::entity::tokens::Model as Token;

pub async fn fetch_uncataloged_tokens<C: ConnectionTrait>(
    db: &C,
) -> Result<Vec<Token>, DbErr> {
    let tokens = blockscout_db::entity::tokens::Entity::find()
        .filter(
            blockscout_db::entity::tokens::Column::Cataloged.eq(true).not()
        )
        .all(db)
        .await?;
    Ok(tokens)
}

