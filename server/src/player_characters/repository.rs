use serde_json::Value;
use sqlx::{postgres::PgRow, Postgres, Row, Transaction};
use underworld_core::components::PlayerCharacter;

use crate::error::GameError;

pub async fn ids(
    transaction: &mut Transaction<'_, Postgres>,
    username: &str,
) -> Result<Vec<String>, GameError> {
    let rows: Vec<String> = sqlx::query("select pc_id from player_characters where username = $1")
        .bind(username)
        .map(|row: PgRow| row.try_get("pc_id").unwrap())
        .fetch_all(transaction)
        .await
        .unwrap();

    Ok(rows)
}

pub async fn save(
    transaction: &mut Transaction<'_, Postgres>,
    username: &str,
    player_character: &PlayerCharacter,
) -> Result<(), GameError> {
    let pc_id = player_character.id.to_string();
    let serialized = serde_json::to_value(player_character).unwrap();

    let query = r#"
        insert into player_characters (username, pc_id, pc) values ($1, $2, $3)
        on conflict (pc_id)
        do
        update set pc = $3
    "#;

    sqlx::query(query)
        .bind(username)
        .bind(&pc_id)
        .bind(&serialized)
        .execute(transaction)
        .await
        .unwrap();

    Ok(())
}

pub async fn by_id(
    transaction: &mut Transaction<'_, Postgres>,
    username: &str,
    pc_id: &str,
) -> Result<Option<PlayerCharacter>, GameError> {
    let row: (Value,) =
        sqlx::query_as("select pc from player_characters where username = $1 and pc_id = $2")
            .bind(username)
            .bind(pc_id)
            .fetch_one(transaction)
            .await
            .unwrap();

    let player_character: PlayerCharacter = serde_json::from_value(row.0).unwrap();
    Ok(Some(player_character))
}

pub async fn current(
    transaction: &mut Transaction<'_, Postgres>,
    username: &str,
) -> Result<Option<PlayerCharacter>, GameError> {
    let row: Option<(String,)> =
        sqlx::query_as("select pc_id from current_player_characters where username = $1")
            .bind(username)
            .fetch_optional(&mut *transaction)
            .await
            .unwrap();

    match row {
        Some(pc_id) => by_id(transaction, username, &pc_id.0).await,
        None => Ok(None),
    }
}

pub async fn set_current(
    transaction: &mut Transaction<'_, Postgres>,
    username: &str,
    player_character: &PlayerCharacter,
) -> Result<(), GameError> {
    let pc_id = player_character.id.to_string();
    let query = r#"
        insert into current_player_characters (username, pc_id)
        values ($1, $2)
        on conflict (username)
        do
        update set pc_id = $2
    "#;
    sqlx::query(query)
        .bind(username)
        .bind(&pc_id)
        .execute(transaction)
        .await
        .unwrap();

    Ok(())
}
