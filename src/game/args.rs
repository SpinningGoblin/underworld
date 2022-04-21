use poem_openapi::Object;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Object, Serialize)]
pub struct RoomLookArgs {
    pub username: String,
    pub game_id: String,
}

#[derive(Deserialize, Object, Serialize)]
pub struct ExitRoomArgs {
    pub username: String,
    pub game_id: String,
    pub exit_id: String,
}
