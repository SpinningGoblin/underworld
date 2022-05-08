pub fn username_key(username: &str) -> String {
    format!("username:{}:game", &username)
}

pub fn username_game_key(username: &str, game_id: &str) -> String {
    format!("username:{}:game:{}", username, game_id)
}
