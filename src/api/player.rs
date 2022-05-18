use poem::{web::Data, Result};
use poem_openapi::{
    param::Path,
    payload::{Json, PlainText},
    ApiResponse, OpenApi,
};
use sqlx::PgPool;
use underworld_core::{components::player::PlayerCharacterView, systems::view::player};

use crate::player_characters::{
    current::{get_current_player_character, set_current_player_character, SetPlayerCharacterArgs},
    generate::{generate_player_character, GeneratePlayerCharacter, GeneratedPlayerCharacter},
    get::{get_player_character, player_character_ids},
};
use crate::tags::UnderworldApiTags;

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
}

pub struct UnderworldPlayerApi;

#[OpenApi]
impl UnderworldPlayerApi {
    /// Generate and save a new player_character for the user.
    /// If user has no player set as current, this one gets set as the current.
    #[oai(
        path = "/player_character/generate",
        method = "post",
        tag = "UnderworldApiTags::PlayerCharacter"
    )]
    async fn generate_player_character(
        &self,
        pool: Data<&PgPool>,
        args: Json<GeneratePlayerCharacter>,
    ) -> Result<PlayerCharacterGeneratedResponse> {
        let mut transaction = pool.0.begin().await.unwrap();
        let result = generate_player_character(&mut transaction, &args).await;
        transaction.commit().await.unwrap();
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
    #[oai(
        path = "/:username/player_characters",
        method = "get",
        tag = "UnderworldApiTags::PlayerCharacter"
    )]
    async fn list_player_characters(
        &self,
        pool: Data<&PgPool>,
        username: Path<String>,
    ) -> Result<PlayerCharacterIdsResponse> {
        let mut transaction = pool.0.begin().await.unwrap();
        let result = player_character_ids(&mut transaction, &username).await;
        transaction.commit().await.unwrap();
        Ok(PlayerCharacterIdsResponse::PlayerCharacterIds(Json(result)))
    }

    /// Check the player character for the user with specified ID.
    #[oai(
        path = "/:username/player_character/:id/check",
        method = "get",
        tag = "UnderworldApiTags::PlayerCharacter"
    )]
    async fn check_player_character(
        &self,
        pool: Data<&PgPool>,
        username: Path<String>,
        id: Path<String>,
    ) -> Result<PlayerCharacterResponse> {
        let mut transaction = pool.0.begin().await.unwrap();
        let result = get_player_character(&mut transaction, &username, &id).await;
        transaction.commit().await.unwrap();

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
    #[oai(
        path = "/:username/check_current_player_character",
        method = "get",
        tag = "UnderworldApiTags::PlayerCharacter"
    )]
    async fn check_current_player_character(
        &self,
        pool: Data<&PgPool>,
        username: Path<String>,
    ) -> Result<PlayerCharacterResponse> {
        let mut transaction = pool.0.begin().await.unwrap();
        let player_character_result = get_current_player_character(&mut transaction, &username)
            .await
            .unwrap();
        transaction.commit().await.unwrap();
        Ok(PlayerCharacterResponse::PlayerCharacter(Json(
            player::check(player_character_result),
        )))
    }

    /// Set the specified player character as the current one for any actions in a game.
    #[oai(
        path = "/set_current_player_character",
        method = "post",
        tag = "UnderworldApiTags::PlayerCharacter"
    )]
    async fn set_current_player_character(
        &self,
        pool: Data<&PgPool>,
        args: Json<SetPlayerCharacterArgs>,
    ) -> Result<SetCurrentPlayerCharacterResponse> {
        let mut transaction = pool.0.begin().await.unwrap();
        set_current_player_character(&mut transaction, &args.0)
            .await
            .unwrap();
        transaction.commit().await.unwrap();
        Ok(SetCurrentPlayerCharacterResponse::PlayerCharacterSet(
            PlainText("Good to go".to_string()),
        ))
    }
}
