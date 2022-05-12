use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};
use underworld_core::{
    actions::{
        action::Action, look_at_current_room::LookAtCurrentRoom, look_at_fixture::LookAtFixture,
        look_at_npc::LookAtNpc,
    },
    components::{
        fixtures::fixture::FixtureView, non_player::NonPlayerView, rooms::room_view::RoomView,
    },
    events::event::Event,
    game::Game,
};

use crate::error::GameError;

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
}

impl From<&NpcLookArgs> for LookAtNpc {
    fn from(val: &NpcLookArgs) -> Self {
        LookAtNpc {
            npc_id: val.npc_id.clone(),
        }
    }
}

pub async fn look_at_room(
    transaction: &mut Transaction<'_, Postgres>,
    args: &RoomLookArgs,
) -> Result<RoomView, GameError> {
    let state = match super::repository::by_id(transaction, &args.username, &args.game_id)
        .await
        .unwrap()
    {
        Some(it) => it,
        None => return Err(GameError::GameNotFound),
    };

    let player = match crate::player_characters::repository::current(transaction, &args.username)
        .await
        .unwrap()
    {
        Some(it) => it,
        None => return Err(GameError::NoPlayerCharacterSet),
    };

    let mut game = Game { state, player };

    let action = Action::LookAtCurrentRoom(LookAtCurrentRoom);
    let events = game.handle_action(&action)?;

    match events.iter().find_map(|event| match event {
        Event::RoomViewed(it) => Some(it),
        _ => None,
    }) {
        Some(room_viewed) => Ok(room_viewed.view.clone()),
        None => Err(GameError::General),
    }
}

pub async fn look_at_npc(
    transaction: &mut Transaction<'_, Postgres>,
    args: &NpcLookArgs,
) -> Result<NonPlayerView, GameError> {
    let state = match super::repository::by_id(transaction, &args.username, &args.game_id)
        .await
        .unwrap()
    {
        Some(it) => it,
        None => return Err(GameError::GameNotFound),
    };

    let player = match crate::player_characters::repository::current(transaction, &args.username)
        .await
        .unwrap()
    {
        Some(it) => it,
        None => return Err(GameError::NoPlayerCharacterSet),
    };

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

#[derive(Deserialize, Object, Serialize)]
pub struct FixtureLookArgs {
    pub username: String,
    pub game_id: String,
    pub fixture_id: String,
}

pub async fn look_at_fixture(
    transaction: &mut Transaction<'_, Postgres>,
    args: &FixtureLookArgs,
) -> Result<FixtureView, GameError> {
    let state = match super::repository::by_id(transaction, &args.username, &args.game_id)
        .await
        .unwrap()
    {
        Some(it) => it,
        None => return Err(GameError::GameNotFound),
    };

    let player = match crate::player_characters::repository::current(transaction, &args.username)
        .await
        .unwrap()
    {
        Some(it) => it,
        None => return Err(GameError::NoPlayerCharacterSet),
    };

    let mut game = Game { state, player };
    let look_args = LookAtFixture {
        fixture_id: args.fixture_id.clone(),
    };
    let action = Action::LookAtFixture(look_args);
    let events = game.handle_action(&action)?;

    match events.iter().find_map(|event| match event {
        Event::FixtureViewed(it) => Some(it),
        _ => None,
    }) {
        Some(fixture_viewed) => Ok(fixture_viewed.fixture_view.clone()),
        None => Err(GameError::General),
    }
}
