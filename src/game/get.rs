use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};
use underworld_core::game::Game;

use crate::{actions::PerformAction, error::GameError};

pub async fn game_ids(transaction: &mut Transaction<'_, Postgres>, username: &str) -> Vec<String> {
    super::repository::ids(transaction, username).await.unwrap()
}

#[derive(Deserialize, Object, Serialize)]
pub struct GameActionsArgs {
    pub username: String,
    pub game_id: String,
}

pub async fn game_actions(
    transaction: &mut Transaction<'_, Postgres>,
    args: &GameActionsArgs,
) -> Result<Vec<PerformAction>, GameError> {
    let state = match super::repository::by_id(transaction, &args.username, &args.game_id).await {
        Ok(Some(game_state)) => game_state,
        Ok(None) => return Err(GameError::GameNotFound),
        Err(_) => return Err(GameError::General),
    };

    let player =
        match crate::player_characters::repository::current(transaction, &args.username).await {
            Ok(Some(player)) => player,
            Ok(None) => return Err(GameError::NoPlayerCharacterSet),
            Err(_) => return Err(GameError::General),
        };

    let game = Game { state, player };
    Ok(crate::actions::game_actions(&game, &args.username))
}
