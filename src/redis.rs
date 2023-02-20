use rustis::client::Client;
use rustis::commands::{ConnectionCommands, PingOptions};
use serde::Deserialize;

fn default_redis_db() -> String {
    "0".to_string()
}

fn default_redis_password() -> String {
    String::from("")
}

fn default_redis_sentinel_master() -> String {
    String::from("mymaster")
}

#[derive(Deserialize, Default, Clone, Debug)]
pub struct RedisSettings {
    pub redis_address: Option<String>,
    pub redis_sentinel_addresses: Option<String>,
    #[serde(default = "default_redis_password")]
    pub redis_password: String,
    #[serde(default = "default_redis_db")]
    pub redis_db: String,
    #[serde(default = "default_redis_sentinel_master")]
    pub redis_sentinel_master: String,
}

pub async fn connect(settings: RedisSettings) -> Option<Client> {
    let connect_uri = match settings.redis_address {
        None => format!(
            "redis+sentinel://:{}@{}/{}?sentinel_password={}",
            settings.redis_password,
            settings.redis_sentinel_addresses.unwrap(),
            settings.redis_sentinel_master,
            settings.redis_password
        ),
        Some(_) => format!(
            "redis://:{}@{}",
            settings.redis_password,
            settings.redis_address.unwrap()
        ),
    };

    let result = Client::connect(connect_uri).await;
    if result.is_err() {
        None
    } else {
        Some(result.unwrap())
    }
}

pub async fn connect_and_ping(config: RedisSettings) -> Option<Client> {
    let redis_connection = connect(config).await;
    if redis_connection.is_none() {
        return None;
    }

    let result: String = redis_connection
        .clone()
        .unwrap()
        .ping(PingOptions::default().message("hello"))
        .await
        .unwrap();
    if result != "hello" {
        return None;
    }

    redis_connection
}
