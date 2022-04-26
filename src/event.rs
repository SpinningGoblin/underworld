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
    ItemTakenFromNpc,
    NpcHit,
    NpcKilled,
    NpcMissed,
    NpcViewed,
    PlayerHit,
    PlayerKilled,
    PlayerMissed,
    RoomExited,
    RoomGenerated,
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
        }
    }
}
