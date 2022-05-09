use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};
use underworld_core::{
    actions::{action::Action, loot_npc::LootNpc},
    game::Game,
};

use crate::{
    actions::{game_actions, PerformAction},
    error::GameError,
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
) -> Result<NpcLooted, GameError> {
    let player =
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

    let mut game = Game { state, player };

    let loot_npc = LootNpc {
        npc_id: args.npc_id.clone(),
        item_ids: args.item_ids.clone(),
    };
    let action = Action::LootNpc(loot_npc);
    let events = game.handle_action(&action)?;

    super::repository::save(transaction, &args.username, &game.state)
        .await
        .unwrap();
    crate::player_characters::repository::save(transaction, &args.username, &game.player)
        .await
        .unwrap();

    let game_events: Vec<GameEvent> = events.into_iter().map(GameEvent::from).collect();

    Ok(NpcLooted {
        events: game_events,
        actions: game_actions(&game, &args.username),
    })
}
