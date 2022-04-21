use poem_openapi::Object;
use serde::Serialize;
use serde_json::Value;
use underworld_core::{actions::action::Action, components::rooms::room::Room};

use crate::{
    game::args::{ExitRoomArgs, RoomLookArgs},
    player_characters::current::SetPlayerCharacterArgs,
};

#[derive(Object, Serialize)]
pub struct PerformAction {
    pub name: String,
    pub description: String,
    pub link: String,
    pub http_action: String,
    pub args: Option<Value>,
}

pub fn get_api_link(original: &str) -> String {
    format!("/api/{}", original)
}

pub fn player_character_actions(username: &str, player_character_id: &str) -> Vec<PerformAction> {
    let args = SetPlayerCharacterArgs {
        username: username.to_string(),
        player_character_id: player_character_id.to_string(),
    };

    vec![
        PerformAction {
            name: "check_player_character".to_string(),
            description: "Check status of player character.".to_string(),
            link: format!(
                "/{}/player_character/{}/check",
                username, player_character_id
            ),
            http_action: "GET".to_string(),
            args: None,
        },
        PerformAction {
            name: "set_current_player_character".to_string(),
            description: "Set the character as the current one to use for the game.".to_string(),
            link: "/set_current_player".to_string(),
            http_action: "POST".to_string(),
            args: Some(serde_json::to_value(&args).unwrap()),
        },
    ]
}

pub fn room_actions(room: &Room, username: &str, game_id: &str) -> Vec<PerformAction> {
    let look_args = RoomLookArgs {
        username: username.to_string(),
        game_id: game_id.to_string(),
    };
    room.current_actions()
        .into_iter()
        .filter_map(|action| match action {
            Action::LookAtTarget(_) => None,
            Action::LookAtRoom(it) => Some(PerformAction {
                name: "look_at_room".to_string(),
                description: it.description(),
                link: get_api_link("game/look_at_current_room"),
                http_action: "POST".to_string(),
                args: Some(serde_json::to_value(&look_args).unwrap()),
            }),
            Action::QuickLookRoom(it) => Some(PerformAction {
                name: "quick_look_room".to_string(),
                description: it.description(),
                link: get_api_link("game/quick_look_current_room"),
                http_action: "POST".to_string(),
                args: Some(serde_json::to_value(&look_args).unwrap()),
            }),
            Action::AttackNpc(_) => None,
            Action::ExitRoom(it) => {
                let exit_args = ExitRoomArgs {
                    username: username.to_string(),
                    game_id: game_id.to_string(),
                    exit_id: it.exit_id,
                };

                Some(PerformAction {
                    name: "exit_room".to_string(),
                    description: "Exit current room using this exit.".to_string(),
                    link: get_api_link("game/exit_current_room"),
                    http_action: "POST".to_string(),
                    args: Some(serde_json::to_value(&exit_args).unwrap()),
                })
            }
        })
        .collect()
}
