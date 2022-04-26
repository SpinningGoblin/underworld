use poem_openapi::Object;
use redis::aio::Connection;
use serde::{Deserialize, Serialize};
use underworld_core::{
    actions::{action::Action, exit_room::ExitRoom},
    game::Game,
};

use crate::{
    actions::{room_actions, PerformAction},
    error::GameError,
    event::GameEvent,
    player_characters::current::get_current_player_character,
};

use super::{get::get_game_state, set::set_game};

#[derive(Serialize, Object)]
pub struct RoomExited {
    events: Vec<GameEvent>,
    actions: Vec<PerformAction>,
}

#[derive(Deserialize, Object, Serialize)]
pub struct ExitRoomArgs {
    pub username: String,
    pub game_id: String,
    pub exit_id: String,
}

pub async fn exit_current_room(
    connection: &mut Connection,
    args: &ExitRoomArgs,
) -> Result<RoomExited, GameError> {
    let player_character = match get_current_player_character(connection, &args.username).await {
        Ok(it) => it,
        Err(it) => return Err(it),
    };

    let state = match get_game_state(connection, &args.username, &args.game_id).await {
        Some(it) => it,
        None => return Err(GameError::GameNotFound),
    };

    let mut game = Game {
        player: player_character,
        state,
    };

    let exit_room = ExitRoom {
        exit_id: args.exit_id.clone(),
    };

    let events = game.handle_action(&Action::ExitRoom(exit_room)).unwrap();
    set_game(connection, &game.state, &args.username).await;

    let game_events: Vec<GameEvent> = events.into_iter().map(GameEvent::from).collect();

    Ok(RoomExited {
        events: game_events,
        actions: room_actions(game.state.current_room(), &args.username, &args.game_id),
    })
}
