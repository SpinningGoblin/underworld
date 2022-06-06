use poem_openapi::Object;
use serde::Serialize;
use sqlx::{Postgres, Transaction};
use underworld_core::{
    game::Game,
    generators::{game::game_generator, generator::Generator},
};

use crate::{
    actions::{game_actions, PerformAction},
    error::GameError,
};

#[derive(Serialize, Object)]
pub struct GeneratedGame {
    pub game_id: String,
    pub actions: Vec<PerformAction>,
}

pub async fn generate_game(
    transaction: &mut Transaction<'_, Postgres>,
    username: &str,
) -> Result<GeneratedGame, GameError> {
    let game_generator = game_generator();
    let game_state = game_generator.generate();

    super::repository::save(transaction, &username, &game_state).await?;

    let player = match crate::player_characters::repository::current(transaction, &username).await?
    {
        Some(it) => it,
        None => return Err(GameError::NoPlayerCharacterSetError),
    };

    let game_id = game_state.id.to_string();
    let game = Game {
        state: game_state,
        player,
    };
    let actions = game_actions(&game, &username);
    Ok(GeneratedGame { actions, game_id })
}
