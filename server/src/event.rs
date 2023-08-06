use poem_openapi::{Enum, Object};
use serde::Serialize;
use serde_json::Value;
use underworld_core::events::Event;

#[derive(Object, Serialize)]
pub struct GameEvent {
    pub name: EventName,

    pub data: Option<Value>,
}

#[derive(Serialize, Enum)]
#[oai(rename_all = "snake_case")]
pub enum EventName {
    DeadNpcBeaten,
    FixtureHasHiddenCompartmentDiscovered,
    FixtureHiddenCompartmentOpened,
    FixtureOpened,
    FixtureViewed,
    GameDangerLevelIncreased,
    GhostEscapesToTheVoid,
    ItemTakenFromFixture,
    ItemTakenFromNpc,
    NpcCoveredInOil,
    NpcDamagedByPoison,
    NpcHealthDiscovered,
    NpcItemDestroyed,
    NpcHitWithAcid,
    NpcMissed,
    NpcPoisoned,
    NpcPackedDiscovered,
    NpcPoisonDurationChanged,
    NpcPoisonEffectDissipated,
    NpcPoisonLevelChanged,
    NpcViewed,
    NpcWeaponReadied,
    PlayerDamagedByPoison,
    PlayerDropsAllItems,
    PlayerGainedGold,
    PlayerGainsResurrectionAura,
    PlayerGainsRetributionAura,
    PlayerGainsShieldAura,
    PlayerHealed,
    PlayerHit,
    PlayerHitNpc,
    PlayerHitWithAcid,
    PlayerMaxHealthChanged,
    PlayerPicksUpItem,
    PlayerPoisonLevelChanged,
    PlayerPoisoned,
    PlayerPoisonDurationChanged,
    PlayerShieldAuraDamaged,
    PlayerShieldAuraDissipated,
    PlayerHealthFullyRestored,
    PlayerItemMoved,
    PlayerItemDestroyed,
    PlayerItemRemoved,
    PlayerItemUsed,
    PlayerKilled,
    PlayerKilledNpc,
    PlayerMissed,
    PlayerPoisonDissipated,
    PlayerResurrected,
    PlayerRetributionAuraDissipated,
    PlayerSpellForgotten,
    PlayerSpellLearned,
    PlayerSpellUsed,
    RoomExited,
    RoomFirstSeen,
    RoomGenerated,
}

