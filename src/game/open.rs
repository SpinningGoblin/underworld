use poem_openapi::Object;
use serde::Serialize;
use sqlx::{Transaction, Postgres};
use underworld_core::{actions::{OpenFixture, Action, OpenFixtureHiddenCompartment}, game::Game};

use crate::{event::GameEvent, actions::{PerformAction, game_actions}, error::GameError};

#[derive(Serialize, Object)]
pub struct FixtureOpened {
    pub events: Vec<GameEvent>,
    pub actions: Vec<PerformAction>,
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

    Ok(FixtureOpened {
        events: game_events,
        actions: game_actions(&game, username),
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

    Ok(FixtureOpened {
        events: game_events,
        actions: game_actions(&game, username),
    })
}
