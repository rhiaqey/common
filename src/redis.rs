use serde::Deserialize;
use rustis::client::Client;

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

    let result = Client::connect(connect_uri).await.unwrap();
    Some(result)
}
