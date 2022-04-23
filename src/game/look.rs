use poem_openapi::Object;
use redis::aio::Connection;
use serde::{Deserialize, Serialize};
use underworld_core::{
    components::rooms::room_view::{RoomView, RoomViewArgs},
    systems::view::room,
};

use crate::error::GameError;

use super::get::get_game_state;

#[derive(Deserialize, Object, Serialize)]
pub struct RoomLookArgs {
    pub username: String,
    pub game_id: String,
}

pub async fn look_at_room(
    connection: &mut Connection,
    args: &RoomLookArgs,
) -> Result<RoomView, GameError> {
    let game_state = match get_game_state(connection, &args.username, &args.game_id).await {
        Some(it) => it,
        None => return Err(GameError::GameNotFound),
    };

    let args = RoomViewArgs {
        can_see_hidden: false,
        can_see_packed: false,
        knows_character_health: false,
        knows_names: true,
    };
    Ok(room::look_at(game_state.current_room(), args, false))
}

pub async fn quick_look_room(
    connection: &mut Connection,
    args: &RoomLookArgs,
) -> Result<RoomView, GameError> {
    let game_state = match get_game_state(connection, &args.username, &args.game_id).await {
        Some(it) => it,
        None => return Err(GameError::GameNotFound),
    };

    Ok(room::quick_look(game_state.current_room()))
}
