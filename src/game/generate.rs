use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};
use underworld_core::{
    game::Game,
    generators::{game::game_generator, generator::Generator},
};

use crate::{
    actions::{game_actions, PerformAction},
    error::GameError,
};

#[derive(Deserialize, Object)]
pub struct GenerateGameArgs {
    pub username: String,
}

#[derive(Serialize, Object)]
pub struct GeneratedGame {
    pub game_id: String,
    pub actions: Vec<PerformAction>,
}

pub async fn generate_game(
    transaction: &mut Transaction<'_, Postgres>,
    args: &GenerateGameArgs,
) -> Result<GeneratedGame, GameError> {
    let game_generator = game_generator();
    let game_state = game_generator.generate();

    super::repository::save(transaction, &args.username, &game_state)
        .await
        .unwrap();

    let player =
        match crate::player_characters::repository::current(transaction, &args.username).await {
            Ok(Some(it)) => it,
            Ok(None) => return Err(GameError::NoPlayerCharacterSet),
            Err(_) => return Err(GameError::General),
        };

    let game_id = game_state.identifier.id.to_string();
    let game = Game {
        state: game_state,
        player,
    };
    let actions = game_actions(&game, &args.username);
    Ok(GeneratedGame { actions, game_id })
}
