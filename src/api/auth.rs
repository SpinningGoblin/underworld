use std::{collections::HashMap, env};

use poem::{http::StatusCode, web::Data, Result};
use poem_openapi::{
    param::Query,
    payload::{Form, PlainText},
    OpenApi,
};
use sqlx::PgPool;

use crate::{auth::repository::UserDetails, config::get_server_auth_url, tags::UnderworldApiTags};

pub struct UnderworldAuthApi;

fn frontend_url(api_token: &str) -> (String, String) {
    match env::var("FRONTEND_URL") {
        Ok(it) => (format!("{}#{}", it, api_token), "SameSite=None".to_string()),
        Err(_) => ("/".to_string(), "SameSite=Lax".to_string()),
    }
}

#[OpenApi(tag = "UnderworldApiTags::Auth")]
impl UnderworldAuthApi {
    /// Login with a username and email. Email will be sent with link to finish login process.
    #[oai(path = "/login", method = "post", operation_id = "login")]
    async fn login(
        &self,
        pool: Data<&PgPool>,
        login: Form<HashMap<String, String>>,
    ) -> Result<poem_openapi::payload::Response<PlainText<String>>> {
        let username = match login.get("username") {
            Some(it) => it,
            None => {
                return Ok(poem_openapi::payload::Response::new(PlainText(
                    "UsernameRequired".to_string(),
                ))
                .header("Location", "/sign-in#username_required")
                .status(StatusCode::FOUND))
            }
        };

        let email = match login.get("email") {
            Some(it) => it,
            None => {
                return Ok(poem_openapi::payload::Response::new(PlainText(
                    "EmailRequired".to_string(),
                ))
                .header("Location", "/sign-in#email_required")
                .status(StatusCode::FOUND))
            }
        };

        let token_type = login
            .get("token_type")
            .cloned()
            .unwrap_or("play_the_game".to_string());

        let callback_api = if token_type == "play_the_game" {
            "enter_the_underworld"
        } else {
            "enter_the_underworld_api"
        };

        let mut transaction = pool.0.begin().await.unwrap();
        let user_details = UserDetails {
            username: username.to_string(),
            email: email.to_string(),
        };
        let db_token = crate::auth::repository::get_mail_token(&mut transaction, &user_details)
            .await
            .unwrap();

        let response = match db_token {
            Some(token) => {
                let server_url = get_server_auth_url();
                let callback = format!("{}/{}?token={}", server_url, callback_api, token);

                poem_openapi::payload::Response::new(PlainText("Success".to_string()))
                    .header("Location", "/success")
                    .header(
                        "Set-Cookie",
                        format!("underworldCallback={}; Path=/; SameSite=Lax", callback),
                    )
                    .status(StatusCode::FOUND)
            }
            None => poem_openapi::payload::Response::new(PlainText("Success".to_string()))
                .header("Location", "/success")
                .header(
                    "Set-Cookie",
                    format!(
                        "underworldCallback={}; Path=/; SameSite=Lax; HttpOnly",
                        "mail_already_sent"
                    ),
                )
                .status(StatusCode::FOUND),
        };

        crate::auth::repository::delete_dead_mail_tokens(&mut transaction)
            .await
            .unwrap();
        transaction.commit().await.unwrap();
        Ok(response)
    }

    /// Validate a token from the mail and go to the game UI.
    #[oai(
        path = "/enter_the_underworld",
        method = "get",
        operation_id = "enter_the_underworld"
    )]
    async fn enter_the_underworld(
        &self,
        pool: Data<&PgPool>,
        token: Query<String>,
    ) -> Result<poem_openapi::payload::Response<PlainText<String>>> {
        let mut transaction = pool.0.begin().await.unwrap();
        let user_details =
            match crate::auth::repository::fetch_details_from_mail_token(&mut transaction, &token)
                .await
                .unwrap()
            {
                Some(it) => it,
                None => {
                    return Ok(poem_openapi::payload::Response::new(PlainText(
                        "INVALID TOKEN".to_string(),
                    ))
                    .status(StatusCode::BAD_REQUEST));
                }
            };

        let api_token = crate::auth::repository::get_api_token(&mut transaction, &user_details)
            .await
            .unwrap();

        let (location, same_site) = frontend_url(&api_token);
        let cookie = format!(
            "underworldToken={}; Path=/; {}; HttpOnly",
            &api_token, &same_site
        );

        let response = poem_openapi::payload::Response::new(PlainText("Success".to_string()))
            .header("Location", location)
            .header("Set-Cookie", cookie)
            .status(StatusCode::FOUND);
        transaction.commit().await.unwrap();
        Ok(response)
    }

    /// Validate a token from the mail and give the user the token.
    #[oai(
        path = "/enter_the_underworld_api",
        method = "get",
        operation_id = "enter_the_underworld_api"
    )]
    async fn enter_the_underworld_api(
        &self,
        pool: Data<&PgPool>,
        token: Query<String>,
    ) -> Result<poem_openapi::payload::Response<PlainText<String>>> {
        let mut transaction = pool.0.begin().await.unwrap();
        let user_details =
            match crate::auth::repository::fetch_details_from_mail_token(&mut transaction, &token)
                .await
                .unwrap()
            {
                Some(it) => it,
                None => {
                    return Ok(poem_openapi::payload::Response::new(PlainText(
                        "INVALID TOKEN".to_string(),
                    ))
                    .status(StatusCode::BAD_REQUEST));
                }
            };

        let api_token = crate::auth::repository::get_api_token(&mut transaction, &user_details)
            .await
            .unwrap();

        let response = poem_openapi::payload::Response::new(PlainText("Success".to_string()))
            .header("Location", "/token")
            .header(
                "Set-Cookie",
                format!("underworldApiToken={}; Path=/; SameSite=Lax", &api_token),
            )
            .status(StatusCode::FOUND);
        transaction.commit().await.unwrap();
        Ok(response)
    }
}
