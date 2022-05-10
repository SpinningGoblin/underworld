use poem_openapi::Tags;

#[derive(Tags)]
pub enum UnderworldApiTags {
    Npc,
    PlayerCharacter,
    Game,
}
