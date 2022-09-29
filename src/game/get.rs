use sqlx::{Postgres, Transaction};
use underworld_core::{components::games::game_state::GameStateView, Game};

use crate::{actions::PerformAction, error::GameError};

pub async fn game_ids(transaction: &mut Transaction<'_, Postgres>, username: &str) -> Vec<String> {
    super::repository::ids(transaction, username).await.unwrap()
}

pub async fn game_actions(
    transaction: &mut Transaction<'_, Postgres>,
    username: &str,
    game_id: &str,
) -> Result<Vec<PerformAction>, GameError> {
    let state = match super::repository::by_id(transaction, username, game_id).await? {
        Some(game_state) => game_state,
        None => return Err(GameError::GameNotFoundError),
    };

    let player = match crate::player_characters::repository::current(transaction, username).await? {
        Some(player) => player,
        None => return Err(GameError::NoPlayerCharacterSetError),
    };

    let game = Game { state, player };
    Ok(crate::actions::game_actions(&game, username))
}

pub async fn game_state(
    transaction: &mut Transaction<'_, Postgres>,
    username: &str,
    game_id: &str,
) -> Result<GameStateView, GameError> {
    let state = match super::repository::by_id(transaction, username, game_id).await? {
        Some(game_state) => game_state,
        None => return Err(GameError::GameNotFoundError),
    };

    let view = underworld_core::systems::view::game_state::view(&state);
    Ok(view)
}
