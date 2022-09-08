use std::collections::HashMap;

use crate::tags::UnderworldApiTags;
use poem::Result;
use poem_openapi::{payload::Json, ApiResponse, Object, OpenApi};
use serde::{Deserialize, Serialize};
use underworld_core::{
    components::{
        rooms::{self, Dimensions, ExitType, Flavour, RoomType, RoomView},
        CharacterViewArgs, LifeModifier, NonPlayerView, Species,
    },
    generators::{
        generator::Generator, non_players::NonPlayerGeneratorBuilder, ExitGenerationArgs,
        RoomGeneratorBuilder,
    },
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

#[derive(Object, Deserialize)]
struct GenerateRoomsRequest {
    pub room_args: Vec<GenerateSingleRoom>,
}

#[derive(Object, Serialize)]
struct GeneratedRooms {
    pub rooms: Vec<RoomView>,
}

/// An inclusive range of integers.
#[derive(Object, Deserialize)]
pub struct InclusiveRange {
    pub min: u16,
    pub max_inclusive: u16,
}

/// Args to tweak the exit generation inside of a room.
#[derive(Object, Deserialize)]
pub struct RoomExitsGenerationArgs {
    /// Set this to define a range for how many exits should be generated.
    pub num_exits: Option<InclusiveRange>,
    /// What type of exits should be put in the room.
    pub possible_exit_types: Option<Vec<ExitType>>,
}

/// Args to modify the NPC generation inside of the room.
#[derive(Object, Deserialize)]
pub struct RoomNpcGenerationArgs {
    /// How many groups of NPCs should be generated.
    /// The number of NPCs in each group will be random and influenced
    /// by the species that get chosen.
    pub num_groups: Option<InclusiveRange>,
    /// If you want to limit the species that can spawn, set them here.
    /// Otherwise all species will be used.
    pub possible_species: Option<Vec<Species>>,
    /// Limit the life modifiers that NPCs can be.
    /// This does not guarantee that NPCs will spawn with these modifiers.
    pub possible_life_modifiers: Option<Vec<LifeModifier>>,
    /// If you'd like NPCs to not spawn already killed, set this to false.
    /// Defaults to true.
    pub allow_npcs_to_spawn_dead: Option<bool>,
}

/// Args to tweak the room generation.
#[derive(Object, Deserialize)]
struct GenerateSingleRoom {
    /// Set this to specify a particular room type.
    pub room_type: Option<RoomType>,
    /// Set this to give a range for the number of descriptors for the room.
    /// If min/max are same, the room will get that many.
    pub num_descriptors: Option<InclusiveRange>,
    /// The possible descriptors that can be put onto the room.
    pub possible_descriptors: Option<Vec<rooms::Descriptor>>,
    /// Set this to set the danger level for this room.
    /// This will impact the health, and equipment on any NPCs and in fixtures.
    pub danger_level: Option<u32>,
    /// Set these if you want to inform the exit generation.
    pub exit_generation_args: Option<RoomExitsGenerationArgs>,
    /// Set this if you would like to set the dimensions for the room.
    pub dimensions: Option<Dimensions>,
    /// Set this if you'd like to set a name for the room.
    pub name: Option<String>,
    /// Set this to give options for flavour text for the room.
    /// If not set, the default ones for the room type will be used.
    pub possible_flavour_texts: Option<Vec<Flavour>>,
    /// Set this to include or not include flavour text.
    /// If not set, it will be included.
    pub include_flavour_text: Option<bool>,
    /// Set these options if you want to change any base values for NPC generation.
    pub room_npcs_generation_args: Option<RoomNpcGenerationArgs>,
}

#[derive(ApiResponse)]
enum RoomsGeneratedResponse {
    #[oai(status = 200)]
    RoomsGenerated(Json<GeneratedRooms>),
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
        let generator = NonPlayerGeneratorBuilder::new().build();
        let non_player = generator.generate();

        let character_args = CharacterViewArgs::knows_all_args();
        let view = non_player::view(&non_player, &character_args, true);

        let generated = GeneratedNpc { non_player: view };

        Ok(CharacterGeneratedResponse::CharacterGenerated(Json(
            generated,
        )))
    }

    /// Generate a random room with NPCs and fixtures inside.
    ///
    /// # Example
    ///
    /// Call `/random/rooms` to generate one or more rooms.
    #[oai(path = "/rooms", method = "post", operation_id = "get_random_rooms")]
    async fn generate_rooms(
        &self,
        args: Json<GenerateRoomsRequest>,
    ) -> Result<RoomsGeneratedResponse> {
        let rooms: Vec<RoomView> = args
            .room_args
            .iter()
            .map(|room_args| {
                let mut builder = RoomGeneratorBuilder::new();

                if let Some(room_type) = &room_args.room_type {
                    builder.room_type(*room_type);
                }

                if let Some(num_descriptors) = &room_args.num_descriptors {
                    builder.num_descriptors(num_descriptors.min..=num_descriptors.max_inclusive);
                }

                if let Some(possible_descriptors) = &room_args.possible_descriptors {
                    builder.possible_descriptors(possible_descriptors.clone());
                }

                if let Some(danger_level) = &room_args.danger_level {
                    builder.danger_level(*danger_level);
                }

                if let Some(room_exit_args) = &room_args.exit_generation_args {
                    let num_exits = room_exit_args
                        .num_exits
                        .as_ref()
                        .map(|inclusive_range| inclusive_range.min..=inclusive_range.max_inclusive);

                    let exit_generation_args = ExitGenerationArgs {
                        num_exits,
                        possible_exit_types: room_exit_args.possible_exit_types.clone(),
                    };

                    builder.exit_generation_args(exit_generation_args);
                }

                if let Some(room_npc_args) = &room_args.room_npcs_generation_args {
                    let num_groups = room_npc_args
                        .num_groups
                        .as_ref()
                        .map(|inclusive_range| inclusive_range.min..=inclusive_range.max_inclusive);

                    let core_room_npc_args = underworld_core::generators::RoomNpcGenerationArgs {
                        num_groups,
                        possible_species: room_npc_args.possible_species.clone(),
                        possible_life_modifiers: room_npc_args.possible_life_modifiers.clone(),
                        allow_npcs_to_spawn_dead: room_npc_args.allow_npcs_to_spawn_dead,
                    };

                    builder.room_npc_generation_args(core_room_npc_args);
                }

                if let Some(dimensions) = &room_args.dimensions {
                    builder.dimensions(dimensions.clone());
                }

                if let Some(name) = &room_args.name {
                    builder.name(name);
                }

                if let Some(possible_flavour_texts) = &room_args.possible_flavour_texts {
                    builder.possible_flavour_texts(possible_flavour_texts.clone());
                }

                if let Some(include_flavour_text) = &room_args.include_flavour_text {
                    builder.include_flavour_text(*include_flavour_text);
                }

                let room = builder.build().generate();
                underworld_core::systems::view::room::view(
                    &room,
                    HashMap::new(),
                    HashMap::new(),
                    HashMap::new(),
                    true,
                )
            })
            .collect();

        Ok(RoomsGeneratedResponse::RoomsGenerated(Json(
            GeneratedRooms { rooms },
        )))
    }
}
