use std::env;

use poem::endpoint::StaticFilesEndpoint;
use poem::listener::TcpListener;
use poem::middleware::Cors;
use poem::EndpointExt;
use poem::Result;
use poem::Route;
use poem::Server;
use poem_openapi::{
    param::Path,
    payload::{Json, PlainText},
    ApiResponse, Object, OpenApi, OpenApiService,
};
use redis::AsyncCommands;
use redis::RedisError;
use serde::Deserialize;
use serde::Serialize;
use underworld_core::actions::action::Action;
use underworld_core::components::character::CharacterViewArgs;
use underworld_core::components::non_player::NonPlayerView;
use underworld_core::components::rooms::room::Room;
use underworld_core::components::rooms::room_view::RoomView;
use underworld_core::components::rooms::room_view::RoomViewArgs;
use underworld_core::generators::{
    characters::CharacterPrototype, generator::Generator, name::generate_name,
    non_players::NonPlayerPrototype, rooms::RoomPrototype,
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

#[derive(Deserialize, Object, Serialize)]
struct PerformAction {
    pub name: String,
    pub description: String,
    pub link: String,
    pub http_action: String,
}

#[derive(Deserialize, Object, Serialize)]
struct GeneratedRoom {
    pub room_id: String,
    pub room_description: String,
    pub character_descriptions: String,
    pub actions: Vec<PerformAction>,
}

#[derive(Deserialize, Object, Serialize)]
struct AllActions {
    pub actions: Vec<PerformAction>,
}

#[derive(Deserialize, Object, Serialize)]
struct GeneratedRoomDescription {
    room_description: String,
    character_descriptions: String,
}

#[derive(ApiResponse)]
enum GetResponse {
    #[oai(status = 200)]
    RoomIds(Json<Vec<String>>),

    #[oai(status = 200)]
    AllActions(Json<AllActions>),

    #[oai(status = 404)]
    NotFound(PlainText<String>),
}

#[derive(ApiResponse)]
enum LookResponse {
    #[oai(status = 404)]
    LookAtRoom(Json<RoomView>),

    #[oai(status = 404)]
    NotFound(PlainText<String>),
}

#[derive(ApiResponse)]
enum GenerateResponse {
    #[oai(status = 201)]
    RoomGenerated(Json<GeneratedRoom>),

    #[oai(status = 201)]
    RoomDescriptions(Json<GeneratedRoomDescription>),

    #[oai(status = 201)]
    CharacterGenerated(Json<GeneratedNpc>),
}

fn get_port() -> u16 {
    env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080)
}

fn get_redis_url() -> String {
    env::var("REDIS_URL").unwrap()
}

async fn get_redis_connection() -> redis::aio::Connection {
    let client = redis::Client::open(get_redis_url()).unwrap();
    client.get_async_connection().await.unwrap()
}

