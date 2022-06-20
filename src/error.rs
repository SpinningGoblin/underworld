use poem::error::ResponseError;

#[derive(Debug, thiserror::Error, strum_macros::Display)]
pub enum GameError {
    ExitNotFoundError(String),
    FixtureCannotBeFound(String),
    FixtureCannotBeOpened(String),
    FixtureHasNoHiddenCompartment(String),
    FixtureHasHiddenCompartmentUnknown(String),
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
            underworld_core::errors::Error::FixtureCannotBeFound(it) => {
                GameError::FixtureCannotBeFound(it)
            }
            underworld_core::errors::Error::FixtureCannotBeOpened(it) => {
                GameError::FixtureCannotBeOpened(it)
            }
            underworld_core::errors::Error::FixtureHasNoHiddenCompartment(it) => {
                GameError::FixtureHasNoHiddenCompartment(it)
            }
            underworld_core::errors::Error::FixtureHasHiddenCompartmentUnknown(it) => {
                GameError::FixtureHasHiddenCompartmentUnknown(it)
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
            GameError::FixtureCannotBeFound(_) => poem::http::StatusCode::BAD_REQUEST,
            GameError::FixtureCannotBeOpened(_) => poem::http::StatusCode::BAD_REQUEST,
            GameError::FixtureHasNoHiddenCompartment(_) => poem::http::StatusCode::BAD_REQUEST,
            GameError::FixtureHasHiddenCompartmentUnknown(_) => poem::http::StatusCode::BAD_REQUEST,
        }
    }
}
