use crate::tags::UnderworldApiTags;
use poem::Result;
use poem_openapi::{payload::Json, ApiResponse, Object, OpenApi};
use serde::{Deserialize, Serialize};
use underworld_core::{
    components::{CharacterViewArgs, NonPlayerView},
    generators::{generator::Generator, name::generate_name, non_players::npc_generator},
    systems::view::non_player,
};

#[derive(Serialize, Deserialize)]
struct GenerateCharacter {
    pub name: Option<String>,
}

#[derive(Serialize, Object, Deserialize)]
struct GeneratedNpc {
    pub non_player: NonPlayerView,
}

#[derive(ApiResponse)]
enum CharacterGeneratedResponse {
    #[oai(status = 200)]
    CharacterGenerated(Json<GeneratedNpc>),
}

pub struct UnderworldRandomizerApi;

#[OpenApi(tag = "UnderworldApiTags::Randomizers", prefix_path = "/random")]
impl UnderworldRandomizerApi {
    /// Generate a random NPC.
    ///
    /// # Example
    ///
    /// Call `/npc/random` to generate a completely random character
    #[oai(path = "/npc", method = "get", operation_id = "get_random_npc")]
    async fn generate_character(&self) -> Result<CharacterGeneratedResponse> {
        let generator = npc_generator(generate_name());
        let non_player = generator.generate();

        let character_args = CharacterViewArgs {
            knows_health: true,
            knows_inventory: true,
            knows_hidden_in_inventory: true,
            knows_packed_in_inventory: true,
        };
        let view = non_player::view(&non_player, &character_args, true);

        let generated = GeneratedNpc { non_player: view };

        Ok(CharacterGeneratedResponse::CharacterGenerated(Json(
            generated,
        )))
    }
}
