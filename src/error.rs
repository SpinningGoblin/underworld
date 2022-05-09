use std::fmt::Display;

use poem_openapi::{Enum, Object};
use serde::Serialize;
use underworld_core::errors::Errors;

#[derive(Object, Serialize, Debug)]
pub struct Error {
    pub message: String,
}

#[derive(Enum, Serialize)]
pub enum GameError {
    General,
    NoPlayerCharacterSet,
    UnknownPlayerCharacter,
    GameNotFound,
}

impl From<Errors> for GameError {
    fn from(_: Errors) -> Self {
        GameError::General
    }
}

impl Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match *self {
            GameError::General => "general_error",
            GameError::NoPlayerCharacterSet => "no_player_character_set",
            GameError::UnknownPlayerCharacter => "unknown_player_character_specified",
            GameError::GameNotFound => "game_not_found",
        };

        write!(f, "{}", text)
    }
}
