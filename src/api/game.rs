use poem::{web::Data, Result};
use poem_openapi::{param::Path, payload::Json, ApiResponse, OpenApi};
use serde_json::Value;
use sqlx::PgPool;
use underworld_core::components::games::game_state::GameStateView;

use crate::game::{
    generate::{generate_game, GeneratedGame},
    get::{game_ids, game_state},
};
use crate::tags::UnderworldApiTags;

use super::security::UnderworldApiKeyAuthorization;

#[derive(ApiResponse)]
pub enum GenerateGameResponse {
    #[oai(status = 201)]
    GameGenerated(Json<GeneratedGame>),
}

#[derive(ApiResponse)]
enum GameIdResponse {
    #[oai(status = 200)]
    GameIds(Json<Vec<String>>),
}

#[derive(ApiResponse)]
enum GameStateResponse {
    #[oai(status = 200)]
    GameState(Json<GameStateView>),
}

#[derive(ApiResponse)]
enum RawGameStateResponse {
    #[oai(status = 200)]
    GameState(Json<Value>),
}

#[derive(ApiResponse)]
enum UnlockResponse {
    #[oai(status = 201)]
    Success
}

pub struct UnderworldGameApi;

#[OpenApi(tag = "UnderworldApiTags::Games", prefix_path = "/games")]
impl UnderworldGameApi {
    /// Generate and persist a new game.
    ///
    /// # Example
    ///
    /// POST `/games/generate` to generate and save a new game
    #[oai(path = "/generate", method = "post", operation_id = "generate_game")]
    async fn generate_game(
        &self,
        pool: Data<&PgPool>,
        auth: UnderworldApiKeyAuthorization,
    ) -> Result<GenerateGameResponse> {
        let mut transaction = pool.0.begin().await.unwrap();
        let generated_result = generate_game(&mut transaction, &auth.0.email).await?;
        transaction.commit().await.unwrap();

        Ok(GenerateGameResponse::GameGenerated(Json(generated_result)))
    }

    /// Get IDs of all current games
    ///
    /// # Example
    ///
    /// Call `/games/ids` to retrieve all of you game ids.
    #[oai(path = "/ids", method = "get", operation_id = "get_game_ids")]
    async fn list_games(
        &self,
        pool: Data<&PgPool>,
        auth: UnderworldApiKeyAuthorization,
    ) -> Result<GameIdResponse> {
        let mut transaction = pool.0.begin().await.unwrap();
        let result = game_ids(&mut transaction, &auth.0.email).await;
        transaction.commit().await.unwrap();
        Ok(GameIdResponse::GameIds(Json(result)))
    }

    /// Get the current state of the game. Will return some inner state and views
    /// of the rooms based on the knowledge gained from all players from the game.
    #[oai(path = "/:game_id/state", method = "get", operation_id = "game_state")]
    async fn game_state(
        &self,
        pool: Data<&PgPool>,
        auth: UnderworldApiKeyAuthorization,
        game_id: Path<String>,
    ) -> Result<GameStateResponse> {
        let mut transaction = pool.0.begin().await.unwrap();
        let view = game_state(&mut transaction, &auth.0.email, &game_id).await?;
        Ok(GameStateResponse::GameState(Json(view)))
    }

    /// Unlock all of the knowledge in the game for all player characters.
    #[oai(path = "/:game_id/unlock_knowledge", method = "post", operation_id = "unlock_knowledge")]
    async fn unlock_knowledge(
        &self,
        pool: Data<&PgPool>,
        auth: UnderworldApiKeyAuthorization,
        game_id: Path<String>,
    ) -> Result<UnlockResponse> {
        let mut transaction = pool.0.begin().await.unwrap();
        crate::game::unlock::unlock_knowledge(&mut transaction, &auth.0.email, &game_id).await?;
        transaction.commit().await.unwrap();
        Ok(UnlockResponse::Success)
    }

    /// Get the current state of the game. This is a raw export and the inner structure is
    /// intentionally not documented in the Open API. It matches the structure of the GameState
    /// struct inside of the `underworld_core` repository. However, since this is an internal
    /// structure to the game, it should not be relied on for any use. The documented one
    /// returned in `/state` is less likely to change drastically.
    #[oai(
        path = "/:game_id/raw_export",
        method = "get",
        operation_id = "raw_export"
    )]
    async fn raw_export(
        &self,
        pool: Data<&PgPool>,
        auth: UnderworldApiKeyAuthorization,
        game_id: Path<String>,
    ) -> Result<RawGameStateResponse> {
        let mut transaction = pool.0.begin().await.unwrap();
        let value = crate::game::get::raw_export(&mut transaction, &auth.0.email, &game_id).await?;
        Ok(RawGameStateResponse::GameState(Json(value)))
    }
}
