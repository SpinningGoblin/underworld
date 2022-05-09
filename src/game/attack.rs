use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};
use underworld_core::{
    actions::{action::Action, attack_npc::AttackNpc},
    game::Game,
};

use crate::{
    actions::{game_actions, PerformAction},
    error::GameError,
    event::GameEvent,
};

#[derive(Serialize, Object)]
pub struct NpcAttacked {
    /// Events that happened due to the attack.
    events: Vec<GameEvent>,
    /// Actions that can now be performed after the attack.
    actions: Vec<PerformAction>,
}

#[derive(Deserialize, Object, Serialize)]
pub struct AttackNpcArgs {
    pub username: String,
    pub game_id: String,
    pub npc_id: String,
}

pub async fn attack_npc(
    transaction: &mut Transaction<'_, Postgres>,
    args: &AttackNpcArgs,
) -> Result<NpcAttacked, GameError> {
    let player_character =
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

    let mut game = Game {
        player: player_character,
        state,
    };

    let attack_npc = AttackNpc {
        npc_id: args.npc_id.clone(),
    };

    let events = game.handle_action(&Action::AttackNpc(attack_npc)).unwrap();
    super::repository::save(transaction, &args.username, &game.state)
        .await
        .unwrap();
    crate::player_characters::repository::save(transaction, &args.username, &game.player)
        .await
        .unwrap();

    let game_events: Vec<GameEvent> = events.into_iter().map(GameEvent::from).collect();

    Ok(NpcAttacked {
        events: game_events,
        actions: game_actions(&game, &args.username),
    })
}
