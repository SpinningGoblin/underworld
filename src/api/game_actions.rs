use poem::{web::Data, Result};
use poem_openapi::{payload::Json, ApiResponse, OpenApi};
use sqlx::PgPool;
use underworld_core::components::{
    fixtures::fixture::FixtureView, non_player::NonPlayerView, rooms::room_view::RoomView,
};

use crate::{
    actions::PerformAction,
    game::{
        attack::{attack_npc, AttackNpcArgs, NpcAttacked},
        exit::{exit_room, ExitRoomArgs, RoomExited},
        get::{game_actions, GameActionsArgs},
        inspect::{
            inspect_fixture, inspect_npc, FixtureInspected, InspectFixtureArgs, InspectNpcArgs,
            NpcInspected,
        },
        items::{
            move_player_item, use_item_on_player, ItemMoved, ItemUsed, MovePlayerItemArgs,
            UseItemOnPlayerArgs,
        },
        look::{
            look_at_fixture, look_at_npc, look_at_room, FixtureLookArgs, NpcLookArgs, RoomLookArgs,
        },
        loot::{loot_fixture, loot_npc, FixtureLooted, LootFixtureArgs, LootNpcArgs, NpcLooted},
        spells::{cast_spell_on_player, CastSpellOnPlayerArgs, SpellCast},
    },
};

use crate::tags::UnderworldApiTags;

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
enum LootFixtureResponse {
    #[oai(status = 200)]
    FixtureLooted(Json<FixtureLooted>),
}

#[derive(ApiResponse)]
enum AttackNpcResponse {
    #[oai(status = 200)]
    NpcAttacked(Json<NpcAttacked>),
}

#[derive(ApiResponse)]
enum CastSpellResponse {
    #[oai(status = 200)]
    SpellCast(Json<SpellCast>),
}

#[derive(ApiResponse)]
enum UseItemResponse {
    #[oai(status = 200)]
    ItemUsed(Json<ItemUsed>),
}

#[derive(ApiResponse)]
enum MoveItemResponse {
    #[oai(status = 200)]
    ItemMoved(Json<ItemMoved>),
}

#[derive(ApiResponse)]
enum ExitRoomResponse {
    #[oai(status = 200)]
    RoomExited(Json<RoomExited>),
}

#[derive(ApiResponse)]
enum GameActionsResponse {
    #[oai(status = 200)]
    GameActions(Json<Vec<PerformAction>>),
}

pub struct UnderworldGameActionApi;

#[OpenApi(tag = "UnderworldApiTags::GameActions", prefix_path = "/game/action/")]
impl UnderworldGameActionApi {
    /// Exit the current room of the specified game through the specified exit.
    #[oai(path = "/exit_room", method = "post")]
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
    #[oai(path = "/attack_npc", method = "post")]
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

    /// Cast a spell on your player character.
    #[oai(path = "/cast_spell_on_player", method = "post")]
    async fn cast_spell_on_player(
        &self,
        pool: Data<&PgPool>,
        args: Json<CastSpellOnPlayerArgs>,
    ) -> Result<CastSpellResponse> {
        let mut transaction = pool.0.begin().await.unwrap();
        let cast_result = cast_spell_on_player(&mut transaction, &args).await.unwrap();
        transaction.commit().await.unwrap();

        Ok(CastSpellResponse::SpellCast(Json(cast_result)))
    }

    /// Use an item on your player character.
    #[oai(path = "/use_item_on_player", method = "post")]
    async fn use_item_on_player(
        &self,
        pool: Data<&PgPool>,
        args: Json<UseItemOnPlayerArgs>,
    ) -> Result<UseItemResponse> {
        let mut transaction = pool.0.begin().await.unwrap();
        let use_item_result = use_item_on_player(&mut transaction, &args).await.unwrap();
        transaction.commit().await.unwrap();

        Ok(UseItemResponse::ItemUsed(Json(use_item_result)))
    }

    /// Use an item on your player character.
    #[oai(path = "/move_player_item", method = "post")]
    async fn move_player_item(
        &self,
        pool: Data<&PgPool>,
        args: Json<MovePlayerItemArgs>,
    ) -> Result<MoveItemResponse> {
        let mut transaction = pool.0.begin().await.unwrap();
        let move_item_result = move_player_item(&mut transaction, &args).await.unwrap();
        transaction.commit().await.unwrap();

        Ok(MoveItemResponse::ItemMoved(Json(move_item_result)))
    }

    /// Loot some items from an NPC.
    #[oai(path = "/loot_npc", method = "post")]
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

    /// Loot some items from a fixture.
    #[oai(path = "/loot_fixture", method = "post")]
    async fn loot_fixture(
        &self,
        pool: Data<&PgPool>,
        args: Json<LootFixtureArgs>,
    ) -> Result<LootFixtureResponse> {
        let mut transaction = pool.0.begin().await.unwrap();
        let loot_result = loot_fixture(&mut transaction, &args).await.unwrap();
        transaction.commit().await.unwrap();

        Ok(LootFixtureResponse::FixtureLooted(Json(loot_result)))
    }

    /// Take a closer look at the current room.
    #[oai(path = "/look_around_room", method = "post")]
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
    #[oai(path = "/look_at_fixture", method = "post")]
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
    #[oai(path = "/look_at_npc", method = "post")]
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
    #[oai(path = "/inspect_fixture", method = "post")]
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
    #[oai(path = "/inspect_npc", method = "post")]
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
    #[oai(path = "/current_actions", method = "post")]
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
