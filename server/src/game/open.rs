use poem_openapi::Object;
use serde::Serialize;
use sqlx::{Postgres, Transaction};
use underworld_core::{
    actions::{Action, OpenFixture, OpenFixtureHiddenCompartment},
    components::{rooms::RoomView, PlayerCharacterView},
    Game,
};

use crate::{
    actions::{game_actions, PerformAction},
    error::GameError,
    event::GameEvent,
};

#[derive(Serialize, Object)]
pub struct FixtureOpened {
    pub events: Vec<GameEvent>,
    pub actions: Vec<PerformAction>,
    pub current_room: RoomView,
    pub current_player: PlayerCharacterView,
}

pub async fn open_fixture(
    transaction: &mut Transaction<'_, Postgres>,
    username: &str,
    game_id: &str,
    args: &OpenFixture,
) -> Result<FixtureOpened, GameError> {
    let player = match crate::player_characters::repository::current(transaction, username).await? {
        Some(it) => it,
        None => return Err(GameError::NoPlayerCharacterSetError),
    };

    let state = match super::repository::by_id(transaction, username, game_id).await? {
        Some(it) => it,
        None => return Err(GameError::GameNotFoundError),
    };

    let mut game = Game { state, player };
    let action = Action::OpenFixture(args.to_owned());
    let events = game.handle_action(&action)?;

    super::repository::save(transaction, username, &game.state).await?;
    crate::player_characters::repository::save(transaction, username, &game.player).await?;

    let game_events: Vec<GameEvent> = events.into_iter().map(GameEvent::from).collect();

    let current_room = game.state.view_current_room();
    let current_player = underworld_core::systems::view::player::check(&game.player);

    Ok(FixtureOpened {
        events: game_events,
        actions: game_actions(&game, username),
        current_player,
        current_room,
    })
}

pub async fn open_fixture_hidden_compartment(
    transaction: &mut Transaction<'_, Postgres>,
    username: &str,
    game_id: &str,
    args: &OpenFixtureHiddenCompartment,
) -> Result<FixtureOpened, GameError> {
    let player = match crate::player_characters::repository::current(transaction, username).await? {
        Some(it) => it,
        None => return Err(GameError::NoPlayerCharacterSetError),
    };

    let state = match super::repository::by_id(transaction, username, game_id).await? {
        Some(it) => it,
        None => return Err(GameError::GameNotFoundError),
    };

    let mut game = Game { state, player };
    let action = Action::OpenFixtureHiddenCompartment(args.to_owned());
    let events = game.handle_action(&action)?;

    super::repository::save(transaction, username, &game.state).await?;
    crate::player_characters::repository::save(transaction, username, &game.player).await?;

    let game_events: Vec<GameEvent> = events.into_iter().map(GameEvent::from).collect();

    let current_room = game.state.view_current_room();
    let current_player = underworld_core::systems::view::player::check(&game.player);

    Ok(FixtureOpened {
        events: game_events,
        actions: game_actions(&game, username),
        current_room,
        current_player,
    })
}
