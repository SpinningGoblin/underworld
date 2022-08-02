mod actions;
mod api;
mod auth;
pub mod config;
mod error;
mod event;
mod game;
mod player_characters;
pub mod tags;

use api::{
    auth::UnderworldAuthApi, game::UnderworldGameApi, game_actions::UnderworldGameActionApi,
    middleware::CookieToTokenMiddleware, npc::UnderworldNpcApi, player::UnderworldPlayerApi,
};
use config::{get_port, get_psql_url, get_server_api_url, get_server_auth_url};
use poem::{
    endpoint::StaticFilesEndpoint,
    http::StatusCode,
    listener::TcpListener,
    middleware::Cors,
    session::{CookieConfig, CookieSession},
    EndpointExt, IntoResponse, Result, Route, Server,
};
use poem_openapi::OpenApiService;
use sqlx::PgPool;

struct UnauthResponse;

impl IntoResponse for UnauthResponse {
    fn into_response(self) -> poem::Response {
        poem::Response::builder()
            .status(StatusCode::FOUND)
            .header("Location", "/")
            .finish()
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let pool = sqlx::PgPool::connect(&get_psql_url()).await.unwrap();

    let api_service = OpenApiService::new(
        (
            UnderworldNpcApi,
            UnderworldPlayerApi,
            UnderworldGameApi,
            UnderworldGameActionApi,
        ),
        "Underworld",
        "0.5.0",
    )
    .server(get_server_api_url());

    let auth_service = OpenApiService::new(UnderworldAuthApi, "UnderworldAuth", "0.5.0")
        .server(format!("{}/auth", get_server_auth_url()));

    let ui = api_service.swagger_ui();
    let spec = api_service.spec();
    let route = Route::new()
        .nest(
            "/docs",
            StaticFilesEndpoint::new("./public_docs").index_file("index.html"),
        )
        .nest(
            "/",
            StaticFilesEndpoint::new("./public")
                .index_file("index.html")
                .before(|req| async move {
                    let token_cookie = req.cookie().get("underworldToken");
                    let token: String = match &token_cookie {
                        Some(token_cookie) => token_cookie.value_str().to_string(),
                        None => {
                            return Err(poem::Error::from_status(StatusCode::UNAUTHORIZED));
                        }
                    };

                    let pool_option: Option<&PgPool> = req.data();
                    let pool = pool_option.unwrap();

                    let valid_token = auth::repository::valid_api_token(pool, &token).await;
                    match valid_token {
                        Ok(_) => {}
                        Err(_) => {
                            return Err(poem::Error::from_status(StatusCode::UNAUTHORIZED));
                        }
                    }

                    Ok(req)
                })
                .after(|result| async move {
                    match result {
                        Err(e) => {
                            let response = e.into_response();
                            match &response.status() {
                                &StatusCode::UNAUTHORIZED => Ok(poem::Response::builder()
                                    .header("Location", "/sign-in")
                                    .status(StatusCode::FOUND)
                                    .finish()),
                                _ => Ok(response),
                            }
                        }
                        Ok(response) => Ok(response),
                    }
                }),
        )
        .nest(
            "/sign-in",
            StaticFilesEndpoint::new("./sign-in").index_file("index.html"),
        )
        .nest(
            "/success",
            StaticFilesEndpoint::new("./success").index_file("index.html"),
        )
        .nest(
            "/token",
            StaticFilesEndpoint::new("./token").index_file("index.html"),
        )
        .nest("/api", api_service.with(CookieToTokenMiddleware))
        .nest("/auth", auth_service)
        .nest("/swagger_ui", ui)
        .at("/spec", poem::endpoint::make_sync(move |_| spec.clone()))
        .with(Cors::new())
        .with(CookieSession::new(CookieConfig::default().secure(false)))
        .data(pool);

    let listen_url = format!("0.0.0.0:{}", get_port());
    Server::new(TcpListener::bind(listen_url))
        .run(route)
        .await?;
    Ok(())
}
