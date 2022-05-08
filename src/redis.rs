use std::env;

fn get_redis_url() -> String {
    env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1".to_string())
}

pub async fn get_redis_connection() -> redis::aio::Connection {
    let client = redis::Client::open(get_redis_url()).unwrap();
    client.get_async_connection().await.unwrap()
}
