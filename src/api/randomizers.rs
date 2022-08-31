use std::collections::HashMap;

use crate::tags::UnderworldApiTags;
use poem::Result;
use poem_openapi::{payload::Json, ApiResponse, Object, OpenApi};
use serde::{Deserialize, Serialize};
use underworld_core::{
    components::{
        rooms::{self, Dimensions, ExitType, Flavour, RoomType, RoomView},
        CharacterViewArgs, NonPlayerView,
    },
    generators::{
        generator::Generator, name::generate_name, non_players::npc_generator, ExitGenerationArgs,
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

    /// Generate a random NPC.
    ///
    /// # Example
    ///
    /// Call `/npc/random` to generate a completely random character
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
                    builder.room_type(room_type.clone());
                }

                if let Some(num_descriptors) = &room_args.num_descriptors {
                    builder.num_descriptors(num_descriptors.min..=num_descriptors.max_inclusive);
                }

                if let Some(possible_descriptors) = &room_args.possible_descriptors {
                    builder.possible_descriptors(possible_descriptors.clone());
                }

                if let Some(danger_level) = &room_args.danger_level {
                    builder.danger_level(danger_level.clone());
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

                if let Some(dimensions) = &room_args.dimensions {
                    builder.dimensions(dimensions.clone());
                }

                if let Some(name) = &room_args.name {
                    builder.name(&name);
                }

                if let Some(possible_flavour_texts) = &room_args.possible_flavour_texts {
                    builder.possible_flavour_texts(possible_flavour_texts.clone());
                }

                if let Some(include_flavour_text) = &room_args.include_flavour_text {
                    builder.include_flavour_text(include_flavour_text.clone());
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
