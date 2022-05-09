use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};
use underworld_core::components::player::PlayerCharacter;

use crate::error::GameError;

pub async fn get_current_player_character(
    transaction: &mut Transaction<'_, Postgres>,
    username: &str,
) -> Result<PlayerCharacter, GameError> {
    let player_character = super::repository::current(transaction, username)
        .await
        .unwrap();

    match player_character {
        Some(it) => Ok(it),
        None => Err(GameError::NoPlayerCharacterSet),
    }
}

#[derive(Deserialize, Object, Serialize)]
pub struct SetPlayerCharacterArgs {
    pub username: String,
    pub player_character_id: String,
}

pub async fn set_current_player_character(
    transaction: &mut Transaction<'_, Postgres>,
    args: &SetPlayerCharacterArgs,
) -> Result<(), GameError> {
    let player_result =
        super::repository::by_id(transaction, &args.username, &args.player_character_id)
            .await
            .unwrap();

    match player_result {
        Some(it) => {
            super::repository::set_current(transaction, &args.username, &it)
                .await
                .unwrap();
            Ok(())
        }
        None => Err(GameError::UnknownPlayerCharacter),
    }
}
