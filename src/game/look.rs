use poem_openapi::Object;
use redis::aio::Connection;
use serde::{Deserialize, Serialize};
use underworld_core::{
    actions::{
        action::Action,
        look_at::{InspectNpc, LookAtNpc},
    },
    components::{
        non_player::NonPlayerView,
        rooms::room_view::{RoomView, RoomViewArgs},
    },
    events::event::Event,
    game::Game,
    systems::view::room,
};

use crate::{
    actions::{game_actions, PerformAction},
    error::GameError,
    player_characters::current::get_current_player_character,
};

use super::get::get_game_state;

#[derive(Deserialize, Object, Serialize)]
pub struct RoomLookArgs {
    pub username: String,
    pub game_id: String,
}

#[derive(Deserialize, Object, Serialize)]
pub struct NpcLookArgs {
    pub username: String,
    pub game_id: String,
    pub npc_id: String,
}

impl From<&NpcLookArgs> for LookAtNpc {
    fn from(val: &NpcLookArgs) -> Self {
        LookAtNpc {
            npc_id: val.npc_id.clone(),
        }
    }
}

pub async fn look_at_room(
    connection: &mut Connection,
    args: &RoomLookArgs,
) -> Result<RoomView, GameError> {
    let game_state = get_game_state(connection, &args.username, &args.game_id).await?;

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
    let game_state = get_game_state(connection, &args.username, &args.game_id).await?;

    Ok(room::quick_look(game_state.current_room()))
}

pub async fn look_at_npc(
    connection: &mut Connection,
    args: &NpcLookArgs,
) -> Result<NonPlayerView, GameError> {
    let state = get_game_state(connection, &args.username, &args.game_id).await?;
    let player = get_current_player_character(connection, &args.username).await?;
    let mut game = Game { state, player };
    let look_args: LookAtNpc = LookAtNpc::from(args);
    let action = Action::LookAtNpc(look_args);
    let events = game.handle_action(&action)?;

    match events.iter().find_map(|event| match event {
        Event::NpcViewed(it) => Some(it),
        _ => None,
    }) {
        Some(npc_viewed) => Ok(npc_viewed.npc_view.clone()),
        None => Err(GameError::General),
    }
}

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
    connection: &mut Connection,
    args: &InspectNpcArgs,
) -> Result<NpcInspected, GameError> {
    let state = get_game_state(connection, &args.username, &args.game_id).await?;
    let player = get_current_player_character(connection, &args.username).await?;
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
