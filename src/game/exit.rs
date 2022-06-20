use poem_openapi::Object;
use serde::Serialize;
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
/// Result of exiting the room.
pub struct RoomExited {
    events: Vec<GameEvent>,
    actions: Vec<PerformAction>,
}

pub async fn exit_room(
    transaction: &mut Transaction<'_, Postgres>,
    username: &str,
    game_id: &str,
    args: &ExitRoom,
) -> Result<RoomExited, GameError> {
    let player_character =
        match crate::player_characters::repository::current(transaction, username).await? {
            Some(it) => it,
            None => return Err(GameError::NoPlayerCharacterSetError),
        };

    let state = match super::repository::by_id(transaction, username, game_id).await? {
        Some(it) => it,
        None => return Err(GameError::GameNotFoundError),
    };

    let mut game = Game {
        player: player_character,
        state,
    };

    let events = game.handle_action(&Action::ExitRoom(args.to_owned()))?;
    super::repository::save(transaction, username, &game.state).await?;
    let game_events: Vec<GameEvent> = events.into_iter().map(GameEvent::from).collect();

    Ok(RoomExited {
        events: game_events,
        actions: game_actions(&game, username),
    })
}
