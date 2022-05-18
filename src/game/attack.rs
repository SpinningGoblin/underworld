use std::error::Error;

use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};
use underworld_core::{
    actions::{action::Action, attack_npc::AttackNpc},
    game::Game,
};

use crate::{
    actions::{game_actions, PerformAction},
    error::{GameNotFoundError, NoPlayerCharacterSetError},
    event::GameEvent,
};

#[derive(Serialize, Object)]
/// Results from attack on the NPC.
pub struct NpcAttacked {
    /// Events that happened due to the attack.
    events: Vec<GameEvent>,
    /// Actions that can now be performed after the attack.
    actions: Vec<PerformAction>,
}

#[derive(Deserialize, Object, Serialize)]
/// Args for an attack action against a single NPC.
pub struct AttackNpcArgs {
    /// Username for the action.
    pub username: String,
    /// The ID of the game which the action will happen in.
    pub game_id: String,
    /// ID of the NPC being attacked.
    pub npc_id: String,
}

pub async fn attack_npc(
    transaction: &mut Transaction<'_, Postgres>,
    args: &AttackNpcArgs,
) -> Result<NpcAttacked, Box<dyn Error>> {
    let player_character =
        match crate::player_characters::repository::current(transaction, &args.username).await? {
            Some(it) => it,
            None => return Err(Box::new(NoPlayerCharacterSetError)),
        };

    let state = match super::repository::by_id(transaction, &args.username, &args.game_id).await? {
        Some(it) => it,
        None => return Err(Box::new(GameNotFoundError)),
    };

    let mut game = Game {
        player: player_character,
        state,
    };

    let attack_npc = AttackNpc {
        npc_id: args.npc_id.clone(),
    };

    let events = game.handle_action(&Action::AttackNpc(attack_npc)).unwrap();
    super::repository::save(transaction, &args.username, &game.state).await?;
    crate::player_characters::repository::save(transaction, &args.username, &game.player).await?;

    let game_events: Vec<GameEvent> = events.into_iter().map(GameEvent::from).collect();

    Ok(NpcAttacked {
        events: game_events,
        actions: game_actions(&game, &args.username),
    })
}
