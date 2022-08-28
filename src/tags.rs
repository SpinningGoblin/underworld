use poem_openapi::Tags;

#[derive(Tags)]
pub enum UnderworldApiTags {
    Auth,
    PlayerCharacters,
    Games,
    GameActions,
    Randomizers,
}
