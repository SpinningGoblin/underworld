use std::error::Error;

use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};
use underworld_core::{
    actions::{action::Action, inspect_fixture::InspectFixture, inspect_npc::InspectNpc},
    events::event::Event,
    game::Game,
};

use crate::{
    actions::{game_actions, PerformAction},
    error::{GameNotFoundError, NoPlayerCharacterSetError},
};

#[derive(Deserialize, Object, Serialize)]
pub struct InspectNpcArgs {
    /// Username to use.
    pub username: String,
    /// Game to perform action.
    pub game_id: String,
    /// NPC to inspect.
    pub npc_id: String,
    /// Attempt to discover the NPC's health.
    pub discover_health: bool,
    /// Attempt to discover the NPC's name.
    pub discover_name: bool,
    /// Attempt to discover the items the NPC has packed away.
    pub discover_packed_items: bool,
    /// Attempt to discover any hidden items the NPC has.
    pub discover_hidden_items: bool,
}

#[derive(Object, Serialize)]
pub struct NpcInspected {
    pub health_discovered: bool,
    pub name_discovered: bool,
    pub packed_items_discovered: bool,
    pub hidden_items_discovered: bool,
    pub actions: Vec<PerformAction>,
}

pub async fn inspect_npc(
    transaction: &mut Transaction<'_, Postgres>,
    args: &InspectNpcArgs,
) -> Result<NpcInspected, Box<dyn Error>> {
    let state = match super::repository::by_id(transaction, &args.username, &args.game_id).await? {
        Some(it) => it,
        None => return Err(Box::new(GameNotFoundError)),
    };
    let player =
        match crate::player_characters::repository::current(transaction, &args.username).await? {
            Some(it) => it,
            None => return Err(Box::new(NoPlayerCharacterSetError)),
        };
    let mut game = Game { state, player };

    let inspect_args = InspectNpc {
        npc_id: args.npc_id.clone(),
        discover_health: args.discover_health,
        discover_name: args.discover_name,
        discover_packed_items: args.discover_packed_items,
        discover_hidden_items: args.discover_hidden_items,
    };
    let action = Action::InspectNpc(inspect_args);
    let events = game.handle_action(&action)?;

    super::repository::save(transaction, &args.username, &game.state)
        .await
        .unwrap();
    crate::player_characters::repository::save(transaction, &args.username, &game.player)
        .await
        .unwrap();

    let mut npc_inspected = NpcInspected {
        health_discovered: false,
        name_discovered: false,
        packed_items_discovered: false,
        hidden_items_discovered: false,
        actions: game_actions(&game, &args.username),
    };

    for event in events {
        match event {
            Event::NpcHealthDiscovered(_) => {
                npc_inspected.health_discovered = true;
            }
            Event::NpcHiddenDiscovered(_) => {
                npc_inspected.hidden_items_discovered = true;
            }
            Event::NpcNameDiscovered(_) => {
                npc_inspected.name_discovered = true;
            }
            Event::NpcPackedDiscovered(_) => {
                npc_inspected.packed_items_discovered = true;
            }
            _ => {}
        }
    }

    Ok(npc_inspected)
}

#[derive(Deserialize, Object, Serialize)]
pub struct InspectFixtureArgs {
    /// Username to use.
    pub username: String,
    /// Game to perform action.
    pub game_id: String,
    /// NPC to inspect.
    pub fixture_id: String,
    /// Attempt to discover any hidden compartments and its contents.
    pub discover_has_hidden: bool,
    /// Attempt to discover any items in any hidden compartments.
    pub discover_hidden_items: bool,
    /// Attempt to discover the items inside of the container, without opening.
    pub discover_contained: bool,
    /// Attempt to discover if the fixture can be opened.
    pub discover_can_be_opened: bool,
}

#[derive(Object, Serialize)]
pub struct FixtureInspected {
    pub can_be_opened_discovered: bool,
    pub has_hidden_discovered: bool,
    pub hidden_items_discovered: bool,
    pub contained_items_discovered: bool,
    pub actions: Vec<PerformAction>,
}

pub async fn inspect_fixture(
    transaction: &mut Transaction<'_, Postgres>,
    args: &InspectFixtureArgs,
) -> Result<FixtureInspected, Box<dyn Error>> {
    let state = match super::repository::by_id(transaction, &args.username, &args.game_id)
        .await
        .unwrap()
    {
        Some(it) => it,
        None => return Err(Box::new(GameNotFoundError)),
    };
    let player = match crate::player_characters::repository::current(transaction, &args.username)
        .await
        .unwrap()
    {
        Some(it) => it,
        None => return Err(Box::new(NoPlayerCharacterSetError)),
    };
    let mut game = Game { state, player };

    let inspect_args = InspectFixture {
        fixture_id: args.fixture_id.clone(),
        discover_can_be_opened: args.discover_can_be_opened,
        discover_contained: args.discover_contained,
        discover_hidden: args.discover_has_hidden,
        discover_hidden_items: args.discover_hidden_items,
    };
    let action = Action::InspectFixture(inspect_args);
    let events = game.handle_action(&action)?;

    super::repository::save(transaction, &args.username, &game.state).await?;
    crate::player_characters::repository::save(transaction, &args.username, &game.player).await?;

    let mut fixture_inspected = FixtureInspected {
        actions: game_actions(&game, &args.username),
        can_be_opened_discovered: false,
        has_hidden_discovered: false,
        hidden_items_discovered: false,
        contained_items_discovered: false,
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
