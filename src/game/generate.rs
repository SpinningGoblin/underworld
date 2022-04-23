use poem_openapi::Object;
use redis::aio::Connection;
use serde::{Deserialize, Serialize};
use underworld_core::generators::{game::game_generator, generator::Generator};

use crate::{
    actions::{room_actions, PerformAction},
    error::GameError,
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
    let room_actions = room_actions(game_state.current_room(), &args.username, &game_id);
    Ok(GeneratedGame {
        actions: room_actions,
        game_id,
    })
}
