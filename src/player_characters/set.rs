use redis::{aio::Connection, AsyncCommands};
use underworld_core::components::player::PlayerCharacter;

use super::utils::username_player_character_key;

pub async fn set_player_character(
    connection: &mut Connection,
    player: &PlayerCharacter,
    username: &str,
) {
    let player_id = player.identifier.id.to_string();
    let redis_key = username_player_character_key(username, &player_id);
    let serialized = serde_json::to_string(&player).unwrap();
    let _: () = connection.set(&redis_key, serialized).await.unwrap();
}
