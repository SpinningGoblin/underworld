use std::error::Error;

use sqlx::{Postgres, Transaction};
use underworld_core::game::Game;

use crate::{
    actions::PerformAction,
    error::{GameNotFoundError, NoPlayerCharacterSetError},
};

pub async fn game_ids(transaction: &mut Transaction<'_, Postgres>, username: &str) -> Vec<String> {
    super::repository::ids(transaction, username).await.unwrap()
}

pub async fn game_actions(
    transaction: &mut Transaction<'_, Postgres>,
    username: &str,
    game_id: &str,
) -> Result<Vec<PerformAction>, Box<dyn Error>> {
    let state = match super::repository::by_id(transaction, &username, &game_id).await? {
        Some(game_state) => game_state,
        None => return Err(Box::new(GameNotFoundError)),
    };

    let player = match crate::player_characters::repository::current(transaction, &username).await?
    {
        Some(player) => player,
        None => return Err(Box::new(NoPlayerCharacterSetError)),
    };

    let game = Game { state, player };
    Ok(crate::actions::game_actions(&game, &username))
}
