use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};
use underworld_core::{
    actions::{action::Action, exit_room::ExitRoom},
    game::Game,
};

use crate::{
    actions::{game_actions, PerformAction},
    error::GameError,
    event::GameEvent,
};

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

pub async fn exit_room(
    transaction: &mut Transaction<'_, Postgres>,
    args: &ExitRoomArgs,
) -> Result<RoomExited, GameError> {
    let player_character =
        match crate::player_characters::repository::current(transaction, &args.username)
            .await
            .unwrap()
        {
            Some(it) => it,
            None => return Err(GameError::NoPlayerCharacterSet),
        };

    let state = match super::repository::by_id(transaction, &args.username, &args.game_id)
        .await
        .unwrap()
    {
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
    super::repository::save(transaction, &args.username, &game.state)
        .await
        .unwrap();
    let game_events: Vec<GameEvent> = events.into_iter().map(GameEvent::from).collect();

    Ok(RoomExited {
        events: game_events,
        actions: game_actions(&game, &args.username),
    })
}
