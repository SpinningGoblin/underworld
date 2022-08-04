use std::{collections::HashMap, env};

use poem::{http::StatusCode, web::Data, Result};
use poem_openapi::{
    param::Query,
    payload::{Form, PlainText},
    OpenApi,
};
use sqlx::PgPool;

use crate::{
    auth::repository::UserDetails, config::get_server_auth_url, mail::send_mail,
    tags::UnderworldApiTags,
};

pub struct UnderworldAuthApi;

fn frontend_url() -> Option<String> {
    match env::var("FRONTEND_URL") {
        Ok(it) => Some(it),
        Err(_) => None,
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
        let email = match login.get("email") {
            Some(it) => it,
            None => {
                let frontend_url = frontend_url().unwrap_or("/".to_string());
                return Ok(poem_openapi::payload::Response::new(PlainText(
                    "EmailRequired".to_string(),
                ))
                .header(
                    "Location",
                    format!("{}/sign-in?error=email_required", &frontend_url),
                )
                .status(StatusCode::FOUND));
            }
        };

        let token_type = login
            .get("token_type")
            .cloned()
            .unwrap_or("play_the_game".to_string());

        let mut transaction = pool.0.begin().await.unwrap();
        let user_details = UserDetails {
            email: email.to_string(),
        };
        let db_token = crate::auth::repository::get_mail_token(&mut transaction, &user_details)
            .await
            .unwrap();

        let frontend_url = frontend_url().unwrap_or("/".to_string());
        let response = match db_token {
            Some(token) => {
                let callback = if token_type == "play_the_game" {
                    format!("{}?mail_token={}", &frontend_url, token)
                } else {
                    format!(
                        "{}/enter_the_underworld_api?token={}",
                        get_server_auth_url(),
                        &token
                    )
                };
                // let server_url = get_server_auth_url();

                send_mail(email, &env::var("FROM_EMAIL").unwrap(), &callback).await;

                poem_openapi::payload::Response::new(PlainText("Success".to_string()))
                    .header("Location", format!("{}/success", &frontend_url))
                    .status(StatusCode::FOUND)
            }
            None => poem_openapi::payload::Response::new(PlainText("Success".to_string()))
                .header("Location", format!("{}/success", &frontend_url))
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

        let frontend_url = frontend_url().unwrap_or("/".to_string());
        let cookie = format!(
            "underworldToken={}; Path=/; SameSite=Lax; HttpOnly",
            &api_token,
        );

        let response = poem_openapi::payload::Response::new(PlainText("Success".to_string()))
            .header("Location", format!("{}#{}", &frontend_url, &api_token))
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
