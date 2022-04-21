pub fn username_game_key(username: &str, game_id: &str) -> String {
    format!("username:{}:game:{}", username, game_id)
}
