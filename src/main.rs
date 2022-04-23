mod actions;
mod error;
mod event;
mod game;
mod player_characters;

use std::env;

use actions::PerformAction;
use error::GameError;
use game::{
    attack::{attack_npc, AttackNpcArgs, NpcAttacked},
    exit::{exit_current_room, ExitRoomArgs, RoomExited},
    generate::{generate_game, GenerateGameArgs, GeneratedGame},
    look::{look_at_room, quick_look_room, RoomLookArgs},
};
use player_characters::{
    current::{get_current_player_character, set_current_player_character, SetPlayerCharacterArgs},
    generate::{generate_player_character, GeneratePlayerCharacter, GeneratedPlayerCharacter},
    get::{get_player_character, player_character_ids},
};
use poem::{
    endpoint::StaticFilesEndpoint, listener::TcpListener, middleware::Cors, EndpointExt, Result,
    Route, Server,
};
use poem_openapi::{
    param::Path,
    payload::{Json, PlainText},
    ApiResponse, Object, OpenApi, OpenApiService,
};
use serde::{Deserialize, Serialize};
use underworld_core::{
    components::{
        character::CharacterViewArgs,
        non_player::NonPlayerView,
        player::PlayerCharacterView,
        rooms::room_view::{RoomView, RoomViewArgs},
    },
    generators::{
        characters::CharacterPrototype, generator::Generator, name::generate_name,
        non_players::NonPlayerPrototype, rooms::random_room_generator,
    },
    systems::view::{non_player, player, room},
};

#[derive(Serialize, Deserialize)]
struct GenerateCharacter {
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct GenerateRoomPostRequest {}

#[derive(Deserialize, Serialize)]
struct GenerateRoomGetRequest {}

#[derive(Serialize, Object, Deserialize)]
struct GeneratedNpc {
    pub non_player: NonPlayerView,
    pub inventory_description: String,
    pub species_description: String,
}

#[derive(Object, Serialize)]
struct GeneratedRoom {
    pub room_id: String,
    pub room_description: String,
    pub character_descriptions: String,
    pub actions: Vec<PerformAction>,
}

#[derive(Object, Serialize)]
struct AllActions {
    pub actions: Vec<PerformAction>,
}

#[derive(Deserialize, Object, Serialize)]
struct GeneratedRoomDescription {
    room_description: String,
    character_descriptions: String,
}

#[derive(ApiResponse)]
enum PlayerCharacterResponse {
    #[oai(status = 200)]
    PlayerCharacter(Json<PlayerCharacterView>),

    #[oai(status = 404)]
    NotFound(PlainText<String>),
}

#[derive(ApiResponse)]
enum PlayerCharacterIdsResponse {
    #[oai(status = 200)]
    PlayerCharacterIds(Json<Vec<String>>),
}

#[derive(ApiResponse)]
enum LookResponse {
    #[oai(status = 200)]
    LookAtRoom(Json<RoomView>),

    #[oai(status = 404)]
    NotFound(PlainText<String>),
}

#[derive(ApiResponse)]
enum RoomDescriptionResponse {
    #[oai(status = 201)]
    RoomDescriptions(Json<GeneratedRoomDescription>),
}

#[derive(ApiResponse)]
enum CharacterGeneratedResponse {
    #[oai(status = 200)]
    CharacterGenerated(Json<GeneratedNpc>),
}

#[derive(ApiResponse)]
enum PlayerCharacterGeneratedResponse {
    #[oai(status = 201)]
    PlayerCharacterGenerated(Json<GeneratedPlayerCharacter>),
}

#[derive(ApiResponse)]
pub enum GenerateGameResponse {
    #[oai(status = 201)]
    GameGenerated(Json<GeneratedGame>),

    #[oai(status = 500)]
    GameError(Json<GameError>),
}

#[derive(ApiResponse)]
enum SetCurrentPlayerCharacterResponse {
    #[oai(status = 200)]
    PlayerCharacterSet(PlainText<String>),

    #[oai(status = 500)]
    BadRequest(Json<Error>),
}

#[derive(ApiResponse)]
enum AttackNpcResponse {
    #[oai(status = 200)]
    NpcAttacked(Json<NpcAttacked>),

