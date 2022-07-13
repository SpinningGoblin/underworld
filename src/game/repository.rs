use serde_json::Value;
use sqlx::{postgres::PgRow, Postgres, Row, Transaction};
use underworld_core::components::games::GameState;

use crate::error::GameError;

pub async fn by_id(
    transaction: &mut Transaction<'_, Postgres>,
    username: &str,
    game_state_id: &str,
) -> Result<Option<GameState>, GameError> {
    let query = r#"
        select game_state from game_states
        where username = $1 and game_state_id = $2
    "#;

    let row: Option<(Value,)> = sqlx::query_as(query)
        .bind(&username)
        .bind(&game_state_id)
        .fetch_optional(transaction)
        .await
        .unwrap();

    match row {
        Some(value) => {
            let game_state: GameState = serde_json::from_value(value.0).unwrap();
            Ok(Some(game_state))
        }
        None => Ok(None),
    }
}

pub async fn save(
    transaction: &mut Transaction<'_, Postgres>,
    username: &str,
    game_state: &GameState,
) -> Result<(), GameError> {
    let query = r#"
        insert into game_states (username, game_state_id, game_state)
        values ($1, $2, $3)
        on conflict (game_state_id)
        do
            update set game_state = $3
    "#;
    let serialized = serde_json::to_value(&game_state).unwrap();
    let game_state_id = game_state.id.to_string();

    sqlx::query(query)
        .bind(&username)
        .bind(&game_state_id)
        .bind(&serialized)
        .execute(transaction)
        .await
        .unwrap();

    Ok(())
}

pub async fn ids(
    transaction: &mut Transaction<'_, Postgres>,
    username: &str,
) -> Result<Vec<String>, GameError> {
    let rows: Vec<String> =
        sqlx::query("select game_state_id from game_states where username = $1")
            .bind(&username)
            .map(|row: PgRow| row.try_get("game_state_id").unwrap())
            .fetch_all(transaction)
            .await
            .unwrap();

    Ok(rows)
}
