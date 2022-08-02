use poem_openapi::Tags;

#[derive(Tags)]
pub enum UnderworldApiTags {
    Auth,
    Npcs,
    PlayerCharacters,
    Games,
    GameActions,
}
