use std::env;

use tide::prelude::*;
use tide::Body;
use tide::Request;
use tide::Response;
use underworld_core::components::non_player::NonPlayer;
use underworld_core::components::rooms::room::Room;
use underworld_core::generators::characters::CharacterPrototype;
use underworld_core::generators::generator::Generator;
use underworld_core::generators::non_players::NonPlayerPrototype;
use underworld_core::generators::rooms::RoomPrototype;
use underworld_core::generators::name::generate_name;

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
    pub room: Room,
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
    app.at("/").serve_dir("public")?;
    app.at("/").serve_file("public/index.html")?;
    app.listen(format!("0.0.0.0:{}", get_port())).await?;
    Ok(())
}

async fn generate_room_description_get(_req: Request<()>) -> tide::Result {
    generate_room_description()
}

async fn generate_room_description_post(_req: Request<()>) -> tide::Result {
    generate_room_description()
}

async fn generate_room_post(_req: Request<()>) -> tide::Result {
    generate_room()
}

async fn generate_room_get(_req: Request<()>) -> tide::Result {
    generate_room()
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

    let mut response = Response::new(200);
    let body = Body::from_json(&generated)?;
    response.set_body(body);

    Ok(response)
}

fn generate_room() -> tide::Result {
    let prototype = RoomPrototype::build_random();
    let room = prototype.generate();
    let generated = GeneratedRoom {
        room_description: format!("{}", &room),
        character_descriptions: room.look_at_inhabitants(),
        room,
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
        character_descriptions: room.look_at_inhabitants(),
    };
    let mut response = Response::new(200);
    let body = Body::from_json(&generated)?;
    response.set_body(body);

    Ok(response)
}
