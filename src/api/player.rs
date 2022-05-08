use poem::Result;
use poem_openapi::{
    param::Path,
    payload::{Json, PlainText},
    ApiResponse, OpenApi,
};
use underworld_core::{components::player::PlayerCharacterView, systems::view::player};

use crate::{
    error::Error,
    player_characters::{
        current::{
            get_current_player_character, set_current_player_character, SetPlayerCharacterArgs,
        },
        generate::{generate_player_character, GeneratePlayerCharacter, GeneratedPlayerCharacter},
        get::{get_player_character, player_character_ids},
    },
    redis::get_redis_connection,
};

#[derive(ApiResponse)]
enum PlayerCharacterResponse {
    #[oai(status = 200)]
    PlayerCharacter(Json<PlayerCharacterView>),

    #[oai(status = 404)]
    NotFound(PlainText<String>),
}

#[derive(ApiResponse)]
enum PlayerCharacterIdsResponse {
    #[oai(status = 200)]
    PlayerCharacterIds(Json<Vec<String>>),
}

#[derive(ApiResponse)]
enum PlayerCharacterGeneratedResponse {
    #[oai(status = 201)]
    PlayerCharacterGenerated(Json<GeneratedPlayerCharacter>),
}

#[derive(ApiResponse)]
enum SetCurrentPlayerCharacterResponse {
    #[oai(status = 200)]
    PlayerCharacterSet(PlainText<String>),

    #[oai(status = 500)]
    BadRequest(Json<Error>),
}

pub struct UnderworldPlayerApi;

#[OpenApi]
impl UnderworldPlayerApi {
    /// Generate and save a new player_character for the user.
    #[oai(path = "/player_character/generate", method = "post")]
    async fn generate_player_character(
        &self,
        args: Json<GeneratePlayerCharacter>,
    ) -> Result<PlayerCharacterGeneratedResponse> {
        let mut connection = crate::redis::get_redis_connection().await;
        let result = generate_player_character(&mut connection, &args).await;

        Ok(PlayerCharacterGeneratedResponse::PlayerCharacterGenerated(
            Json(result),
        ))
    }

    /// Get IDs of all player characters
    ///
    /// # Example
    ///
    /// Call `/my_username/player_characters` to retrieve all player character
    /// ids for my_username
    #[oai(path = "/:username/player_characters", method = "get")]
    async fn list_player_characters(
        &self,
        username: Path<String>,
    ) -> Result<PlayerCharacterIdsResponse> {
        let mut connection = get_redis_connection().await;
        let result = player_character_ids(&mut connection, &username).await;

        Ok(PlayerCharacterIdsResponse::PlayerCharacterIds(Json(result)))
    }

    /// Check the player character for the user with specified ID.
    #[oai(path = "/:username/player_character/:id/check", method = "get")]
    async fn check_player_character(
        &self,
        username: Path<String>,
        id: Path<String>,
    ) -> Result<PlayerCharacterResponse> {
        let mut connection = get_redis_connection().await;
        let result = get_player_character(&mut connection, &username, &id).await;

        match result {
            Some(it) => Ok(PlayerCharacterResponse::PlayerCharacter(Json(
                player::check(it),
            ))),
            None => Ok(PlayerCharacterResponse::NotFound(PlainText(format!(
                "No player character found for user {} id {}",
                &username.0, &id.0
            )))),
        }
    }

    /// Check the status of the current player character.
    #[oai(path = "/:username/check_current_player_character", method = "get")]
    async fn check_current_player_character(
        &self,
        username: Path<String>,
    ) -> Result<PlayerCharacterResponse> {
        let mut connection = get_redis_connection().await;
        let player_character_result =
            get_current_player_character(&mut connection, &username).await;

        match player_character_result {
            Ok(it) => Ok(PlayerCharacterResponse::PlayerCharacter(Json(
                player::check(it),
            ))),
            Err(_) => Ok(PlayerCharacterResponse::NotFound(PlainText(
                "No character found".to_string(),
            ))),
        }
    }

    /// Set the specified player character as the current one for any actions in a game action.
    #[oai(path = "/set_current_player_character", method = "post")]
    async fn set_current_player_character(
        &self,
        args: Json<SetPlayerCharacterArgs>,
    ) -> Result<SetCurrentPlayerCharacterResponse> {
        let mut connection = get_redis_connection().await;
        let result = set_current_player_character(&mut connection, &args.0).await;

        match result {
            Ok(_) => Ok(SetCurrentPlayerCharacterResponse::PlayerCharacterSet(
                PlainText("Good to go".to_string()),
            )),
            Err(e) => Ok(SetCurrentPlayerCharacterResponse::BadRequest(Json(Error {
                message: format!("{}", e),
            }))),
        }
    }
}
