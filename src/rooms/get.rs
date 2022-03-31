use redis::{aio::Connection, AsyncCommands, RedisError};
use underworld_core::components::rooms::room::Room;

pub async fn load_room(connection: &mut Connection, id: &str) -> Option<Room> {
    let serialized: Result<String, RedisError> = connection.get(format!("room:{}", &id)).await;
    match serialized {
        Ok(it) => {
            let room: Room = serde_json::from_str(&it).unwrap();
            Some(room)
        }
        Err(_) => None,
    }
}
