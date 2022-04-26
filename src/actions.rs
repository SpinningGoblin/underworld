use poem_openapi::{Enum, Object};
use serde::Serialize;
use serde_json::Value;
use underworld_core::{actions::action::Action, components::rooms::room::Room};

use crate::{
    game::{
        attack::AttackNpcArgs,
        exit::ExitRoomArgs,
        look::{CharacterViewArgs, NpcLookArgs, RoomLookArgs},
        loot::LootNpcArgs,
    },
    player_characters::current::SetPlayerCharacterArgs,
};

#[derive(Enum, Serialize)]
#[oai(rename_all = "snake_case")]
pub enum ActionName {
    AttackNpc,
    CheckPlayerCharacter,
    ExitRoom,
    LookAtNpc,
    LookAtRoom,
    LootNpc,
    QuickLookRoom,
    SetCurrentPlayerCharacter,
}

#[derive(Object, Serialize)]
/// Actions, via a web call, that can be taken.
pub struct PerformAction {
    /// Name of the action.
    pub name: ActionName,
    /// What the action does, in English.
    pub description: String,
    /// The web link to complete the action, to put onto the base url of the server.
    pub link: String,
    /// What HTTP action to use.
    pub http_action: String,
    /// Any required args for the action, as JSON.
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
            name: ActionName::CheckPlayerCharacter,
            description: "Check status of player character.".to_string(),
            link: format!(
                "/{}/player_character/{}/check",
                username, player_character_id
            ),
            http_action: "GET".to_string(),
            args: None,
        },
        PerformAction {
            name: ActionName::SetCurrentPlayerCharacter,
            description: "Set the character as the current one to use for the game.".to_string(),
            link: get_api_link("set_current_player_character"),
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
                name: ActionName::LookAtRoom,
                description: it.description(),
                link: get_api_link("game/look_at_current_room"),
                http_action: "POST".to_string(),
                args: Some(serde_json::to_value(&look_args).unwrap()),
            }),
            Action::QuickLookRoom(it) => Some(PerformAction {
                name: ActionName::QuickLookRoom,
                description: it.description(),
                link: get_api_link("game/quick_look_current_room"),
                http_action: "POST".to_string(),
                args: Some(serde_json::to_value(&look_args).unwrap()),
            }),
            Action::AttackNpc(it) => {
                let attack_args = AttackNpcArgs {
                    username: username.to_string(),
                    game_id: game_id.to_string(),
                    npc_id: it.npc_id,
                };
                Some(PerformAction {
                    name: ActionName::AttackNpc,
                    description: "Attack an NPC in the room.".to_string(),
                    link: get_api_link("game/attack_npc"),
                    http_action: "POST".to_string(),
                    args: Some(serde_json::to_value(&attack_args).unwrap()),
                })
            }
            Action::ExitRoom(it) => {
                let exit_args = ExitRoomArgs {
                    username: username.to_string(),
                    game_id: game_id.to_string(),
                    exit_id: it.exit_id,
                };

                Some(PerformAction {
                    name: ActionName::ExitRoom,
                    description: "Exit current room using this exit.".to_string(),
                    link: get_api_link("game/exit_current_room"),
                    http_action: "POST".to_string(),
                    args: Some(serde_json::to_value(&exit_args).unwrap()),
                })
            }
            Action::LookAtNpc(it) => Some(PerformAction {
                name: ActionName::LookAtNpc,
                description: "Look at an NPC".to_string(),
                link: get_api_link("game/look_at_npc"),
                http_action: "POST".to_string(),
                args: Some(
                    serde_json::to_value(NpcLookArgs {
                        username: username.to_string(),
                        game_id: game_id.to_string(),
                        npc_id: it.npc_id,
                        knows_all: it.knows_all,
                        knows_name: it.knows_name,
                        view_args: CharacterViewArgs::from(&it.args),
                    })
                    .unwrap(),
                ),
            }),
            Action::LootNpc(loot_npc) => Some(PerformAction {
                name: ActionName::LootNpc,
                description: "Loot an NPC.".to_string(),
                link: get_api_link("game/loot_npc"),
                http_action: "POST".to_string(),
                args: Some(
                    serde_json::to_value(LootNpcArgs {
                        username: username.to_string(),
                        game_id: game_id.to_string(),
                        npc_id: loot_npc.npc_id,
                        item_ids: loot_npc.item_ids,
                    })
                    .unwrap(),
                ),
            }),
        })
        .collect()
}
