use poem_openapi::Object;
use serde::Serialize;
use sqlx::{Postgres, Transaction};
use underworld_core::{
    actions::{Action, CastSpellOnNpc, CastSpellOnPlayer},
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
pub struct SpellCast {
    /// Events that happened due to the attack.
    pub events: Vec<GameEvent>,
    /// Actions that can now be performed after the attack.
    pub actions: Vec<PerformAction>,
    pub current_room: RoomView,
    pub current_player: PlayerCharacterView,
}

pub async fn cast_spell_on_player(
    transaction: &mut Transaction<'_, Postgres>,
    username: &str,
    game_id: &str,
    args: &CastSpellOnPlayer,
) -> Result<SpellCast, GameError> {
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

    let events = game.handle_action(&Action::CastSpellOnPlayer(args.to_owned()))?;
    super::repository::save(transaction, username, &game.state).await?;
    crate::player_characters::repository::save(transaction, username, &game.player).await?;

    let game_events: Vec<GameEvent> = events.into_iter().map(GameEvent::from).collect();

    let current_room = game.state.view_current_room();
    let current_player = underworld_core::systems::view::player::check(&game.player);

    Ok(SpellCast {
        events: game_events,
        actions: game_actions(&game, username),
        current_player,
        current_room,
    })
}

pub async fn cast_spell_on_npc(
    transaction: &mut Transaction<'_, Postgres>,
    username: &str,
    game_id: &str,
    args: &CastSpellOnNpc,
) -> Result<SpellCast, GameError> {
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

    let events = game.handle_action(&Action::CastSpellOnNpc(args.to_owned()))?;
    super::repository::save(transaction, username, &game.state).await?;
    crate::player_characters::repository::save(transaction, username, &game.player).await?;

    let game_events: Vec<GameEvent> = events.into_iter().map(GameEvent::from).collect();

    let current_room = game.state.view_current_room();
    let current_player = underworld_core::systems::view::player::check(&game.player);

    Ok(SpellCast {
        events: game_events,
        actions: game_actions(&game, username),
        current_player,
        current_room,
    })
}
