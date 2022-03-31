pub fn current_player_character_key(username: &str) -> String {
    format!("username:{}:current_player_character", username)
}
