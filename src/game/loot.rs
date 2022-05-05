use poem_openapi::Object;
use redis::aio::Connection;
use serde::{Deserialize, Serialize};
use underworld_core::{
    actions::{action::Action, loot_npc::LootNpc},
    game::Game,
};

use crate::{
    actions::{game_actions, PerformAction},
    error::GameError,
    event::GameEvent,
    player_characters::{current::get_current_player_character, set::set_player_character},
};

use super::{get::get_game_state, set::set_game};

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
    connection: &mut Connection,
    args: &LootNpcArgs,
) -> Result<NpcLooted, GameError> {
    let state = get_game_state(connection, &args.username, &args.game_id).await?;
    let player = get_current_player_character(connection, &args.username).await?;

    let mut game = Game { state, player };

    let loot_npc = LootNpc {
        npc_id: args.npc_id.clone(),
        item_ids: args.item_ids.clone(),
    };
    let action = Action::LootNpc(loot_npc);
    let events = game.handle_action(&action)?;

    set_game(connection, &game.state, &args.username).await;
    set_player_character(connection, &game.player, &args.username).await;

    let game_events: Vec<GameEvent> = events.into_iter().map(GameEvent::from).collect();

    Ok(NpcLooted {
        events: game_events,
        actions: game_actions(&game, &args.username),
    })
}
