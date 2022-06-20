use poem_openapi::{Enum, Object};
use serde::Serialize;
use serde_json::Value;
use underworld_core::{actions::action::Action, game::Game};

#[derive(Enum, Serialize)]
#[oai(rename_all = "snake_case")]
pub enum ActionName {
    AttackNpc,
    CastSpellOnNpc,
    CastSpellOnPlayer,
    CheckPlayerCharacter,
    ExitRoom,
    InspectFixture,
    InspectNpc,
    LookAtFixture,
    LookAtNpc,
    LookAtRoom,
    LootFixture,
    LootNpc,
    MovePlayerItem,
    OpenFixture,
    OpenFixtureHiddenCompartment,
    QuickLookRoom,
    SetCurrentPlayerCharacter,
    UseItemOnPlayer,
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

pub fn player_character_actions(_username: &str, player_character_id: &str) -> Vec<PerformAction> {
    vec![
        PerformAction {
            name: ActionName::CheckPlayerCharacter,
            description: "Check status of player character.".to_string(),
            link: format!("pcs/{}/check", player_character_id),
            http_action: "GET".to_string(),
            args: None,
        },
        PerformAction {
            name: ActionName::SetCurrentPlayerCharacter,
            description: "Set the character as the current one to use for the game.".to_string(),
            link: get_api_link(&format!("pcs/{}/set_as_current", player_character_id)),
            http_action: "POST".to_string(),
            args: None,
        },
    ]
}

pub fn game_actions(game: &Game, _username: &str) -> Vec<PerformAction> {
    let game_id = game.state.id.to_string();

    let view_actions = vec![PerformAction {
        name: ActionName::LookAtRoom,
        description: "Look at current room".to_string(),
        link: get_api_link(&format!("game/{}/look_around_room", &game_id)),
        http_action: "POST".to_string(),
        args: None,
    }];

    let game_actions = game
        .current_actions()
        .into_iter()
        .map(|action| match action {
            Action::AttackNpc(it) => PerformAction {
                name: ActionName::AttackNpc,
                description: "Attack an NPC in the room.".to_string(),
                link: get_api_link(&format!("game/{}/attack_npc", &game_id)),
                http_action: "POST".to_string(),
                args: Some(serde_json::to_value(&it).unwrap()),
            },
            Action::ExitRoom(it) => PerformAction {
                name: ActionName::ExitRoom,
                description: "Exit current room using this exit.".to_string(),
                link: get_api_link(&format!("game/{}/exit_room", &game_id)),
                http_action: "POST".to_string(),
                args: Some(serde_json::to_value(&it).unwrap()),
            },
            Action::LookAtNpc(it) => PerformAction {
                name: ActionName::LookAtNpc,
                description: "Look at an NPC".to_string(),
                link: get_api_link(&format!("game/{}/look_at_npc", &game_id)),
                http_action: "POST".to_string(),
                args: Some(serde_json::to_value(&it).unwrap()),
            },
            Action::LootNpc(loot_npc) => PerformAction {
                name: ActionName::LootNpc,
                description: "Loot an NPC.".to_string(),
                link: get_api_link(&format!("game/{}/loot_npc", &game_id)),
                http_action: "POST".to_string(),
                args: Some(serde_json::to_value(&loot_npc).unwrap()),
            },
            Action::MovePlayerItem(move_player_item) => PerformAction {
                name: ActionName::MovePlayerItem,
                description: "Move an item around on the player, either equipping or unequipping."
                    .to_string(),
                link: get_api_link(&format!("game/{}/move_player_item", &game_id)),
                http_action: "POST".to_string(),
                args: Some(serde_json::to_value(&move_player_item).unwrap()),
            },
            Action::InspectNpc(inspect) => PerformAction {
                name: ActionName::InspectNpc,
                description: "Inspect an NPC to reveal more information.".to_string(),
                link: get_api_link(&format!("game/{}/inspect_npc", &game_id)),
                http_action: "POST".to_string(),
                args: Some(serde_json::to_value(&inspect).unwrap()),
            },
            Action::InspectFixture(inspect) => PerformAction {
                name: ActionName::InspectFixture,
                description: "Inspect a fixture to discover new information".to_string(),
                link: get_api_link(&format!("game/{}/inspect_npc", &game_id)),
                http_action: "POST".to_string(),
                args: Some(serde_json::to_value(&inspect).unwrap()),
            },
            Action::LookAtFixture(look_at) => PerformAction {
                name: ActionName::LookAtFixture,
                description: "Look at a fixture.".to_string(),
                link: get_api_link(&format!("game/{}/look_at_fixture", &game_id)),
                http_action: "POST".to_string(),
                args: Some(serde_json::to_value(&look_at).unwrap()),
            },
            Action::LootFixture(loot_fixture) => PerformAction {
                name: ActionName::LootFixture,
                description: "Loot a fixture".to_string(),
                link: get_api_link(&format!("game/{}/loot_fixture", &game_id)),
                http_action: "POST".to_string(),
                args: Some(serde_json::to_value(&loot_fixture).unwrap()),
            },
            Action::CastSpellOnNpc(cast_spell_on_npc) => PerformAction {
                name: ActionName::CastSpellOnNpc,
                description: "Cast a spell on an NPC".to_string(),
                link: get_api_link(&format!("game/{}/cast_spell_on_npc", &game_id)),
                http_action: "POST".to_string(),
                args: Some(serde_json::to_value(&cast_spell_on_npc).unwrap()),
            },
            Action::CastSpellOnPlayer(cast_spell_on_player) => PerformAction {
                name: ActionName::CastSpellOnPlayer,
                description: "Cast a spell on yourself".to_string(),
                link: get_api_link(&format!("game/{}/cast_spell_on_player", &game_id)),
                http_action: "POST".to_string(),
                args: Some(serde_json::to_value(&cast_spell_on_player).unwrap()),
            },
            Action::UseItemOnPlayer(use_item_on_player) => PerformAction {
                name: ActionName::UseItemOnPlayer,
                description: "Use an item on yourself".to_string(),
                link: get_api_link(&format!("game/{}/use_item_on_player", &game_id)),
                http_action: "POST".to_string(),
                args: Some(serde_json::to_value(&use_item_on_player).unwrap()),
            },
            Action::OpenFixture(open_fixture) => PerformAction {
                name: ActionName::OpenFixture,
                description: "Open a fixture.".to_string(),
                link: get_api_link(&format!("game/{}/open_fixture", &game_id)),
                http_action: "POST".to_string(),
                args: Some(serde_json::to_value(&open_fixture).unwrap()),
            },
            Action::OpenFixtureHiddenCompartment(open) => PerformAction {
                name: ActionName::OpenFixtureHiddenCompartment,
                description: "Open the hidden compartment, if there is one, of the fixture."
                    .to_string(),
                link: get_api_link(&format!(
                    "game/{}/open_fixture_hidden_compartment",
                    &game_id
                )),
                http_action: "POST".to_string(),
                args: Some(serde_json::to_value(&open).unwrap()),
            },
        });

    game_actions.chain(view_actions.into_iter()).collect()
}
