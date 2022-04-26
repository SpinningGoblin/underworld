use poem_openapi::Object;
use redis::aio::Connection;
use serde::{Deserialize, Serialize};
use underworld_core::{
    actions::{action::Action, attack_npc::AttackNpc},
    game::Game,
};

use crate::{
    actions::{room_actions, PerformAction},
    error::GameError,
    event::GameEvent,
    player_characters::{current::get_current_player_character, set::set_player_character},
};

use super::{get::get_game_state, set::set_game};

#[derive(Serialize, Object)]
pub struct NpcAttacked {
    /// Events that happened due to the attack.
    events: Vec<GameEvent>,
    /// Actions that can now be performed after the attack.
    actions: Vec<PerformAction>,
}

#[derive(Deserialize, Object, Serialize)]
pub struct AttackNpcArgs {
    pub username: String,
    pub game_id: String,
    pub npc_id: String,
}

pub async fn attack_npc(
    connection: &mut Connection,
    args: &AttackNpcArgs,
) -> Result<NpcAttacked, GameError> {
    let player_character = match get_current_player_character(connection, &args.username).await {
        Ok(it) => it,
        Err(it) => return Err(it),
    };

    let state = get_game_state(connection, &args.username, &args.game_id).await?;

    let mut game = Game {
        player: player_character,
        state,
    };

    let attack_npc = AttackNpc {
        npc_id: args.npc_id.clone(),
    };

    let events = game.handle_action(&Action::AttackNpc(attack_npc)).unwrap();
    set_game(connection, &game.state, &args.username).await;
    set_player_character(connection, &game.player, &args.username).await;

    let game_events: Vec<GameEvent> = events.into_iter().map(GameEvent::from).collect();

    Ok(NpcAttacked {
        events: game_events,
        actions: room_actions(
            game.state.current_room(),
            &args.username,
            &game.state.identifier.id.to_string(),
        ),
    })
}