fn get_server_url() -> String {
    env::var("SERVER_URL")
        .ok()
        .unwrap_or(format!("http://localhost:{}", get_port()))
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let api_service =
        OpenApiService::new(UnderworldApi, "Underworld", "0.1.0").server(get_server_url());

    let ui = api_service.swagger_ui();
    let spec = api_service.spec();
    let route = Route::new()
        .nest("/", StaticFilesEndpoint::new("./public").index_file("index.html"))
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

async fn load_room(id: &str) -> Option<Room> {
    let mut connection = get_redis_connection().await;
    let serialized: Result<String, RedisError> = connection.get(format!("room:{}", &id)).await;
    match serialized {
        Ok(it) => {
            let room: Room = serde_json::from_str(&it).unwrap();
            Some(room)
        }
        Err(_) => None,
    }
}

fn get_api_link(original: &str) -> String {
    format!("/api/{}", original)
}

fn get_look_at_link(room_id: &str) -> String {
    get_api_link(&format!("room/{}/look_at", &room_id))
}

fn get_quick_look_link(room_id: &str) -> String {
    get_api_link(&format!("room/{}/quick_look", &room_id))
}

struct UnderworldApi;

#[OpenApi]
impl UnderworldApi {
    #[oai(path = "/rooms", method = "get")]
    async fn get_all_room_ids(&self) -> Result<GetResponse> {
        let mut connection = get_redis_connection().await;
        let keys: Vec<String> = connection.keys("room:*").await.unwrap();
        let ids: Vec<String> = keys.iter().map(|k| k.replace("room:", "")).collect();
        Ok(GetResponse::RoomIds(Json(ids)))
    }

    #[oai(path = "/room/:id/actions", method = "get")]
    async fn get_all_actions(&self, id: Path<String>) -> Result<GetResponse> {
        match load_room(&id.0).await {
            Some(room) => {
                let actions: Vec<PerformAction> = room
                    .current_actions()
                    .iter()
                    .filter_map(|action| match action {
                        Action::LookAtTarget(_) => None,
                        Action::LookAtRoom(it) => Some(PerformAction {
                            name: "look_at_room".to_string(),
                            description: it.description(),
                            link: get_look_at_link(&id.0),
                            http_action: "POST".to_string(),
                        }),
                        Action::QuickLookRoom(it) => Some(PerformAction {
                            name: "quick_look_room".to_string(),
                            description: it.description(),
                            link: get_quick_look_link(&id.0),
                            http_action: "POST".to_string(),
                        }),
                    })
                    .collect();
                let all_actions = AllActions { actions };
                Ok(GetResponse::AllActions(Json(all_actions)))
            }
            None => Ok(GetResponse::NotFound(PlainText(format!(
                "room `{}` not found",
                &id.0
            )))),
        }
    }

    #[oai(path = "/room/:id/look_at", method = "post")]
    async fn look_at_room(&self, id: Path<String>) -> Result<LookResponse> {
        match load_room(&id.0).await {
            Some(room) => {
                let args = RoomViewArgs {
                    can_see_hidden: false,
                    can_see_packed: false,
                    knows_character_health: false,
                    knows_names: true,
                };
                let view = room.look_at(args, false);
                Ok(LookResponse::LookAtRoom(Json(view)))
            }
            None => Ok(LookResponse::NotFound(PlainText(format!(
                "room `{}` not found",
                &id.0
            )))),
        }
    }

    #[oai(path = "/room/:id/quick_look", method = "post")]
    async fn quick_look_at_room(&self, id: Path<String>) -> Result<LookResponse> {
        match load_room(&id.0).await {
            Some(room) => {
                let view = room.quick_look();
                Ok(LookResponse::LookAtRoom(Json(view)))
            }
            None => Ok(LookResponse::NotFound(PlainText(format!(
                "room `{}` not found",
                &id.0
            )))),
        }
    }

    #[oai(path = "/room/generate", method = "post")]
    async fn generate_room(&self) -> Result<GenerateResponse> {
        let prototype = RoomPrototype::build_random();
        let room = prototype.generate();
        let serialized = serde_json::to_string(&room).unwrap();
        let mut connection = get_redis_connection().await;
        let _: () = connection
            .set(format!("room:{}", &room.identifier.id), serialized)
            .await
            .unwrap();

        let room_id = room.identifier.id.to_string();

        let mut actions: Vec<PerformAction> = room
            .current_actions()
            .iter()
            .filter_map(|action| match action {
                Action::LookAtTarget(_) => None,
                Action::LookAtRoom(it) => Some(PerformAction {
                    name: "look_at_room".to_string(),
                    description: it.description(),
                    link: get_look_at_link(&room_id),
                    http_action: "POST".to_string(),
                }),
                Action::QuickLookRoom(it) => Some(PerformAction {
                    name: "quick_look_room".to_string(),
                    description: it.description(),
                    link: get_quick_look_link(&room_id),
                    http_action: "POST".to_string(),
                }),
            })
            .collect();

        actions.push(PerformAction {
            name: "get_all_actions".to_string(),
            description: "Get all actions for the room".to_string(),
            link: get_api_link(&format!("room/{}/actions", &room_id)),
            http_action: "GET".to_string(),
        });

        let generated = GeneratedRoom {
            room_description: format!("{}", &room),
            character_descriptions: room.describe_inhabitants(),
            room_id,
            actions,
        };

        Ok(GenerateResponse::RoomGenerated(Json(generated)))
    }

    #[oai(path = "/room/descriptions/generate", method = "post")]
    async fn generate_room_description(&self) -> Result<GenerateResponse> {
        let prototype = RoomPrototype::build_random();
        let room = prototype.generate();
        let generated = GeneratedRoomDescription {
            room_description: format!("{}", &room),
            character_descriptions: room.describe_inhabitants(),
        };

        Ok(GenerateResponse::RoomDescriptions(Json(generated)))
    }

    #[oai(path = "/generate/npc", method = "post")]
    async fn generate_character(&self) -> Result<GenerateResponse> {
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

        let generated = GeneratedNpc {
            inventory_description: non_player.character.describe_inventory("").clone(),
            species_description: non_player.character.describe_species().clone(),
            non_player: non_player.look_at(&character_args, true, true),
        };

        Ok(GenerateResponse::CharacterGenerated(Json(generated)))
    }
}
