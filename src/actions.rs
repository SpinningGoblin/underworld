use poem_openapi::{Enum, Object};
use serde::Serialize;
use serde_json::Value;
use underworld_core::{actions::action::Action, game::Game};

use crate::{
    game::{
        attack::AttackNpcArgs,
        exit::ExitRoomArgs,
        inspect::{InspectFixtureArgs, InspectNpcArgs},
        look::{FixtureLookArgs, NpcLookArgs, RoomLookArgs},
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
    InspectFixture,
    InspectNpc,
    LookAtFixture,
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

pub fn game_actions(game: &Game, username: &str) -> Vec<PerformAction> {
    let game_id = game.state.identifier.id.to_string();
    let look_args = RoomLookArgs {
        username: username.to_string(),
        game_id: game_id.clone(),
    };
    game.current_actions()
        .into_iter()
        .filter_map(|action| match action {
            Action::LookAtCurrentRoom(_) => Some(PerformAction {
                name: ActionName::LookAtRoom,
                description: "Look at current room".to_string(),
                link: get_api_link("game/look_around_room"),
                http_action: "POST".to_string(),
                args: Some(serde_json::to_value(&look_args).unwrap()),
            }),
            Action::AttackNpc(it) => {
                let attack_args = AttackNpcArgs {
                    username: username.to_string(),
                    game_id: game_id.clone(),
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
                    game_id: game_id.clone(),
                    exit_id: it.exit_id,
                };

                Some(PerformAction {
                    name: ActionName::ExitRoom,
                    description: "Exit current room using this exit.".to_string(),
                    link: get_api_link("game/exit_room"),
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
                        game_id: game_id.clone(),
                        npc_id: it.npc_id,
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
                        game_id: game_id.clone(),
                        npc_id: loot_npc.npc_id,
                        item_ids: loot_npc.item_ids,
                    })
                    .unwrap(),
                ),
            }),
            Action::MovePlayerItem(_) => None,
            Action::InspectNpc(inspect) => Some(PerformAction {
                name: ActionName::InspectNpc,
                description: "Inspect an NPC to reveal more information.".to_string(),
                link: get_api_link("game/inspect_npc"),
                http_action: "POST".to_string(),
                args: Some(
                    serde_json::to_value(InspectNpcArgs {
                        username: username.to_string(),
                        game_id: game_id.clone(),
                        npc_id: inspect.npc_id,
                        discover_health: inspect.discover_health,
                        discover_name: inspect.discover_name,
                        discover_packed_items: inspect.discover_packed_items,
                        discover_hidden_items: inspect.discover_hidden_items,
                    })
                    .unwrap(),
                ),
            }),
            Action::InspectFixture(inspect) => Some(PerformAction {
                name: ActionName::InspectFixture,
                description: "Inspect a fixture to discover new information".to_string(),
                link: get_api_link("game/inspect_npc"),
                http_action: "POST".to_string(),
                args: Some(
                    serde_json::to_value(InspectFixtureArgs {
                        username: username.to_string(),
                        game_id: game_id.clone(),
                        fixture_id: inspect.fixture_id,
                        discover_has_hidden: inspect.discover_hidden,
                        discover_hidden_items: inspect.discover_hidden_items,
                        discover_contained: inspect.discover_contained,
                        discover_can_be_opened: inspect.discover_can_be_opened,
                    })
                    .unwrap(),
                ),
            }),
            Action::LookAtFixture(look_at) => Some(PerformAction {
                name: ActionName::LookAtFixture,
                description: "Look at a fixture.".to_string(),
                link: get_api_link("game/look_at_fixture"),
                http_action: "POST".to_string(),
                args: Some(
                    serde_json::to_value(FixtureLookArgs {
                        username: username.to_string(),
                        game_id: game_id.clone(),
                        fixture_id: look_at.fixture_id,
                    })
                    .unwrap(),
                ),
            }),
            Action::LootFixture(_) => None,
        })
        .collect()
}
