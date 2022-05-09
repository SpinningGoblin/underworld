use poem_openapi::{Enum, Object};
use serde::Serialize;
use serde_json::Value;
use underworld_core::events::event::Event;

#[derive(Object, Serialize)]
pub struct GameEvent {
    pub name: EventName,

    pub data: Option<Value>,
}

#[derive(Serialize, Enum)]
#[oai(rename_all = "snake_case")]
pub enum EventName {
    DeadNpcBeaten,
    FixtureCanBeOpenedDiscovered,
    FixtureContainedDiscovered,
    FixtureHasHiddenDiscovered,
    FixtureHiddenItemsDiscovered,
    FixtureViewed,
    ItemTakenFromNpc,
    NpcHealthDiscovered,
    NpcHiddenDiscovered,
    NpcHit,
    NpcKilled,
    NpcMissed,
    NpcNameDiscovered,
    NpcPackedDiscovered,
    NpcViewed,
    NpcWeaponReadied,
    PlayerHit,
    PlayerItemMoved,
    PlayerKilled,
    PlayerMissed,
    RoomExited,
    RoomFirstSeen,
    RoomGenerated,
    RoomViewed,
}

impl From<Event> for GameEvent {
    fn from(event: Event) -> Self {
        match event {
            Event::NpcHit(it) => GameEvent {
                name: EventName::NpcHit,
                data: Some(serde_json::to_value(&it).unwrap()),
            },
            Event::NpcKilled(it) => GameEvent {
                name: EventName::NpcKilled,
                data: Some(serde_json::to_value(&it).unwrap()),
            },
            Event::NpcMissed(it) => GameEvent {
                name: EventName::NpcMissed,
                data: Some(serde_json::to_value(&it).unwrap()),
            },
            Event::PlayerHit(it) => GameEvent {
                name: EventName::PlayerHit,
                data: Some(serde_json::to_value(&it).unwrap()),
            },
            Event::PlayerKilled(it) => GameEvent {
                name: EventName::PlayerKilled,
                data: Some(serde_json::to_value(&it).unwrap()),
            },
            Event::PlayerMissed(it) => GameEvent {
                name: EventName::PlayerMissed,
                data: Some(serde_json::to_value(&it).unwrap()),
            },
            Event::RoomExited(it) => GameEvent {
                name: EventName::RoomExited,
                data: Some(serde_json::to_value(&it).unwrap()),
            },
            Event::RoomGenerated(it) => GameEvent {
                name: EventName::RoomGenerated,
                data: Some(serde_json::to_value(&it).unwrap()),
            },
            Event::ItemTakenFromNpc(it) => GameEvent {
                name: EventName::ItemTakenFromNpc,
                data: Some(serde_json::to_value(&it).unwrap()),
            },
            Event::NpcViewed(it) => GameEvent {
                name: EventName::NpcViewed,
                data: Some(serde_json::to_value(&it).unwrap()),
            },
            Event::DeadNpcBeaten(it) => GameEvent {
                name: EventName::DeadNpcBeaten,
                data: Some(serde_json::to_value(&it).unwrap()),
            },
            Event::NpcWeaponReadied(it) => GameEvent {
                name: EventName::NpcWeaponReadied,
                data: Some(serde_json::to_value(&it).unwrap()),
            },
            Event::PlayerItemMoved(it) => GameEvent {
                name: EventName::PlayerItemMoved,
                data: Some(serde_json::to_value(&it).unwrap()),
            },
            Event::NpcHealthDiscovered(it) => GameEvent {
                name: EventName::NpcHealthDiscovered,
                data: Some(serde_json::to_value(&it).unwrap()),
            },
            Event::NpcHiddenDiscovered(it) => GameEvent {
                name: EventName::NpcHiddenDiscovered,
                data: Some(serde_json::to_value(&it).unwrap()),
            },
            Event::NpcNameDiscovered(it) => GameEvent {
                name: EventName::NpcNameDiscovered,
                data: Some(serde_json::to_value(&it).unwrap()),
            },
            Event::NpcPackedDiscovered(it) => GameEvent {
                name: EventName::NpcPackedDiscovered,
                data: Some(serde_json::to_value(&it).unwrap()),
            },
            Event::FixtureCanBeOpenedDiscovered(it) => GameEvent {
                name: EventName::FixtureCanBeOpenedDiscovered,
                data: Some(serde_json::to_value(&it).unwrap()),
            },
            Event::FixtureContainedDiscovered(it) => GameEvent {
                name: EventName::FixtureContainedDiscovered,
                data: Some(serde_json::to_value(&it).unwrap()),
            },
            Event::FixtureHasHiddenDiscovered(it) => GameEvent {
                name: EventName::FixtureHasHiddenDiscovered,
                data: Some(serde_json::to_value(&it).unwrap()),
            },
            Event::FixtureHiddenItemsDiscovered(it) => GameEvent {
                name: EventName::FixtureHiddenItemsDiscovered,
                data: Some(serde_json::to_value(&it).unwrap()),
            },
            Event::FixtureViewed(it) => GameEvent {
                name: EventName::FixtureViewed,
                data: Some(serde_json::to_value(&it).unwrap()),
            },
            Event::RoomViewed(it) => GameEvent {
                name: EventName::RoomViewed,
                data: Some(serde_json::to_value(&it).unwrap()),
            },
            Event::RoomFirstSeen(it) => GameEvent {
                name: EventName::RoomFirstSeen,
                data: Some(serde_json::to_value(&it).unwrap()),
            },
        }
    }
}
