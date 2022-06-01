mod actions;
mod api;
mod error;
mod event;
mod game;
mod player_characters;
mod psql;
pub mod tags;

use std::env;

use api::{
    game::UnderworldGameApi, game_actions::UnderworldGameActionApi, npc::UnderworldNpcApi,
    player::UnderworldPlayerApi,
};
use poem::{
    endpoint::StaticFilesEndpoint, listener::TcpListener, middleware::Cors, EndpointExt, Result,
    Route, Server,
};
use poem_openapi::OpenApiService;
use psql::get_psql_url;

fn get_port() -> u16 {
    env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080)
}

fn get_server_url() -> String {
    let base_url = env::var("SERVER_URL")
        .ok()
        .unwrap_or(format!("http://localhost:{}", get_port()));
    format!("{}/api", base_url)
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
        "0.2.0",
    )
    .server(get_server_url());

    let ui = api_service.swagger_ui();
    let spec = api_service.spec();
    let route = Route::new()
        .nest(
            "/docs",
            StaticFilesEndpoint::new("./public_docs").index_file("index.html"),
        )
        .nest("/", StaticFilesEndpoint::new("./public").index_file("index.html"))
        .nest("/api", api_service)
        .nest("/swagger_ui", ui)
        .at("/spec", poem::endpoint::make_sync(move |_| spec.clone()))
        .with(Cors::new())
        .data(pool);

    let listen_url = format!("0.0.0.0:{}", get_port());
    Server::new(TcpListener::bind(listen_url))
        .run(route)
        .await?;
    Ok(())
}
