use std::fmt::Display;

use poem_openapi::Enum;
use serde::Serialize;

#[derive(Enum, Serialize)]
pub enum GameError {
    General,
    NoPlayerCharacterSet,
    UnknownPlayerCharacter,
    GameNotFound,
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
