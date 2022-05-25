use std::error::Error;

use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};
use underworld_core::{
    actions::{action::Action, loot_npc::LootNpc, LootFixture},
    game::Game,
};

use crate::{
    actions::{game_actions, PerformAction},
    error::{GameNotFoundError, NoPlayerCharacterSetError},
    event::GameEvent,
};

#[derive(Deserialize, Object, Serialize)]
pub struct LootNpcArgs {
    pub username: String,
    pub game_id: String,
    pub npc_id: String,
    pub item_ids: Vec<String>,
}

#[derive(Serialize, Object)]
pub struct NpcLooted {
    pub events: Vec<GameEvent>,
    pub actions: Vec<PerformAction>,
}

pub async fn loot_npc(
    transaction: &mut Transaction<'_, Postgres>,
    args: &LootNpcArgs,
) -> Result<NpcLooted, Box<dyn Error>> {
    let player =
        match crate::player_characters::repository::current(transaction, &args.username).await? {
            Some(it) => it,
            None => return Err(Box::new(NoPlayerCharacterSetError)),
        };

    let state = match super::repository::by_id(transaction, &args.username, &args.game_id).await? {
        Some(it) => it,
        None => return Err(Box::new(GameNotFoundError)),
    };

    let mut game = Game { state, player };

    let loot_npc = LootNpc {
        npc_id: args.npc_id.clone(),
        item_ids: args.item_ids.clone(),
    };
    let action = Action::LootNpc(loot_npc);
    let events = game.handle_action(&action)?;

    super::repository::save(transaction, &args.username, &game.state).await?;
    crate::player_characters::repository::save(transaction, &args.username, &game.player).await?;

    let game_events: Vec<GameEvent> = events.into_iter().map(GameEvent::from).collect();

    Ok(NpcLooted {
        events: game_events,
        actions: game_actions(&game, &args.username),
    })
}

#[derive(Deserialize, Object, Serialize)]
pub struct LootFixtureArgs {
    pub username: String,
    pub game_id: String,
    pub fixture_id: String,
    pub item_ids: Vec<String>,
}

#[derive(Serialize, Object)]
pub struct FixtureLooted {
    pub events: Vec<GameEvent>,
    pub actions: Vec<PerformAction>,
}

pub async fn loot_fixture(
    transaction: &mut Transaction<'_, Postgres>,
    args: &LootFixtureArgs,
) -> Result<FixtureLooted, Box<dyn Error>> {
    let player =
        match crate::player_characters::repository::current(transaction, &args.username).await? {
            Some(it) => it,
            None => return Err(Box::new(NoPlayerCharacterSetError)),
        };

    let state = match super::repository::by_id(transaction, &args.username, &args.game_id).await? {
        Some(it) => it,
        None => return Err(Box::new(GameNotFoundError)),
    };

    let mut game = Game { state, player };

    let loot_fixture = LootFixture {
        fixture_id: args.fixture_id.clone(),
        item_ids: args.item_ids.clone(),
    };
    let action = Action::LootFixture(loot_fixture);
    let events = game.handle_action(&action)?;

    super::repository::save(transaction, &args.username, &game.state).await?;
    crate::player_characters::repository::save(transaction, &args.username, &game.player).await?;

    let game_events: Vec<GameEvent> = events.into_iter().map(GameEvent::from).collect();

    Ok(FixtureLooted {
        events: game_events,
        actions: game_actions(&game, &args.username),
    })
}
