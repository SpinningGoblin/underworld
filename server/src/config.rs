use std::env;

pub fn get_psql_url() -> String {
    env::var("DATABASE_URL").unwrap()
}

pub fn get_port() -> u16 {
    env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080)
}

pub fn get_server_api_url() -> String {
    format!("{}/api", get_server_url())
}

pub fn get_server_url() -> String {
    env::var("SERVER_URL")
        .ok()
        .unwrap_or(format!("http://localhost:{}", get_port()))
}

pub fn get_server_auth_url() -> String {
    format!("{}/auth", get_server_url())
}
