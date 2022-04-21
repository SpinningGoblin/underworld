use redis::{aio::Connection, AsyncCommands};
use underworld_core::components::games::game_state::GameState;

use super::utils::username_game_key;

pub async fn set_game(connection: &mut Connection, game_state: &GameState, username: &str) {
    let game_id = game_state.identifier.id.to_string();
    let redis_key = username_game_key(username, &game_id);
    let serialized = serde_json::to_string(&game_state).unwrap();
    let _: () = connection.set(&redis_key, serialized).await.unwrap();
}
