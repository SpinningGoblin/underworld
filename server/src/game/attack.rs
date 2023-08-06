use poem_openapi::Object;
use serde::Serialize;
use sqlx::{Postgres, Transaction};
use underworld_core::{
    actions::{Action, AttackNpc},
    components::{rooms::RoomView, PlayerCharacterView},
    Game,
};

use crate::{
    actions::{game_actions, PerformAction},
    error::GameError,
    event::GameEvent,
};

#[derive(Serialize, Object)]
/// Results from attack on the NPC.
pub struct NpcAttacked {
    /// Events that happened due to the attack.
    events: Vec<GameEvent>,
    /// Actions that can now be performed after the attack.
    actions: Vec<PerformAction>,
    current_room: RoomView,
    current_player: PlayerCharacterView,
}

pub async fn attack_npc(
    transaction: &mut Transaction<'_, Postgres>,
    username: &str,
    game_id: &str,
    args: &AttackNpc,
) -> Result<NpcAttacked, GameError> {
    let player_character =
        match crate::player_characters::repository::current(transaction, username).await? {
            Some(it) => it,
            None => return Err(GameError::NoPlayerCharacterSetError),
        };

    let state = match super::repository::by_id(transaction, username, game_id).await? {
        Some(it) => it,
        None => return Err(GameError::GameNotFoundError),
    };

    let mut game = Game {
        player: player_character,
        state,
    };

    let events = game.handle_action(&Action::AttackNpc(args.to_owned()))?;
    super::repository::save(transaction, username, &game.state).await?;
    crate::player_characters::repository::save(transaction, username, &game.player).await?;

    let game_events: Vec<GameEvent> = events.into_iter().map(GameEvent::from).collect();

    let current_room = game.state.view_current_room();
    let current_player = underworld_core::systems::view::player::check(&game.player);

    Ok(NpcAttacked {
        events: game_events,
        actions: game_actions(&game, username),
        current_room,
        current_player,
    })
}
