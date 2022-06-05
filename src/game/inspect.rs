use std::error::Error;

use poem_openapi::Object;
use serde::Serialize;
use sqlx::{Postgres, Transaction};
use underworld_core::{
    actions::{action::Action, inspect_fixture::InspectFixture, inspect_npc::InspectNpc},
    events::event::Event,
    game::Game,
};

use crate::{
    actions::{game_actions, PerformAction},
    error::{GameNotFoundError, NoPlayerCharacterSetError}, event::GameEvent,
};

#[derive(Object, Serialize)]
pub struct NpcInspected {
    pub health_discovered: bool,
    pub packed_items_discovered: bool,
    pub hidden_items_discovered: bool,
    pub actions: Vec<PerformAction>,
    pub events: Vec<GameEvent>,
}

pub async fn inspect_npc(
    transaction: &mut Transaction<'_, Postgres>,
    username: &str,
    game_id: &str,
    args: &InspectNpc,
) -> Result<NpcInspected, Box<dyn Error>> {
    let state = match super::repository::by_id(transaction, &username, &game_id).await? {
        Some(it) => it,
        None => return Err(Box::new(GameNotFoundError)),
    };
    let player = match crate::player_characters::repository::current(transaction, &username).await?
    {
        Some(it) => it,
        None => return Err(Box::new(NoPlayerCharacterSetError)),
    };
    let mut game = Game { state, player };

    let action = Action::InspectNpc(args.to_owned());
    let events = game.handle_action(&action)?;

    super::repository::save(transaction, &username, &game.state)
        .await
        .unwrap();
    crate::player_characters::repository::save(transaction, &username, &game.player)
        .await
        .unwrap();

    let game_events: Vec<GameEvent> = events.iter().cloned().map(GameEvent::from).collect();

    let mut npc_inspected = NpcInspected {
        health_discovered: false,
        packed_items_discovered: false,
        hidden_items_discovered: false,
        actions: game_actions(&game, &username),
        events: game_events,
    };

    for event in events {
        match event {
            Event::NpcHealthDiscovered(_) => {
                npc_inspected.health_discovered = true;
            }
            Event::NpcHiddenDiscovered(_) => {
                npc_inspected.hidden_items_discovered = true;
            }
            Event::NpcPackedDiscovered(_) => {
                npc_inspected.packed_items_discovered = true;
            }
            _ => {}
        }
    }

    Ok(npc_inspected)
}

#[derive(Object, Serialize)]
pub struct FixtureInspected {
    pub can_be_opened_discovered: bool,
    pub has_hidden_discovered: bool,
    pub hidden_items_discovered: bool,
    pub contained_items_discovered: bool,
    pub actions: Vec<PerformAction>,
    pub events: Vec<GameEvent>,
}

pub async fn inspect_fixture(
    transaction: &mut Transaction<'_, Postgres>,
    username: &str,
    game_id: &str,
    args: &InspectFixture,
) -> Result<FixtureInspected, Box<dyn Error>> {
    let state = match super::repository::by_id(transaction, &username, &game_id)
        .await
        .unwrap()
    {
        Some(it) => it,
        None => return Err(Box::new(GameNotFoundError)),
    };
    let player = match crate::player_characters::repository::current(transaction, &username)
        .await
        .unwrap()
    {
        Some(it) => it,
        None => return Err(Box::new(NoPlayerCharacterSetError)),
    };
    let mut game = Game { state, player };

    let action = Action::InspectFixture(args.to_owned());
    let events = game.handle_action(&action)?;

    super::repository::save(transaction, &username, &game.state).await?;
    crate::player_characters::repository::save(transaction, &username, &game.player).await?;
    let game_events: Vec<GameEvent> = events.iter().cloned().map(GameEvent::from).collect();

    let mut fixture_inspected = FixtureInspected {
        actions: game_actions(&game, &username),
        can_be_opened_discovered: false,
        has_hidden_discovered: false,
        hidden_items_discovered: false,
        contained_items_discovered: false,
        events: game_events,
    };

    for event in events {
        match event {
            Event::FixtureCanBeOpenedDiscovered(_) => {
                fixture_inspected.can_be_opened_discovered = true;
            }
            Event::FixtureContainedDiscovered(_) => {
                fixture_inspected.contained_items_discovered = true;
            }
            Event::FixtureHasHiddenDiscovered(_) => {
                fixture_inspected.has_hidden_discovered = true;
            }
            Event::FixtureHiddenItemsDiscovered(_) => {
                fixture_inspected.hidden_items_discovered = true;
            }
            _ => {}
        }
    }

    Ok(fixture_inspected)
}
