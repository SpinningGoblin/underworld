use std::env;

use tide::Body;
use tide::Request;
use tide::Response;
use tide::prelude::*;
use underworld_core::components::non_player::NonPlayer;
use underworld_core::generators::characters::CharacterPrototype;
use underworld_core::generators::generator::Generator;
use underworld_core::generators::non_players::NonPlayerPrototype;

#[derive(Serialize, Deserialize)]
struct GenerateCharacter {
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct GeneratedNpc {
    pub non_player: NonPlayer,
    pub inventory_description: String,
    pub species_description: String,
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
    app.listen(format!("0.0.0.0:{}", get_port())).await?;
    Ok(())
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
        inventory_description: non_player.character.describe_inventory().clone(),
        species_description: non_player.character.describe_species().clone(),
        non_player,
    };

    let mut response = Response::new(200);
    let body = Body::from_json(&generated)?;
    response.set_body(body);

    Ok(response)
}
