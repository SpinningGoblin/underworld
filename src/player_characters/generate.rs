use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};
use underworld_core::{
    components::{size::Size, species::Species},
    generators::{generator::Generator, players::player_generator},
};

use crate::actions::{player_character_actions, PerformAction};

#[derive(Deserialize, Object)]
pub struct GeneratePlayerCharacter {
    pub character_size: Option<Size>,
    pub character_species: Option<Species>,
    pub character_name: Option<String>,
}

#[derive(Serialize, Object)]
pub struct GeneratedPlayerCharacter {
    pub actions: Vec<PerformAction>,
    pub player_character_id: String,
    pub set_as_current: bool,
}

pub async fn generate_player_character(
    transaction: &mut Transaction<'_, Postgres>,
    username: &str,
    args: &GeneratePlayerCharacter,
) -> GeneratedPlayerCharacter {
    let generator = player_generator(
        args.character_name.clone(),
        args.character_species.clone(),
        args.character_size.clone(),
    );

    let player_character = generator.generate();
    super::repository::save(transaction, &username, &player_character)
        .await
        .unwrap();

    let mut set_as_current = false;

    if super::repository::current(transaction, &username)
        .await
        .unwrap()
        .is_none()
    {
        super::repository::set_current(transaction, &username, &player_character)
            .await
            .unwrap();
        set_as_current = true;
    }

    GeneratedPlayerCharacter {
        actions: player_character_actions(&username, &player_character.identifier.id.to_string()),
        player_character_id: player_character.identifier.id.to_string(),
        set_as_current,
    }
}
