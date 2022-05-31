use poem::{web::Data, Result};
use poem_openapi::{
    param::{Header, Path},
    payload::{Json, PlainText},
    ApiResponse, OpenApi,
};
use sqlx::PgPool;
use underworld_core::{components::player::PlayerCharacterView, systems::view::player};

use crate::player_characters::{
    current::{get_current_player_character, set_current_player_character},
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

#[OpenApi(tag = "UnderworldApiTags::PlayerCharacters", prefix_path = "/pcs/")]
impl UnderworldPlayerApi {
    /// Generate and save a new player_character for the user.
    /// If user has no player set as current, this one gets set as the current.
    #[oai(path = "/generate", method = "post", operation_id = "generate_pc")]
    async fn generate_player_character(
        &self,
        pool: Data<&PgPool>,
        #[oai(name = "underworld-username")] username: Header<String>,
        args: Json<GeneratePlayerCharacter>,
    ) -> Result<PlayerCharacterGeneratedResponse> {
        let mut transaction = pool.0.begin().await.unwrap();
        let result = generate_player_character(&mut transaction, &username, &args).await;
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
    #[oai(path = "/ids", method = "get", operation_id = "get_pc_ids")]
    async fn list_player_characters(
        &self,
        pool: Data<&PgPool>,
        #[oai(name = "underworld-username")] username: Header<String>,
    ) -> Result<PlayerCharacterIdsResponse> {
        let mut transaction = pool.0.begin().await.unwrap();
        let result = player_character_ids(&mut transaction, &username).await;
        transaction.commit().await.unwrap();
        Ok(PlayerCharacterIdsResponse::PlayerCharacterIds(Json(result)))
    }

    /// Check the player character for the user with specified ID.
    #[oai(path = "/:id/check", method = "get", operation_id = "get_pc")]
    async fn check_player_character(
        &self,
        pool: Data<&PgPool>,
        #[oai(name = "underworld-username")] username: Header<String>,
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
    #[oai(path = "/current", method = "get", operation_id = "get_current_pc")]
    async fn check_current_player_character(
        &self,
        pool: Data<&PgPool>,
        #[oai(name = "underworld-username")] username: Header<String>,
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
        path = "/:id/set_as_current",
        method = "post",
        operation_id = "set_pc_as_current"
    )]
    async fn set_current_player_character(
        &self,
        pool: Data<&PgPool>,
        #[oai(name = "underworld-username")] username: Header<String>,
        id: Path<String>,
    ) -> Result<SetCurrentPlayerCharacterResponse> {
        let mut transaction = pool.0.begin().await.unwrap();
        set_current_player_character(&mut transaction, &username, &id)
            .await
            .unwrap();
        transaction.commit().await.unwrap();
        Ok(SetCurrentPlayerCharacterResponse::PlayerCharacterSet(
            PlainText("Good to go".to_string()),
        ))
    }
}
