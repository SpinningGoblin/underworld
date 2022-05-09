use sqlx::{Postgres, Transaction};
use underworld_core::components::player::PlayerCharacter;

pub async fn get_player_character(
    transaction: &mut Transaction<'_, Postgres>,
    username: &str,
    id: &str,
) -> Option<PlayerCharacter> {
    super::repository::by_id(transaction, username, id)
        .await
        .unwrap()
}

pub async fn player_character_ids(
    transaction: &mut Transaction<'_, Postgres>,
    username: &str,
) -> Vec<String> {
    super::repository::ids(transaction, username)
        .await
        .unwrap()
}
