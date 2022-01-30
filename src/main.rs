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

#[derive(Serialize, Deserialize)]
struct GenerateCharacter {
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct GenerateRoomPostRequest {
    #[cfg_attr(feature = "serialization", serde(default))]
    pub names: Option<Vec<String>>,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub max_num_characters: Option<usize>,
}

#[derive(Deserialize, Serialize)]
struct GenerateRoomGetRequest {
    #[cfg_attr(feature = "serialization", serde(default))]
    pub names: Option<String>,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub max_num_characters: Option<usize>,
}

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
    app.listen(format!("0.0.0.0:{}", get_port())).await?;
    Ok(())
}

async fn generate_room_description_get(req: Request<()>) -> tide::Result {
    let GenerateRoomGetRequest {
        names,
        max_num_characters,
    } = req.query()?;

    let npc_names: Vec<String> = names
        .map(|n| n.split(",").map(|f| f.to_string()).collect())
        .unwrap_or_default();

    generate_room_description(npc_names, max_num_characters.unwrap_or(3))
}

async fn generate_room_description_post(mut req: Request<()>) -> tide::Result {
    let GenerateRoomPostRequest {
        names,
        max_num_characters,
    } = req.body_json().await?;

    generate_room_description(names.unwrap_or_default(), max_num_characters.unwrap_or(3))
}

async fn generate_room_post(mut req: Request<()>) -> tide::Result {
    let GenerateRoomPostRequest {
        names,
        max_num_characters,
    } = req.body_json().await?;

    generate_room(names.unwrap_or_default(), max_num_characters.unwrap_or(3))
}

async fn generate_room_get(req: Request<()>) -> tide::Result {
    let GenerateRoomGetRequest {
        names,
        max_num_characters,
    } = req.query()?;

    let npc_names: Vec<String> = names
    .map(|n| n.split(",").map(|f| f.to_string()).collect())
    .unwrap_or_default();

generate_room(npc_names, max_num_characters.unwrap_or(3))
}

async fn generate_character_get(req: Request<()>) -> tide::Result {
    let GenerateCharacter { name } = req.query()?;

    generate_character(name)
}

async fn generate_character_post(mut req: Request<()>) -> tide::Result {
    let GenerateCharacter { name } = req.body_json().await?;

    generate_character(name)
}

fn generate_character(name: Option<String>) -> tide::Result {
    let prototype = NonPlayerPrototype {
        name,
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

fn generate_room(names: Vec<String>, max_num_characters: usize) -> tide::Result {
    let prototype = RoomPrototype::build_random(names, 0..max_num_characters);
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

fn generate_room_description(names: Vec<String>, max_num_characters: usize) -> tide::Result {
    let prototype = RoomPrototype::build_random(names, 0..max_num_characters);
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
