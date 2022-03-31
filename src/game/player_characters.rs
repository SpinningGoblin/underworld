use poem_openapi::Object;
use redis::{aio::Connection, AsyncCommands, RedisError};
use serde::{Deserialize, Serialize};
use underworld_core::components::player::PlayerCharacter;

use crate::player_characters::get::get_player_character;

use super::{error::GameError, utils::current_player_character_key};

pub async fn get_current_player_character(
    connection: &mut Connection,
    username: &str,
) -> Result<PlayerCharacter, GameError> {
    let key = current_player_character_key(&username);
    let player_id_result: Result<String, RedisError> = connection.get(&key).await;

    match player_id_result {
        Ok(player_id) => get_player_character(connection, &username, &player_id)
            .await
            .map(|player_character| Ok(player_character))
            .unwrap_or(Err(GameError::General)),
        Err(_) => Err(GameError::NoPlayerCharacterSet),
    }
}

#[derive(Deserialize, Object, Serialize)]
pub struct SetPlayerCharacterArgs {
    pub username: String,
    pub player_character_id: String,
}

pub async fn set_current_player_character(
    connection: &mut Connection,
    args: &SetPlayerCharacterArgs,
) -> Result<(), GameError> {
    let player_result =
        get_player_character(connection, &args.username, &args.player_character_id).await;

    match player_result {
        Some(_) => {
            let key = current_player_character_key(&args.username);
            let _: () = connection
                .set(&key, &args.player_character_id)
                .await
                .unwrap();
            Ok(())
        }
        None => Err(GameError::UnknownPlayerCharacter),
    }
}
