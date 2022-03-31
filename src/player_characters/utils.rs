pub fn username_key(username: &str) -> String {
    format!("username:{}:player_characters", &username)
}

pub fn username_player_character_key(username: &str, id: &str) -> String {
    format!("{}:{}", username_key(username), id)
}
