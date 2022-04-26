use poem_openapi::Object;
use redis::aio::Connection;
use serde::{Deserialize, Serialize};
use underworld_core::{
    actions::{action::Action, look_at::LookAtNpc},
    components::{
        character,
        non_player::NonPlayerView,
        rooms::room_view::{RoomView, RoomViewArgs},
    },
    events::event::Event,
    game::Game,
    systems::view::room,
};

use crate::{error::GameError, player_characters::current::get_current_player_character};

use super::get::get_game_state;

#[derive(Deserialize, Object, Serialize)]
pub struct RoomLookArgs {
    pub username: String,
    pub game_id: String,
}

#[derive(Deserialize, Object, Serialize)]
pub struct NpcLookArgs {
    pub username: String,
    pub game_id: String,
    pub npc_id: String,
    pub knows_all: bool,
    pub knows_name: bool,
    pub view_args: CharacterViewArgs,
}

impl From<&NpcLookArgs> for LookAtNpc {
    fn from(val: &NpcLookArgs) -> Self {
        LookAtNpc {
            npc_id: val.npc_id.clone(),
            args: character::CharacterViewArgs::from(&val.view_args),
            knows_name: val.knows_name,
            knows_all: val.knows_all,
        }
    }
}

#[derive(Deserialize, Object, Serialize)]
pub struct CharacterViewArgs {
    pub knows_health: bool,
    pub knows_species: bool,
    pub knows_life_modifier: bool,
    pub knows_inventory: bool,
    pub knows_hidden_in_inventory: bool,
    pub knows_packed_in_inventory: bool,
}

impl From<&CharacterViewArgs> for character::CharacterViewArgs {
    fn from(val: &CharacterViewArgs) -> Self {
        character::CharacterViewArgs {
            knows_health: val.knows_health,
            knows_species: val.knows_species,
            knows_life_modifier: val.knows_life_modifier,
            knows_inventory: val.knows_inventory,
            knows_hidden_in_inventory: val.knows_hidden_in_inventory,
            knows_packed_in_inventory: val.knows_packed_in_inventory,
        }
    }
}

impl From<&character::CharacterViewArgs> for CharacterViewArgs {
    fn from(val: &character::CharacterViewArgs) -> Self {
        CharacterViewArgs {
            knows_health: val.knows_health,
            knows_species: val.knows_species,
            knows_life_modifier: val.knows_life_modifier,
            knows_inventory: val.knows_inventory,
            knows_hidden_in_inventory: val.knows_hidden_in_inventory,
            knows_packed_in_inventory: val.knows_packed_in_inventory,
        }
    }
}

pub async fn look_at_room(
    connection: &mut Connection,
    args: &RoomLookArgs,
) -> Result<RoomView, GameError> {
    let game_state = match get_game_state(connection, &args.username, &args.game_id).await {
        Some(it) => it,
        None => return Err(GameError::GameNotFound),
    };

    let args = RoomViewArgs {
        can_see_hidden: false,
        can_see_packed: false,
        knows_character_health: false,
        knows_names: true,
    };
    Ok(room::look_at(game_state.current_room(), args, false))
}

pub async fn quick_look_room(
    connection: &mut Connection,
    args: &RoomLookArgs,
) -> Result<RoomView, GameError> {
    let game_state = match get_game_state(connection, &args.username, &args.game_id).await {
        Some(it) => it,
        None => return Err(GameError::GameNotFound),
    };

    Ok(room::quick_look(game_state.current_room()))
}

pub async fn look_at_npc(
    connection: &mut Connection,
    args: &NpcLookArgs,
) -> Result<NonPlayerView, GameError> {
    let state = match get_game_state(connection, &args.username, &args.game_id).await {
        Some(it) => it,
        None => return Err(GameError::GameNotFound),
    };
    let player = get_current_player_character(connection, &args.username).await?;
    let mut game = Game { state, player };
    let look_args: LookAtNpc = LookAtNpc::from(args);
    let action = Action::LookAtNpc(look_args);
    let events = game.handle_action(&action)?;

    match events.iter().find_map(|event| match event {
        Event::NpcViewed(it) => Some(it),
        _ => None,
    }) {
        Some(npc_viewed) => Ok(npc_viewed.npc_view.clone()),
        None => Err(GameError::General),
    }
}
