use redis::{aio::Connection, AsyncCommands, RedisError};
use underworld_core::components::games::game_state::GameState;

use super::utils::username_game_key;

pub async fn get_game_state(
    connection: &mut Connection,
    username: &str,
    id: &str,
) -> Option<GameState> {
    let key = username_game_key(username, id);
    let serialized: Result<String, RedisError> = connection.get(&key).await;

    match serialized {
        Ok(it) => {
            let game_state: GameState = serde_json::from_str(&it).unwrap();
            Some(game_state)
        }
        Err(_) => None,
    }
}
