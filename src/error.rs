use poem::error::ResponseError;

#[derive(Debug, thiserror::Error, strum_macros::Display)]
pub enum GameError {
    ExitNotFoundError(String),
    FixtureNotFoundError(String),
    InvalidIdError(String),
    ItemNotDirectlyUsableError(String),
    ItemNotFoundError(String),
    NpcNotFoundError(String),
    PlayerIsDeadError,
    SpellNotFoundError(String),
    TooManyWeaponsEquippedError,
    TooManyWearablesEquippedError,
    GeneralError(String),
    NoPlayerCharacterSetError,
    UnknownPlayerCharacterError,
    GameNotFoundError,
}

impl From<underworld_core::errors::Error> for GameError {
    fn from(error: underworld_core::errors::Error) -> Self {
        match error {
            underworld_core::errors::Error::ExitNotFoundError(it) => {
                GameError::ExitNotFoundError(it)
            }
            underworld_core::errors::Error::FixtureNotFoundError(it) => {
                GameError::FixtureNotFoundError(it)
            }
            underworld_core::errors::Error::InvalidIdError(it) => GameError::InvalidIdError(it),
            underworld_core::errors::Error::ItemNotDirectlyUsableError(it) => {
                GameError::ItemNotDirectlyUsableError(it)
            }
            underworld_core::errors::Error::ItemNotFoundError(it) => {
                GameError::ItemNotFoundError(it)
            }
            underworld_core::errors::Error::NpcNotFoundError(it) => GameError::NpcNotFoundError(it),
            underworld_core::errors::Error::PlayerIsDeadError => GameError::PlayerIsDeadError,
            underworld_core::errors::Error::SpellNotFoundError(it) => {
                GameError::SpellNotFoundError(it)
            }
            underworld_core::errors::Error::TooManyWeaponsEquippedError => {
                GameError::TooManyWeaponsEquippedError
            }
            underworld_core::errors::Error::TooManyWearablesEquippedError => {
                GameError::TooManyWearablesEquippedError
            }
        }
    }
}

impl ResponseError for GameError {
    fn status(&self) -> poem::http::StatusCode {
        match self {
            GameError::ExitNotFoundError(_) => poem::http::StatusCode::BAD_REQUEST,
            GameError::FixtureNotFoundError(_) => poem::http::StatusCode::BAD_REQUEST,
            GameError::InvalidIdError(_) => poem::http::StatusCode::BAD_REQUEST,
            GameError::ItemNotDirectlyUsableError(_) => poem::http::StatusCode::BAD_REQUEST,
            GameError::ItemNotFoundError(_) => poem::http::StatusCode::BAD_REQUEST,
            GameError::NpcNotFoundError(_) => poem::http::StatusCode::BAD_REQUEST,
            GameError::PlayerIsDeadError => poem::http::StatusCode::BAD_REQUEST,
            GameError::SpellNotFoundError(_) => poem::http::StatusCode::BAD_REQUEST,
            GameError::TooManyWeaponsEquippedError => poem::http::StatusCode::BAD_REQUEST,
            GameError::TooManyWearablesEquippedError => poem::http::StatusCode::BAD_REQUEST,
            GameError::GeneralError(_) => poem::http::StatusCode::BAD_REQUEST,
            GameError::NoPlayerCharacterSetError => poem::http::StatusCode::BAD_REQUEST,
            GameError::UnknownPlayerCharacterError => poem::http::StatusCode::NOT_FOUND,
            GameError::GameNotFoundError => poem::http::StatusCode::NOT_FOUND,
        }
    }
}