    #[oai(status = 500)]
    BadRequest(Json<Error>),
}

#[derive(ApiResponse)]
enum ExitRoomResponse {
    #[oai(status = 200)]
    RoomExited(Json<RoomExited>),

    #[oai(status = 500)]
    BadRequest(Json<Error>),
}

#[derive(Object, Serialize)]
pub struct Error {
    pub message: String,
}

fn get_port() -> u16 {
    env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080)
}

fn get_redis_url() -> String {
    env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1".to_string())
}

async fn get_redis_connection() -> redis::aio::Connection {
    let client = redis::Client::open(get_redis_url()).unwrap();
    client.get_async_connection().await.unwrap()
}

fn get_server_url() -> String {
    let base_url = env::var("SERVER_URL")
        .ok()
        .unwrap_or(format!("http://localhost:{}", get_port()));
    format!("{}/api", base_url)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let api_service =
        OpenApiService::new(UnderworldApi, "Underworld", "0.1.0").server(get_server_url());

    let ui = api_service.swagger_ui();
    let spec = api_service.spec();
    let route = Route::new()
        .nest(
            "/",
            StaticFilesEndpoint::new("./public").index_file("index.html"),
        )
        .nest("/api", api_service)
        .nest("/swagger_ui", ui)
        .at("/spec", poem::endpoint::make_sync(move |_| spec.clone()))
        .with(Cors::new());

    let listen_url = format!("0.0.0.0:{}", get_port());
    Server::new(TcpListener::bind(listen_url))
        .run(route)
        .await?;
    Ok(())
}

struct UnderworldApi;

#[OpenApi]
impl UnderworldApi {
    #[oai(path = "/game/generate", method = "post")]
    async fn generate_game(&self, args: Json<GenerateGameArgs>) -> Result<GenerateGameResponse> {
        let mut connection = get_redis_connection().await;
        let generated_result = generate_game(&mut connection, &args).await;

        match generated_result {
            Ok(generated_game) => Ok(GenerateGameResponse::GameGenerated(Json(generated_game))),
            Err(e) => Ok(GenerateGameResponse::GameError(Json(e))),
        }
    }

    #[oai(path = "/game/exit_current_room", method = "post")]
    async fn exit_current_room(&self, args: Json<ExitRoomArgs>) -> Result<ExitRoomResponse> {
        let mut connection = get_redis_connection().await;

        let exit_result = exit_current_room(&mut connection, &args).await;
        match exit_result {
            Ok(it) => Ok(ExitRoomResponse::RoomExited(Json(it))),
            Err(it) => Ok(ExitRoomResponse::BadRequest(Json(Error {
                message: it.to_string(),
            }))),
        }
    }

    #[oai(path = "/game/attack_npc", method = "post")]
    async fn attack_npc(&self, args: Json<AttackNpcArgs>) -> Result<AttackNpcResponse> {
        let mut connection = get_redis_connection().await;

        let attack_result = attack_npc(&mut connection, &args).await;

        match attack_result {
            Ok(it) => Ok(AttackNpcResponse::NpcAttacked(Json(it))),
            Err(e) => Ok(AttackNpcResponse::BadRequest(Json(Error {
                message: e.to_string(),
            }))),
        }
    }

    #[oai(path = "/game/look_at_current_room", method = "post")]
    async fn look_at_current_room(&self, args: Json<RoomLookArgs>) -> Result<LookResponse> {
        let mut connection = get_redis_connection().await;
        let view_result = look_at_room(&mut connection, &args).await;

        match view_result {
            Ok(it) => Ok(LookResponse::LookAtRoom(Json(it))),
            Err(e) => Ok(LookResponse::NotFound(PlainText(e.to_string()))),
        }
    }

    #[oai(path = "/game/quick_look_current_room", method = "post")]
    async fn quick_look_current_room(&self, args: Json<RoomLookArgs>) -> Result<LookResponse> {
        let mut connection = get_redis_connection().await;
        let view_result = quick_look_room(&mut connection, &args).await;

        match view_result {
            Ok(it) => Ok(LookResponse::LookAtRoom(Json(it))),
            Err(e) => Ok(LookResponse::NotFound(PlainText(e.to_string()))),
        }
    }

