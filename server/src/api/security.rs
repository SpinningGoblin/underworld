use poem::Request;
use poem_openapi::{auth::ApiKey, SecurityScheme};
use sqlx::PgPool;

use crate::auth::{repository::valid_api_token, User};

/// ApiKey authorization
#[derive(SecurityScheme)]
#[oai(
    ty = "api_key",
    key_name = "UNDERWORLD-TOKEN",
    key_in = "header",
    checker = "api_checker"
)]
pub struct UnderworldApiKeyAuthorization(pub User);

async fn api_checker(req: &Request, api_key: ApiKey) -> Option<User> {
    let pool_option: Option<&PgPool> = req.data();
    let pool = pool_option.unwrap();

    match valid_api_token(pool, &api_key.key).await {
        Ok(it) => Some(it),
        Err(_) => None,
    }
}
