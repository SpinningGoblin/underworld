use std::error::Error;

use poem_openapi::Object;
use serde::Serialize;
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

pub async fn attack_npc(
    transaction: &mut Transaction<'_, Postgres>,
    username: &str,
    game_id: &str,
    args: &AttackNpc,
) -> Result<NpcAttacked, Box<dyn Error>> {
    let player_character =
        match crate::player_characters::repository::current(transaction, &username).await? {
            Some(it) => it,
            None => return Err(Box::new(NoPlayerCharacterSetError)),
        };

    let state = match super::repository::by_id(transaction, &username, &game_id).await? {
        Some(it) => it,
        None => return Err(Box::new(GameNotFoundError)),
    };

    let mut game = Game {
        player: player_character,
        state,
    };

    let events = game
        .handle_action(&Action::AttackNpc(args.to_owned()))
        .unwrap();
    super::repository::save(transaction, &username, &game.state).await?;
    crate::player_characters::repository::save(transaction, &username, &game.player).await?;

    let game_events: Vec<GameEvent> = events.into_iter().map(GameEvent::from).collect();

    Ok(NpcAttacked {
        events: game_events,
        actions: game_actions(&game, &username),
    })
}
