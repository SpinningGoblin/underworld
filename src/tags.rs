use poem_openapi::Tags;

#[derive(Tags)]
pub enum UnderworldApiTags {
    Npcs,
    PlayerCharacters,
    Games,
    GameActions,
}
