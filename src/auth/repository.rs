use chrono::{Duration, Utc};
use sqlx::{postgres::PgRow, PgPool, Postgres, Row, Transaction};
use uuid::Uuid;

use crate::error::AuthError;

use super::User;

pub struct UserDetails {
    pub email: String,
}

async fn has_mail_token(transaction: &mut Transaction<'_, Postgres>, email: &str) -> bool {
    let token: Option<String> =
        sqlx::query("select token from mail_tokens where email = $1 and deleted_after >= $2")
            .bind(email)
            .bind(Utc::now())
            .map(|row: PgRow| row.get("token"))
            .fetch_optional(transaction)
            .await
            .unwrap();
    match token {
        Some(_) => true,
        None => false,
    }
}

pub async fn delete_dead_mail_tokens(
    transaction: &mut Transaction<'_, Postgres>,
) -> Result<(), AuthError> {
    match sqlx::query("delete from mail_tokens where deleted_after < $1")
        .bind(Utc::now())
        .execute(transaction)
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(AuthError::GeneralError(e.to_string())),
    }
}

pub async fn fetch_details_from_mail_token(
    transaction: &mut Transaction<'_, Postgres>,
    token: &str,
) -> Result<Option<UserDetails>, AuthError> {
    let result = sqlx::query("select email from mail_tokens where token = $1")
        .bind(&token)
        .map(|row: PgRow| UserDetails {
            email: row.get("email"),
        })
        .fetch_optional(transaction)
        .await
        .unwrap();

    match result {
        Some(it) => Ok(Some(it)),
        None => Ok(None),
    }
}

pub async fn valid_api_token(pool: &PgPool, token: &str) -> Result<User, AuthError> {
    let select_token_query =
        "select email from api_tokens where token = $1 and deleted_after >= $2";
    let result: Option<User> = sqlx::query(select_token_query)
        .bind(&token)
        .bind(Utc::now())
        .map(|row: PgRow| User {
            email: row.get("email"),
        })
        .fetch_optional(pool)
        .await
        .unwrap();

    match result {
        Some(it) => Ok(it),
        None => Err(AuthError::InvalidToken),
    }
}

async fn try_get_api_token(
    transaction: &mut Transaction<'_, Postgres>,
    user_details: &UserDetails,
) -> Result<Option<String>, AuthError> {
    let select_token_query =
        "select token from api_tokens where email = $1 and deleted_after >= $2";
    let result: Option<String> = sqlx::query(select_token_query)
        .bind(&user_details.email)
        .bind(Utc::now())
        .map(|row: PgRow| row.get("token"))
        .fetch_optional(transaction)
        .await
        .unwrap();

    Ok(result)
}

async fn insert_new_token(
    transaction: &mut Transaction<'_, Postgres>,
    user_details: &UserDetails,
) -> Result<String, AuthError> {
    let token = Uuid::new_v4().to_string();
    let deleted_after = Utc::now() + Duration::days(90);
    sqlx::query(
        "insert into api_tokens (email, token, deleted_after) values ($1, $2, $3)",
    )
    .bind(&user_details.email)
    .bind(&token)
    .bind(&deleted_after)
    .execute(transaction)
    .await
    .unwrap();
    Ok(token)
}

pub async fn get_api_token(
    transaction: &mut Transaction<'_, Postgres>,
    user_details: &UserDetails,
) -> Result<String, AuthError> {
    let result = try_get_api_token(transaction, user_details).await.unwrap();

    match result {
        Some(it) => Ok(it),
        None => insert_new_token(transaction, user_details).await,
    }
}

pub async fn get_mail_token(
    transaction: &mut Transaction<'_, Postgres>,
    user_details: &UserDetails,
) -> Result<Option<String>, AuthError> {
    if has_mail_token(transaction, &user_details.email).await {
        return Ok(None);
    }

    let token = Uuid::new_v4().to_string();
    let created_at = Utc::now();
    let deleted_after = created_at + Duration::minutes(30);
    let query =
        "insert into mail_tokens (email, token, created_at, deleted_after) values ($1, $2, $3, $4)";
    sqlx::query(query)
        .bind(&user_details.email)
        .bind(&token)
        .bind(&created_at)
        .bind(&deleted_after)
        .execute(transaction)
        .await
        .unwrap();

    Ok(Some(token))
}
