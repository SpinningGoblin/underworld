use poem_openapi::Object;
use redis::{aio::Connection, AsyncCommands};
use serde::{Deserialize, Serialize};
use underworld_core::{
    components::{size::Size, species::Species},
    generators::{generator::Generator, players::player_generator},
};

use crate::actions::{player_character_actions, PerformAction};

use super::utils::username_player_character_key;

#[derive(Deserialize, Object)]
pub struct GeneratePlayerCharacter {
    pub username: String,
    pub character_size: Option<Size>,
    pub character_species: Option<Species>,
    pub character_name: Option<String>,
}

#[derive(Serialize, Object)]
pub struct GeneratedPlayerCharacter {
    pub actions: Vec<PerformAction>,
    pub player_character_id: String,
}

pub async fn generate_player_character(
    connection: &mut Connection,
    args: &GeneratePlayerCharacter,
) -> GeneratedPlayerCharacter {
    let generator = player_generator(
        &args.username,
        args.character_name.clone(),
        args.character_species.clone(),
        args.character_size.clone(),
    );

    let player_character = generator.generate();
    let pc_id = player_character.identifier.id.to_string();
    let redis_key = username_player_character_key(&args.username, &pc_id);
    let serialized = serde_json::to_string(&player_character).unwrap();
    let _: () = connection.set(&redis_key, serialized).await.unwrap();

    GeneratedPlayerCharacter {
        actions: player_character_actions(&args.username, &pc_id),
        player_character_id: pc_id,
    }
}
