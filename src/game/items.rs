use poem_openapi::Object;
use serde::Serialize;
use sqlx::{Postgres, Transaction};
use underworld_core::{
    actions::{action::Action, MovePlayerItem, SellPlayerItem, UseItemOnPlayer},
    game::Game,
};

use crate::{
    actions::{game_actions, PerformAction},
    error::GameError,
    event::GameEvent,
};

#[derive(Serialize, Object)]
/// Results from attack on the NPC.
pub struct ItemUsed {
    /// Events that happened due to the attack.
    events: Vec<GameEvent>,
    /// Actions that can now be performed after the attack.
    actions: Vec<PerformAction>,
}

pub async fn use_item_on_player(
    transaction: &mut Transaction<'_, Postgres>,
    username: &str,
    game_id: &str,
    args: &UseItemOnPlayer,
) -> Result<ItemUsed, GameError> {
    let player_character =
        match crate::player_characters::repository::current(transaction, username).await? {
            Some(it) => it,
            None => return Err(GameError::NoPlayerCharacterSetError),
        };

    let state = match super::repository::by_id(transaction, username, game_id).await? {
        Some(it) => it,
        None => return Err(GameError::GameNotFoundError),
    };

    let mut game = Game {
        player: player_character,
        state,
    };

    let events = game.handle_action(&Action::UseItemOnPlayer(args.to_owned()))?;
    super::repository::save(transaction, username, &game.state).await?;
    crate::player_characters::repository::save(transaction, username, &game.player).await?;

    let game_events: Vec<GameEvent> = events.into_iter().map(GameEvent::from).collect();

    Ok(ItemUsed {
        events: game_events,
        actions: game_actions(&game, username),
    })
}

#[derive(Serialize, Object)]
/// Results from attack on the NPC.
pub struct ItemMoved {
    /// Events that happened due to the attack.
    events: Vec<GameEvent>,
    /// Actions that can now be performed after the attack.
    actions: Vec<PerformAction>,
}

pub async fn move_player_item(
    transaction: &mut Transaction<'_, Postgres>,
    username: &str,
    game_id: &str,
    args: &MovePlayerItem,
) -> Result<ItemMoved, GameError> {
    let player_character =
        match crate::player_characters::repository::current(transaction, username).await? {
            Some(it) => it,
            None => return Err(GameError::NoPlayerCharacterSetError),
        };

    let state = match super::repository::by_id(transaction, username, game_id).await? {
        Some(it) => it,
        None => return Err(GameError::GameNotFoundError),
    };

    let mut game = Game {
        player: player_character,
        state,
    };

    let events = game.handle_action(&Action::MovePlayerItem(args.to_owned()))?;
    super::repository::save(transaction, username, &game.state).await?;
    crate::player_characters::repository::save(transaction, username, &game.player).await?;

    let game_events: Vec<GameEvent> = events.into_iter().map(GameEvent::from).collect();

    Ok(ItemMoved {
        events: game_events,
        actions: game_actions(&game, username),
    })
}

#[derive(Serialize, Object)]
/// Results from attack on the NPC.
pub struct ItemSold {
    /// Events that happened due to the attack.
    events: Vec<GameEvent>,
    /// Actions that can now be performed after the attack.
    actions: Vec<PerformAction>,
}

pub async fn sell_player_item(
    transaction: &mut Transaction<'_, Postgres>,
    username: &str,
    game_id: &str,
    args: &SellPlayerItem,
) -> Result<ItemSold, GameError> {
    let player_character =
        match crate::player_characters::repository::current(transaction, username).await? {
            Some(it) => it,
            None => return Err(GameError::NoPlayerCharacterSetError),
        };

    let state = match super::repository::by_id(transaction, username, game_id).await? {
        Some(it) => it,
        None => return Err(GameError::GameNotFoundError),
    };

    let mut game = Game {
        player: player_character,
        state,
    };

    let events = game.handle_action(&Action::SellPlayerItem(args.to_owned()))?;
    super::repository::save(transaction, username, &game.state).await?;
    crate::player_characters::repository::save(transaction, username, &game.player).await?;

    let game_events: Vec<GameEvent> = events.into_iter().map(GameEvent::from).collect();

    Ok(ItemSold {
        events: game_events,
        actions: game_actions(&game, username),
    })
}
