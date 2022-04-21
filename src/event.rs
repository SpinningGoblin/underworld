use poem_openapi::Object;
use serde::Serialize;
use serde_json::Value;
use underworld_core::events::event::Event;

#[derive(Object, Serialize)]
pub struct GameEvent {
    pub name: String,
    pub data: Option<Value>,
}

impl From<Event> for GameEvent {
    fn from(event: Event) -> Self {
        match event {
            Event::NpcHit(it) => GameEvent {
                name: "npc_hit".to_string(),
                data: Some(serde_json::to_value(&it).unwrap()),
            },
            Event::NpcKilled(it) => GameEvent {
                name: "npc_killed".to_string(),
                data: Some(serde_json::to_value(&it).unwrap()),
            },
            Event::NpcMissed(it) => GameEvent {
                name: "npc_missed".to_string(),
                data: Some(serde_json::to_value(&it).unwrap()),
            },
            Event::PlayerHit(it) => GameEvent {
                name: "player_hit".to_string(),
                data: Some(serde_json::to_value(&it).unwrap()),
            },
            Event::PlayerKilled(it) => GameEvent {
                name: "player_killed".to_string(),
                data: Some(serde_json::to_value(&it).unwrap()),
            },
            Event::PlayerMissed(it) => GameEvent {
                name: "player_missed".to_string(),
                data: Some(serde_json::to_value(&it).unwrap()),
            },
            Event::RoomExited(it) => GameEvent {
                name: "room_exited".to_string(),
                data: Some(serde_json::to_value(&it).unwrap()),
            },
            Event::RoomGenerated(it) => GameEvent {
                name: "room_generated".to_string(),
                data: Some(serde_json::to_value(&it).unwrap()),
            },
        }
    }
}
