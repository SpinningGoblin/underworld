use poem::{web::Data, Result};
use poem_openapi::{param::Path, payload::Json, ApiResponse, OpenApi};
use sqlx::PgPool;
use underworld_core::components::{
    fixtures::fixture::FixtureView, non_player::NonPlayerView, rooms::room_view::RoomView,
};

use crate::{
    actions::PerformAction,
    game::{
        attack::{attack_npc, AttackNpcArgs, NpcAttacked},
        exit::{exit_room, ExitRoomArgs, RoomExited},
        generate::{generate_game, GenerateGameArgs, GeneratedGame},
        get::{game_actions, game_ids, GameActionsArgs},
        look::{
            look_at_fixture, look_at_npc, look_at_room, FixtureLookArgs, NpcLookArgs, RoomLookArgs,
        },
        loot::{loot_npc, LootNpcArgs, NpcLooted},
    },
};
use crate::{
    game::inspect::{
        inspect_fixture, inspect_npc, FixtureInspected, InspectFixtureArgs, InspectNpcArgs,
        NpcInspected,
    },
    tags::UnderworldApiTags,
};

#[derive(ApiResponse)]
enum LookResponse {
    #[oai(status = 200)]
    LookAtRoom(Json<RoomView>),
}

#[derive(ApiResponse)]
enum LookFixtureResponse {
    #[oai(status = 200)]
    FixtureViewed(Json<FixtureView>),
}

#[derive(ApiResponse)]
enum LookNpcResponse {
    #[oai(status = 200)]
    NpcViewed(Json<NonPlayerView>),
}

#[derive(ApiResponse)]
enum InspectNpcResponse {
    #[oai(status = 200)]
    NpcInspected(Json<NpcInspected>),
}

#[derive(ApiResponse)]
enum InspectFixtureResponse {
    #[oai(status = 200)]
    FixtureInspected(Json<FixtureInspected>),
}

#[derive(ApiResponse)]
enum LootNpcResponse {
    #[oai(status = 200)]
    NpcLooted(Json<NpcLooted>),
}

#[derive(ApiResponse)]
pub enum GenerateGameResponse {
    #[oai(status = 201)]
    GameGenerated(Json<GeneratedGame>),
}

#[derive(ApiResponse)]
enum AttackNpcResponse {
    #[oai(status = 200)]
    NpcAttacked(Json<NpcAttacked>),
}

#[derive(ApiResponse)]
enum ExitRoomResponse {
    #[oai(status = 200)]
    RoomExited(Json<RoomExited>),
}

#[derive(ApiResponse)]
enum GameIdResponse {
    #[oai(status = 200)]
    GameIds(Json<Vec<String>>),
}

#[derive(ApiResponse)]
enum GameActionsResponse {
    #[oai(status = 200)]
    GameActions(Json<Vec<PerformAction>>),
}

pub struct UnderworldGameApi;