impl From<Event> for GameEvent {
    fn from(event: Event) -> Self {
        match event {
            Event::PlayerHitNpc(it) => GameEvent {
                name: EventName::PlayerHitNpc,
                data: Some(serde_json::to_value(it).unwrap()),
            },
            Event::PlayerKilledNpc(it) => GameEvent {
                name: EventName::PlayerKilledNpc,
                data: Some(serde_json::to_value(it).unwrap()),
            },
            Event::NpcMissed(it) => GameEvent {
                name: EventName::NpcMissed,
                data: Some(serde_json::to_value(it).unwrap()),
            },
            Event::PlayerHit(it) => GameEvent {
                name: EventName::PlayerHit,
                data: Some(serde_json::to_value(it).unwrap()),
            },
            Event::PlayerKilled(it) => GameEvent {
                name: EventName::PlayerKilled,
                data: Some(serde_json::to_value(it).unwrap()),
            },
            Event::PlayerMissed(it) => GameEvent {
                name: EventName::PlayerMissed,
                data: Some(serde_json::to_value(it).unwrap()),
            },
            Event::RoomExited(it) => GameEvent {
                name: EventName::RoomExited,
                data: Some(serde_json::to_value(it).unwrap()),
            },
            Event::RoomGenerated(it) => GameEvent {
                name: EventName::RoomGenerated,
                data: Some(serde_json::to_value(it).unwrap()),
            },
            Event::ItemTakenFromNpc(it) => GameEvent {
                name: EventName::ItemTakenFromNpc,
                data: Some(serde_json::to_value(it).unwrap()),
            },
            Event::NpcViewed(it) => GameEvent {
                name: EventName::NpcViewed,
                data: Some(serde_json::to_value(it).unwrap()),
            },
            Event::DeadNpcBeaten(it) => GameEvent {
                name: EventName::DeadNpcBeaten,
                data: Some(serde_json::to_value(it).unwrap()),
            },
            Event::NpcWeaponReadied(it) => GameEvent {
                name: EventName::NpcWeaponReadied,
                data: Some(serde_json::to_value(it).unwrap()),
            },
            Event::PlayerItemMoved(it) => GameEvent {
                name: EventName::PlayerItemMoved,
                data: Some(serde_json::to_value(it).unwrap()),
            },
            Event::NpcHealthDiscovered(it) => GameEvent {
                name: EventName::NpcHealthDiscovered,
                data: Some(serde_json::to_value(it).unwrap()),
            },
            Event::NpcPackedDiscovered(it) => GameEvent {
                name: EventName::NpcPackedDiscovered,
                data: Some(serde_json::to_value(it).unwrap()),
            },
            Event::FixtureHasHiddenCompartmentDiscovered(it) => GameEvent {
                name: EventName::FixtureHasHiddenCompartmentDiscovered,
                data: Some(serde_json::to_value(it).unwrap()),
            },
            Event::FixtureViewed(it) => GameEvent {
                name: EventName::FixtureViewed,
                data: Some(serde_json::to_value(it).unwrap()),
            },
            Event::RoomFirstSeen(it) => GameEvent {
                name: EventName::RoomFirstSeen,
                data: Some(serde_json::to_value(it).unwrap()),
            },
            Event::ItemTakenFromFixture(it) => GameEvent {
                name: EventName::ItemTakenFromFixture,
                data: Some(serde_json::to_value(it).unwrap()),
            },
            Event::PlayerGainsResurrectionAura => GameEvent {
                name: EventName::PlayerGainsResurrectionAura,
                data: None,
            },
            Event::PlayerGainsRetributionAura(it) => GameEvent {
                name: EventName::PlayerGainsRetributionAura,
                data: Some(serde_json::to_value(it).unwrap()),
            },
            Event::PlayerGainsShieldAura(it) => GameEvent {
                name: EventName::PlayerGainsShieldAura,
                data: Some(serde_json::to_value(it).unwrap()),
            },
            Event::PlayerHealed(it) => GameEvent {
                name: EventName::PlayerHealed,
                data: Some(serde_json::to_value(it).unwrap()),
            },
            Event::PlayerItemRemoved(it) => GameEvent {
                name: EventName::PlayerItemRemoved,
                data: Some(serde_json::to_value(it).unwrap()),
            },
            Event::PlayerItemUsed(it) => GameEvent {
                name: EventName::PlayerItemUsed,
                data: Some(serde_json::to_value(it).unwrap()),
            },
            Event::PlayerResurrected => GameEvent {
                name: EventName::PlayerResurrected,
                data: None,
            },
            Event::PlayerRetributionAuraDissipated => GameEvent {
                name: EventName::PlayerRetributionAuraDissipated,
                data: None,
            },
            Event::PlayerSpellForgotten(it) => GameEvent {
                name: EventName::PlayerSpellForgotten,
                data: Some(serde_json::to_value(it).unwrap()),
            },
            Event::PlayerSpellLearned(it) => GameEvent {
                name: EventName::PlayerSpellLearned,
                data: Some(serde_json::to_value(it).unwrap()),
            },
            Event::PlayerSpellUsed(it) => GameEvent {
                name: EventName::PlayerSpellUsed,
                data: Some(serde_json::to_value(it).unwrap()),
            },
            Event::GameDangerLevelIncreased(_) => GameEvent {
                name: EventName::GameDangerLevelIncreased,
                data: None,
            },
            Event::FixtureHiddenCompartmentOpened(it) => GameEvent {
                name: EventName::FixtureHiddenCompartmentOpened,
                data: Some(serde_json::to_value(it).unwrap()),
            },
            Event::FixtureOpened(it) => GameEvent {
                name: EventName::FixtureOpened,
                data: Some(serde_json::to_value(it).unwrap()),
            },
            Event::NpcDamagedByPoison(it) => GameEvent {
                name: EventName::NpcDamagedByPoison,
                data: Some(serde_json::to_value(it).unwrap()),
            },
            Event::NpcPoisonDurationChanged(it) => GameEvent {
                name: EventName::NpcPoisonDurationChanged,
                data: Some(serde_json::to_value(it).unwrap()),
            },
            Event::NpcPoisonLevelChanged(it) => GameEvent {
                name: EventName::NpcPoisonLevelChanged,
                data: Some(serde_json::to_value(it).unwrap()),
            },
            Event::NpcPoisoned(it) => GameEvent {
                name: EventName::NpcPoisoned,
                data: Some(serde_json::to_value(it).unwrap()),
            },
            Event::PlayerDamagedByPoison(it) => GameEvent {
                name: EventName::PlayerDamagedByPoison,
                data: Some(serde_json::to_value(it).unwrap()),
            },
            Event::PlayerMaxHealthChanged(it) => GameEvent {
                name: EventName::PlayerMaxHealthChanged,
                data: Some(serde_json::to_value(it).unwrap()),
            },
            Event::PlayerPoisonLevelChanged(it) => GameEvent {
                name: EventName::PlayerPoisonLevelChanged,
                data: Some(serde_json::to_value(it).unwrap()),
            },
            Event::PlayerPoisoned(it) => GameEvent {
                name: EventName::PlayerPoisoned,
                data: Some(serde_json::to_value(it).unwrap()),
            },
            Event::PlayerPoisonDurationChanged(it) => GameEvent {
                name: EventName::PlayerPoisonDurationChanged,
                data: Some(serde_json::to_value(it).unwrap()),
            },
            Event::PlayerHealthFullyRestored => GameEvent {
                name: EventName::PlayerHealthFullyRestored,
                data: None,
            },
            Event::PlayerGainedGold(gold) => GameEvent {
                name: EventName::PlayerGainedGold,
                data: Some(serde_json::to_value(gold).unwrap()),
            },
            Event::PlayerShieldAuraDamaged(damage) => GameEvent {
                name: EventName::PlayerShieldAuraDamaged,
                data: Some(serde_json::to_value(damage).unwrap()),
            },
            Event::PlayerShieldAuraDissipated => GameEvent {
                name: EventName::PlayerShieldAuraDissipated,
                data: None,
            },
            Event::NpcPoisonEffectDissipated(dissipated) => GameEvent {
                name: EventName::NpcPoisonEffectDissipated,
                data: Some(serde_json::to_value(dissipated).unwrap()),
            },
            Event::NpcHitWithAcid(it) => GameEvent {
                name: EventName::NpcHitWithAcid,
                data: Some(serde_json::to_value(it).unwrap()),
            },
            Event::NpcItemDestroyed(it) => GameEvent {
                name: EventName::NpcItemDestroyed,
                data: Some(serde_json::to_value(it).unwrap()),
            },
            Event::PlayerHitWithAcid => GameEvent {
                name: EventName::PlayerHitWithAcid,
                data: None,
            },
            Event::PlayerItemDestroyed(it) => GameEvent {
                name: EventName::PlayerItemDestroyed,
                data: Some(serde_json::to_value(it).unwrap()),
            },
            Event::PlayerPoisonDissipated => GameEvent {
                name: EventName::PlayerPoisonDissipated,
                data: None,
            },
            Event::NpcCoveredInOil(id) => GameEvent {
                name: EventName::NpcCoveredInOil,
                data: Some(serde_json::to_value(id).unwrap()),
            },
            Event::GhostEscapesToTheVoid(escapes) => GameEvent {
                name: EventName::GhostEscapesToTheVoid,
                data: Some(serde_json::to_value(escapes).unwrap()),
            },
            Event::PlayerDropsAllItems => GameEvent {
                name: EventName::PlayerDropsAllItems,
                data: None,
            },
            Event::PlayerPicksUpItem(id) => GameEvent {
                name: EventName::PlayerPicksUpItem,
                data: Some(serde_json::to_value(id).unwrap()),
            },
        }
    }
}
