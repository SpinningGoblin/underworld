use poem_openapi::Object;
use serde::Serialize;
use sqlx::{Postgres, Transaction};
use underworld_core::{
    actions::{Action, LootFixture, LootNpc},
    Game,
};

use crate::{
    actions::{game_actions, PerformAction},
    error::GameError,
    event::GameEvent,
};

#[derive(Serialize, Object)]
pub struct NpcLooted {
    pub events: Vec<GameEvent>,
    pub actions: Vec<PerformAction>,
}

pub async fn loot_npc(
    transaction: &mut Transaction<'_, Postgres>,
    username: &str,
    game_id: &str,
    args: &LootNpc,
) -> Result<NpcLooted, GameError> {
    let player = match crate::player_characters::repository::current(transaction, username).await? {
        Some(it) => it,
        None => return Err(GameError::NoPlayerCharacterSetError),
    };

    let state = match super::repository::by_id(transaction, username, game_id).await? {
        Some(it) => it,
        None => return Err(GameError::GameNotFoundError),
    };

    let mut game = Game { state, player };
    let action = Action::LootNpc(args.to_owned());
    let events = game.handle_action(&action)?;

    super::repository::save(transaction, username, &game.state).await?;
    crate::player_characters::repository::save(transaction, username, &game.player).await?;

    let game_events: Vec<GameEvent> = events.into_iter().map(GameEvent::from).collect();

    Ok(NpcLooted {
        events: game_events,
        actions: game_actions(&game, username),
    })
}

#[derive(Serialize, Object)]
pub struct FixtureLooted {
    pub events: Vec<GameEvent>,
    pub actions: Vec<PerformAction>,
}

pub async fn loot_fixture(
    transaction: &mut Transaction<'_, Postgres>,
    username: &str,
    game_id: &str,
    args: &LootFixture,
) -> Result<FixtureLooted, GameError> {
    let player = match crate::player_characters::repository::current(transaction, username).await? {
        Some(it) => it,
        None => return Err(GameError::NoPlayerCharacterSetError),
    };

    let state = match super::repository::by_id(transaction, username, game_id).await? {
        Some(it) => it,
        None => return Err(GameError::GameNotFoundError),
    };

    let mut game = Game { state, player };
    let action = Action::LootFixture(args.to_owned());
    let events = game.handle_action(&action)?;

    super::repository::save(transaction, username, &game.state).await?;
    crate::player_characters::repository::save(transaction, username, &game.player).await?;

    let game_events: Vec<GameEvent> = events.into_iter().map(GameEvent::from).collect();

    Ok(FixtureLooted {
        events: game_events,
        actions: game_actions(&game, username),
    })
}
