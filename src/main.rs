use std::env;

use redis::AsyncCommands;
use redis::RedisError;
use tide::prelude::*;
use tide::Body;
use tide::Request;
use tide::Response;
use tide::StatusCode;
use underworld_core::components::rooms::room::Room;
use underworld_core::components::rooms::room::RoomViewArgs;
use underworld_core::{
    components::non_player::NonPlayer,
    generators::{
        characters::CharacterPrototype, generator::Generator, name::generate_name,
        non_players::NonPlayerPrototype, rooms::RoomPrototype,
    },
};

#[derive(Serialize, Deserialize)]
struct GenerateCharacter {
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct GenerateRoomPostRequest {}

#[derive(Deserialize, Serialize)]
struct GenerateRoomGetRequest {}

#[derive(Serialize, Deserialize)]
struct GeneratedNpc {
    pub non_player: NonPlayer,
    pub inventory_description: String,
    pub species_description: String,
}

#[derive(Deserialize, Serialize)]
struct GeneratedRoom {
    pub room_id: String,
    pub room_description: String,
    pub character_descriptions: String,
}

#[derive(Deserialize, Serialize)]
struct GeneratedRoomDescription {
    room_description: String,
    character_descriptions: String,
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

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/generate/npc").post(generate_character_post);
    app.at("/generate/npc").get(generate_character_get);
    app.at("/generate/room").post(generate_room_post);
    app.at("/generate/room").get(generate_room_get);
    app.at("/generate/room_description")
        .post(generate_room_description_post);
    app.at("/generate/room_description")
        .get(generate_room_description_get);
    app.at("/rooms").get(get_all_rooms);
    app.at("room/:id/quick_look").get(quick_look_at_room);
    app.at("room/:id/look_at").get(look_at_room);
    app.at("/").serve_dir("public")?;
    app.at("/").serve_file("public/index.html")?;
    app.listen(format!("0.0.0.0:{}", get_port())).await?;
    Ok(())
}

async fn load_room(id: String) -> Option<Room> {
    let mut connection = get_redis_connection().await;
    let serialized: Result<String, RedisError> = connection.get(format!("room:{}", &id)).await;
    match serialized {
        Ok(it) => {
            let room: Room = serde_json::from_str(&it).unwrap();
            Some(room)
        },
        Err(_) => None,
    }
}

async fn get_all_rooms(_req: Request<()>) -> tide::Result {
    let mut connection = get_redis_connection().await;
    let keys: Vec<String> = connection.keys("room:*").await.unwrap();
    let ids: Vec<String> = keys.iter().map(|k| k.replace("room:", "")).collect();
    let mut response = Response::new(StatusCode::Ok);
    let body = Body::from_json(&ids).unwrap();
    response.set_body(body);
    Ok(response)
}

async fn look_at_room(req: Request<()>) -> tide::Result {
    let id = req.param("id").unwrap();
    match load_room(id.to_string()).await {
        Some(room) => {
            let args = RoomViewArgs {
                can_see_hidden: false,
                can_see_packed: false,
                knows_character_health: false,
                knows_names: false,
            };
            let view = room.look_at(args, false);
            let mut response = Response::new(StatusCode::Ok);
            let body = Body::from_json(&view)?;
            response.set_body(body);
            Ok(response)
        },
        None => Ok(Response::new(StatusCode::NotFound)),
    }
}

async fn quick_look_at_room(req: Request<()>) -> tide::Result {
    let id = req.param("id").unwrap();
    match load_room(id.to_string()).await {
        Some(room) => {
            let view = room.quick_look();
            let mut response = Response::new(StatusCode::Ok);
            let body = Body::from_json(&view)?;
            response.set_body(body);
            Ok(response)
        },
        None => Ok(Response::new(StatusCode::NotFound)),
    }
}

async fn generate_room_description_get(_req: Request<()>) -> tide::Result {
    generate_room_description()
}

async fn generate_room_description_post(_req: Request<()>) -> tide::Result {
    generate_room_description()
}

async fn generate_room_post(_req: Request<()>) -> tide::Result {
    generate_room().await
}

async fn generate_room_get(_req: Request<()>) -> tide::Result {
    generate_room().await
}

async fn generate_character_get(_req: Request<()>) -> tide::Result {
    generate_character()
}

async fn generate_character_post(_req: Request<()>) -> tide::Result {
    generate_character()
}

fn generate_character() -> tide::Result {
    let prototype = NonPlayerPrototype {
        name: generate_name(),
        character_generator: Box::new(CharacterPrototype::random_species_overloaded()),
    };

    let non_player = prototype.generate();
    let generated = GeneratedNpc {
        inventory_description: non_player.character.describe_inventory("").clone(),
        species_description: non_player.character.describe_species().clone(),
        non_player,
    };

    let mut response = Response::new(StatusCode::Ok);
    let body = Body::from_json(&generated)?;
    response.set_body(body);

    Ok(response)
}

async fn generate_room() -> tide::Result {
    let prototype = RoomPrototype::build_random();
    let room = prototype.generate();
    let serialized = serde_json::to_string(&room).unwrap();
    let mut connection = get_redis_connection().await;
    let _: () = connection
        .set(format!("room:{}", &room.identifier.id), serialized)
        .await
        .unwrap();

    let generated = GeneratedRoom {
        room_description: format!("{}", &room),
        character_descriptions: room.describe_inhabitants(),
        room_id: room.identifier.id.to_string(),
    };

    let mut response = Response::new(200);
    let body = Body::from_json(&generated)?;
    response.set_body(body);

    Ok(response)
}

fn generate_room_description() -> tide::Result {
    let prototype = RoomPrototype::build_random();
    let room = prototype.generate();
    let generated = GeneratedRoomDescription {
        room_description: format!("{}", &room),
        character_descriptions: room.describe_inhabitants(),
    };
    let mut response = Response::new(200);
    let body = Body::from_json(&generated)?;
    response.set_body(body);

    Ok(response)
}
