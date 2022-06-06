use std::error::Error;

use sqlx::{Postgres, Transaction};
use underworld_core::{
    actions::{
        action::Action, look_at_fixture::LookAtFixture,
        look_at_npc::LookAtNpc,
    },
    components::{
        fixtures::fixture::FixtureView, non_player::NonPlayerView, rooms::room_view::RoomView,
    },
    events::event::Event,
    game::Game,
};

use crate::error::{GameNotFoundError, GeneralError, NoPlayerCharacterSetError};

pub async fn look_at_room(
    transaction: &mut Transaction<'_, Postgres>,
    username: &str,
    game_id: &str,
) -> Result<RoomView, Box<dyn Error>> {
    let state = match super::repository::by_id(transaction, &username, &game_id).await? {
        Some(it) => it,
        None => return Err(Box::new(GameNotFoundError)),
    };

    Ok(state.view_current_room())
}

pub async fn look_at_npc(
    transaction: &mut Transaction<'_, Postgres>,
    username: &str,
    game_id: &str,
    args: &LookAtNpc,
) -> Result<NonPlayerView, Box<dyn Error>> {
    let state = match super::repository::by_id(transaction, &username, &game_id)
        .await
        .unwrap()
    {
        Some(it) => it,
        None => return Err(Box::new(GameNotFoundError)),
    };

    let player = match crate::player_characters::repository::current(transaction, &username)
        .await
        .unwrap()
    {
        Some(it) => it,
        None => return Err(Box::new(NoPlayerCharacterSetError)),
    };

    let mut game = Game { state, player };
    let action = Action::LookAtNpc(args.to_owned());
    let events = game.handle_action(&action)?;

    match events.iter().find_map(|event| match event {
        Event::NpcViewed(it) => Some(it),
        _ => None,
    }) {
        Some(npc_viewed) => Ok(npc_viewed.npc_view.clone()),
        None => Err(Box::new(GeneralError("look_at_npc_failed".to_string()))),
    }
}

pub async fn look_at_fixture(
    transaction: &mut Transaction<'_, Postgres>,
    username: &str,
    game_id: &str,
    args: &LookAtFixture,
) -> Result<FixtureView, Box<dyn Error>> {
    let state = match super::repository::by_id(transaction, &username, &game_id)
        .await
        .unwrap()
    {
        Some(it) => it,
        None => return Err(Box::new(GameNotFoundError)),
    };

    let player = match crate::player_characters::repository::current(transaction, &username)
        .await
        .unwrap()
    {
        Some(it) => it,
        None => return Err(Box::new(NoPlayerCharacterSetError)),
    };

    let mut game = Game { state, player };
    let action = Action::LookAtFixture(args.to_owned());
    let events = game.handle_action(&action)?;

    match events.iter().find_map(|event| match event {
        Event::FixtureViewed(it) => Some(it),
        _ => None,
    }) {
        Some(fixture_viewed) => Ok(fixture_viewed.fixture_view.clone()),
        None => Err(Box::new(GeneralError("look_at_fixture_failed".to_string()))),
    }
}
