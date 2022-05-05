use poem_openapi::Object;
use redis::aio::Connection;
use serde::{Deserialize, Serialize};
use underworld_core::{
    components::player::PlayerCharacter,
    game::Game,
    generators::{game::game_generator, generator::Generator},
};

use crate::{
    actions::{game_actions, PerformAction},
    error::GameError,
    player_characters::current::get_current_player_character,
};

use super::set::set_game;

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
    connection: &mut Connection,
    args: &GenerateGameArgs,
) -> Result<GeneratedGame, GameError> {
    let game_generator = game_generator();
    let game_state = game_generator.generate();

    let game_id = game_state.identifier.id.to_string();

    set_game(connection, &game_state, &args.username).await;

    let player: PlayerCharacter = get_current_player_character(connection, &args.username).await?;
    let game = Game {
        state: game_state,
        player,
    };
    let actions = game_actions(&game, &args.username);
    Ok(GeneratedGame { actions, game_id })
}
