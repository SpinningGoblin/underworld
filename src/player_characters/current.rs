use sqlx::{Postgres, Transaction};
use underworld_core::components::player::PlayerCharacter;

use crate::error::GameError;

pub async fn get_current_player_character(
    transaction: &mut Transaction<'_, Postgres>,
    username: &str,
) -> Result<PlayerCharacter, GameError> {
    let player_character = super::repository::current(transaction, username).await?;

    match player_character {
        Some(it) => Ok(it),
        None => Err(GameError::NoPlayerCharacterSetError),
    }
}

pub async fn set_current_player_character(
    transaction: &mut Transaction<'_, Postgres>,
    username: &str,
    pc_id: &str,
) -> Result<(), GameError> {
    let player_result = super::repository::by_id(transaction, &username, &pc_id).await?;

    match player_result {
        Some(it) => {
            super::repository::set_current(transaction, &username, &it).await?;
            Ok(())
        }
        None => Err(GameError::UnknownPlayerCharacterError),
    }
}
