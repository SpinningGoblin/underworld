use poem::{web::Data, Result};
use poem_openapi::{payload::Json, ApiResponse, OpenApi};
use sqlx::PgPool;

use crate::game::{
    generate::{generate_game, GeneratedGame},
    get::game_ids,
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

pub struct UnderworldGameApi;

#[OpenApi(tag = "UnderworldApiTags::Games", prefix_path = "/games")]
impl UnderworldGameApi {
    /// Generate and persist a new game.
    ///
    /// # Example
    ///
    /// POST `/my_username/games/generate` to generate and save
    /// a new game for my_username
    #[oai(path = "/generate", method = "post", operation_id = "generate_game")]
    async fn generate_game(
        &self,
        pool: Data<&PgPool>,
        auth: UnderworldApiKeyAuthorization,
    ) -> Result<GenerateGameResponse> {
        let mut transaction = pool.0.begin().await.unwrap();
        let generated_result = generate_game(&mut transaction, &auth.0.username).await?;
        transaction.commit().await.unwrap();

        Ok(GenerateGameResponse::GameGenerated(Json(generated_result)))
    }

    /// Get IDs of all current games
    ///
    /// # Example
    ///
    /// Call `/my_username/games/ids` to retrieve all game ids for my_username
    #[oai(path = "/ids", method = "get", operation_id = "get_game_ids")]
    async fn list_games(
        &self,
        pool: Data<&PgPool>,
        auth: UnderworldApiKeyAuthorization,
    ) -> Result<GameIdResponse> {
        let mut transaction = pool.0.begin().await.unwrap();
        let result = game_ids(&mut transaction, &auth.0.username).await;
        transaction.commit().await.unwrap();
        Ok(GameIdResponse::GameIds(Json(result)))
    }
}
