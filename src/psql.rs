use std::env;
pub fn get_psql_url() -> String {
    env::var("DATABASE_URL").unwrap()
}
