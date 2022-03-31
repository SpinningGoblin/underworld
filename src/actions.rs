use poem_openapi::Object;
use redis::aio::Connection;
use serde::Serialize;
use serde_json::Value;

use crate::{
    game::player_characters::SetPlayerCharacterArgs, player_characters::get::player_character_ids,
};

#[derive(Object, Serialize)]
pub struct PerformAction {
    pub name: String,
    pub description: String,
    pub link: String,
    pub http_action: String,
    pub args: Option<Value>,
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

pub async fn current_user_actions(
    username: &str,
    mut connection: &mut Connection,
) -> Vec<PerformAction> {
    let mut actions: Vec<PerformAction> = vec![
        PerformAction {
            name: "current_player_characters".to_string(),
            description: "Get current player characters.".to_string(),
            link: format!("{}/player_characters", username),
            http_action: "GET".to_string(),
            args: None,
        },
        PerformAction {
            name: "check_current_player_character".to_string(),
            description: "Check current player character.".to_string(),
            link: format!("/{}/check_current_player_character", &username),
            http_action: "GET".to_string(),
            args: None,
        },
    ];
    player_character_ids(&mut connection, username)
        .await
        .into_iter()
        .flat_map(|player_character_id| player_character_actions(username, &player_character_id))
        .for_each(|action| actions.push(action));

    actions
}