    #[oai(path = "/room/descriptions/generate", method = "post")]
    async fn generate_room_description(&self) -> Result<RoomDescriptionResponse> {
        let room_generator = random_room_generator(None);
        let room = room_generator.generate();

        let quick_view = room::quick_look(&room);
        let args = RoomViewArgs {
            can_see_hidden: false,
            can_see_packed: false,
            knows_character_health: false,
            knows_names: true,
        };
        let deeper_look = room::look_at(&room, args, false);
        let generated = GeneratedRoomDescription {
            room_description: format!("{}", &quick_view),
            character_descriptions: deeper_look.describe_inhabitants(),
        };

        Ok(RoomDescriptionResponse::RoomDescriptions(Json(generated)))
    }

    #[oai(path = "/npc/random", method = "get")]
    async fn generate_character(&self) -> Result<CharacterGeneratedResponse> {
        let prototype = NonPlayerPrototype {
            name: generate_name(),
            character_generator: Box::new(CharacterPrototype::random_species_overloaded()),
        };

        let non_player = prototype.generate();

        let character_args = CharacterViewArgs {
            knows_health: true,
            knows_species: true,
            knows_life_modifier: true,
            knows_inventory: true,
            knows_hidden_in_inventory: true,
            knows_packed_in_inventory: true,
        };
        let view = non_player::look_at(&non_player, &character_args, true, true);

        let generated = GeneratedNpc {
            inventory_description: view.character.describe_inventory(""),
            species_description: view.character.describe_species(),
            non_player: view,
        };

        Ok(CharacterGeneratedResponse::CharacterGenerated(Json(
            generated,
        )))
    }

    #[oai(path = "/player_character/generate", method = "post")]
    async fn generate_player_character(
        &self,
        args: Json<GeneratePlayerCharacter>,
    ) -> Result<PlayerCharacterGeneratedResponse> {
        let mut connection = get_redis_connection().await;
        let result = generate_player_character(&mut connection, &args).await;

        Ok(PlayerCharacterGeneratedResponse::PlayerCharacterGenerated(
            Json(result),
        ))
    }

    #[oai(path = "/:username/player_characters", method = "get")]
    async fn list_player_characters(
        &self,
        username: Path<String>,
    ) -> Result<PlayerCharacterIdsResponse> {
        let mut connection = get_redis_connection().await;
        let result = player_character_ids(&mut connection, &username).await;

        Ok(PlayerCharacterIdsResponse::PlayerCharacterIds(Json(result)))
    }

    #[oai(path = "/:username/player_character/:id/check", method = "get")]
    async fn check_player_character(
        &self,
        username: Path<String>,
        id: Path<String>,
    ) -> Result<PlayerCharacterResponse> {
        let mut connection = get_redis_connection().await;
        let result = get_player_character(&mut connection, &username, &id).await;

        match result {
            Some(it) => Ok(PlayerCharacterResponse::PlayerCharacter(Json(
                player::check(it),
            ))),
            None => Ok(PlayerCharacterResponse::NotFound(PlainText(format!(
                "No player character found for user {} id {}",
                &username.0, &id.0
            )))),
        }
    }

    #[oai(path = "/set_current_player_character", method = "post")]
    async fn set_current_player_character(
        &self,
        args: Json<SetPlayerCharacterArgs>,
    ) -> Result<SetCurrentPlayerCharacterResponse> {
        let mut connection = get_redis_connection().await;
        let result = set_current_player_character(&mut connection, &args.0).await;

        match result {
            Ok(_) => Ok(SetCurrentPlayerCharacterResponse::PlayerCharacterSet(
                PlainText("Good to go".to_string()),
            )),
            Err(e) => Ok(SetCurrentPlayerCharacterResponse::BadRequest(Json(Error {
                message: format!("{}", e),
            }))),
        }
    }

    #[oai(path = "/:username/check_current_player_character", method = "get")]
    async fn check_current_player_character(
        &self,
        username: Path<String>,
    ) -> Result<PlayerCharacterResponse> {
        let mut connection = get_redis_connection().await;
        let player_character_result =
            get_current_player_character(&mut connection, &username).await;

        match player_character_result {
            Ok(it) => Ok(PlayerCharacterResponse::PlayerCharacter(Json(
                player::check(it),
            ))),
            Err(_) => Ok(PlayerCharacterResponse::NotFound(PlainText(
                "No character found".to_string(),
            ))),
        }
    }
}
