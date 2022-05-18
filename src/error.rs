use std::{error::Error, fmt::Display};

use poem::error::ResponseError;

#[derive(Debug)]
pub struct GeneralError(pub String);

impl Display for GeneralError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GeneralError:{}", self.0)
    }
}

impl Error for GeneralError {}

impl ResponseError for GeneralError {
    fn status(&self) -> poem::http::StatusCode {
        poem::http::StatusCode::BAD_REQUEST
    }
}

#[derive(Debug)]
pub struct NoPlayerCharacterSetError;

impl Display for NoPlayerCharacterSetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NoPlayerCharacterSet")
    }
}

impl Error for NoPlayerCharacterSetError {}

impl ResponseError for NoPlayerCharacterSetError {
    fn status(&self) -> poem::http::StatusCode {
        poem::http::StatusCode::BAD_REQUEST
    }
}

#[derive(Debug)]
pub struct UnknownPlayerCharacterError;

impl Display for UnknownPlayerCharacterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UnknownPlayerCharacter")
    }
}

impl Error for UnknownPlayerCharacterError {}

impl ResponseError for UnknownPlayerCharacterError {
    fn status(&self) -> poem::http::StatusCode {
        poem::http::StatusCode::NOT_FOUND
    }
}

#[derive(Debug)]
pub struct GameNotFoundError;

impl Display for GameNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GameNotFound")
    }
}

impl Error for GameNotFoundError {}

impl ResponseError for GameNotFoundError {
    fn status(&self) -> poem::http::StatusCode {
        poem::http::StatusCode::NOT_FOUND
    }
}
