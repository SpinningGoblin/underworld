use redis::{aio::Connection, AsyncCommands, RedisError};
use underworld_core::components::player::PlayerCharacter;

use super::utils::{username_key, username_player_character_key};

pub async fn get_player_character(
    connection: &mut Connection,
    username: &str,
    id: &str,
) -> Option<PlayerCharacter> {
    let key = username_player_character_key(username, id);
    let serialized: Result<String, RedisError> = connection.get(&key).await;

    match serialized {
        Ok(it) => {
            let player: PlayerCharacter = serde_json::from_str(&it).unwrap();
            Some(player)
        }
        Err(_) => None,
    }
}

pub async fn player_character_ids(connection: &mut Connection, username: &str) -> Vec<String> {
    let key_start = username_key(username);
    let redis_keys: Result<Vec<String>, RedisError> =
        connection.keys(format!("{}*", &key_start)).await;

    match redis_keys {
        Ok(keys) => keys
            .iter()
            .map(|key| key.replace(&format!("{}:", &key_start), ""))
            .collect(),
        Err(it) => {
            println!("{:?}", &it);
            Vec::new()
        }
    }
}
