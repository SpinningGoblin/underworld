use sqlx::{Postgres, Transaction};

pub async fn game_ids(transaction: &mut Transaction<'_, Postgres>, username: &str) -> Vec<String> {
    super::repository::ids(transaction, username).await.unwrap()
}
