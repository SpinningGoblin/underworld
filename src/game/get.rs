use std::error::Error;

use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};
use underworld_core::game::Game;

use crate::{
    actions::PerformAction,
    error::{GameNotFoundError, NoPlayerCharacterSetError},
};

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
) -> Result<Vec<PerformAction>, Box<dyn Error>> {
    let state = match super::repository::by_id(transaction, &args.username, &args.game_id).await? {
        Some(game_state) => game_state,
        None => return Err(Box::new(GameNotFoundError)),
    };

    let player =
        match crate::player_characters::repository::current(transaction, &args.username).await? {
            Some(player) => player,
            None => return Err(Box::new(NoPlayerCharacterSetError)),
        };

    let game = Game { state, player };
    Ok(crate::actions::game_actions(&game, &args.username))
}
