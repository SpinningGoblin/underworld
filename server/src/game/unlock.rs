use sqlx::{Postgres, Transaction};

use crate::error::GameError;

pub async fn unlock_knowledge(
    transaction: &mut Transaction<'_, Postgres>,
    username: &str,
    game_id: &str,
) -> Result<(), GameError> {
    let mut state = match super::repository::by_id(transaction, username, game_id).await? {
        Some(game_state) => game_state,
        None => return Err(GameError::GameNotFoundError),
    };

    state.all_knowledge_unlocked = true;

    super::repository::save(transaction, username, &state).await?;
    Ok(())
}