#[OpenApi(tag = "UnderworldApiTags::Game")]
impl UnderworldGameApi {
    /// Generate and persist a new game.
    ///
    /// # Example
    ///
    /// Call `/game/generate` with
    /// ```
    /// {
    ///   "username": "my_username"
    /// }
    /// ```
    /// to generate and save a new game for my_username
    #[oai(
        path = "/game/generate",
        method = "post",
    )]
    async fn generate_game(
        &self,
        pool: Data<&PgPool>,
        args: Json<GenerateGameArgs>,
    ) -> Result<GenerateGameResponse> {
        let mut transaction = pool.0.begin().await.unwrap();
        let generated_result = generate_game(&mut transaction, &args).await.unwrap();
        transaction.commit().await.unwrap();

        Ok(GenerateGameResponse::GameGenerated(Json(generated_result)))
    }

    /// Get IDs of all current games
    ///
    /// # Example
    ///
    /// Call `/my_username/games` to retrieve all game ids for my_username
    #[oai(
        path = "/:username/games",
        method = "get",
    )]
    async fn list_games(
        &self,
        pool: Data<&PgPool>,
        username: Path<String>,
    ) -> Result<GameIdResponse> {
        let mut transaction = pool.0.begin().await.unwrap();
        let result = game_ids(&mut transaction, &username).await;
        transaction.commit().await.unwrap();
        Ok(GameIdResponse::GameIds(Json(result)))
    }

    /// Exit the current room of the specified game through the specified exit.
    #[oai(
        path = "/game/exit_room",
        method = "post",
    )]
    async fn exit_room(
        &self,
        pool: Data<&PgPool>,
        args: Json<ExitRoomArgs>,
    ) -> Result<ExitRoomResponse> {
        let mut transaction = pool.0.begin().await.unwrap();
        let exit_result = exit_room(&mut transaction, &args).await.unwrap();
        transaction.commit().await.unwrap();
        Ok(ExitRoomResponse::RoomExited(Json(exit_result)))
    }

    /// Attack a specific NPC inside the current room of the specified game.
    #[oai(
        path = "/game/attack_npc",
        method = "post",
    )]
    async fn attack_npc(
        &self,
        pool: Data<&PgPool>,
        args: Json<AttackNpcArgs>,
    ) -> Result<AttackNpcResponse> {
        let mut transaction = pool.0.begin().await.unwrap();
        let attack_result = attack_npc(&mut transaction, &args).await.unwrap();
        transaction.commit().await.unwrap();

        Ok(AttackNpcResponse::NpcAttacked(Json(attack_result)))
    }

    /// Loot some items from an NPC.
    #[oai(
        path = "/game/loot_npc",
        method = "post",
    )]
    async fn loot_npc(
        &self,
        pool: Data<&PgPool>,
        args: Json<LootNpcArgs>,
    ) -> Result<LootNpcResponse> {
        let mut transaction = pool.0.begin().await.unwrap();
        let loot_result = loot_npc(&mut transaction, &args).await.unwrap();
        transaction.commit().await.unwrap();

        Ok(LootNpcResponse::NpcLooted(Json(loot_result)))
    }

    /// Take a closer look at the current room.
    #[oai(
        path = "/game/look_around_room",
        method = "post",
    )]
    async fn look_around_room(
        &self,
        pool: Data<&PgPool>,
        args: Json<RoomLookArgs>,
    ) -> Result<LookResponse> {
        let mut transaction = pool.0.begin().await.unwrap();
        let view_result = look_at_room(&mut transaction, &args).await.unwrap();
        transaction.commit().await.unwrap();

        Ok(LookResponse::LookAtRoom(Json(view_result)))
    }

    /// Look at a specific Fixture in the current room.
    #[oai(
        path = "/game/look_at_fixture",
        method = "post",
    )]
    async fn look_at_fixture(
        &self,
        pool: Data<&PgPool>,
        args: Json<FixtureLookArgs>,
    ) -> Result<LookFixtureResponse> {
        let mut transaction = pool.0.begin().await.unwrap();
        let view = look_at_fixture(&mut transaction, &args).await.unwrap();
        transaction.commit().await.unwrap();
        Ok(LookFixtureResponse::FixtureViewed(Json(view)))
    }

    /// Look at a specific NPC in the current room.
    #[oai(
        path = "/game/look_at_npc",
        method = "post",
    )]
    async fn look_at_npc(
        &self,
        pool: Data<&PgPool>,
        args: Json<NpcLookArgs>,
    ) -> Result<LookNpcResponse> {
        let mut transaction = pool.0.begin().await.unwrap();
        let view = look_at_npc(&mut transaction, &args).await.unwrap();
        transaction.commit().await.unwrap();
        Ok(LookNpcResponse::NpcViewed(Json(view)))
    }

    /// Inspect a fixture to find out more information about them when looking at them next.
    /// After completing an inspect, look at the fixture to see new information.
    #[oai(
        path = "/game/inspect_fixture",
        method = "post",
    )]
    async fn inspect_fixture(
        &self,
        pool: Data<&PgPool>,
        args: Json<InspectFixtureArgs>,
    ) -> Result<InspectFixtureResponse> {
        let mut transaction = pool.0.begin().await.unwrap();
        let inspection = inspect_fixture(&mut transaction, &args).await.unwrap();
        transaction.commit().await.unwrap();
        Ok(InspectFixtureResponse::FixtureInspected(Json(inspection)))
    }

    /// Inspect an NPC to find out more information about them when looking at them next.
    /// After completing an inspect, look at the NPC to see new information.
    #[oai(
        path = "/game/inspect_npc",
        method = "post",
    )]
    async fn inspect_npc(
        &self,
        pool: Data<&PgPool>,
        args: Json<InspectNpcArgs>,
    ) -> Result<InspectNpcResponse> {
        let mut transaction = pool.0.begin().await.unwrap();
        let inspection = inspect_npc(&mut transaction, &args).await.unwrap();
        transaction.commit().await.unwrap();
        Ok(InspectNpcResponse::NpcInspected(Json(inspection)))
    }

    /// Get the current actions available for the game.
    #[oai(
        path = "/game/current_actions",
        method = "post",
    )]
    async fn current_actions(
        &self,
        pool: Data<&PgPool>,
        args: Json<GameActionsArgs>,
    ) -> Result<GameActionsResponse> {
        let mut transaction = pool.0.begin().await.unwrap();
        let actions = game_actions(&mut transaction, &args).await.unwrap();
        Ok(GameActionsResponse::GameActions(Json(actions)))
    }
}
