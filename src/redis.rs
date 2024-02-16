use rustis::client::Client;
use rustis::commands::{ConnectionCommands, PingOptions};
use rustis::resp::{deserialize_byte_buf, PrimitiveResponse};
use serde::{Deserialize, Serialize};
use crate::error::RhiaqeyError;

fn default_redis_db() -> Option<String> {
    Some("0".to_string())
}

fn default_redis_sentinel_master() -> Option<String> {
    Some(String::from("mymaster"))
}

#[derive(Deserialize, Default, Clone, Debug)]
pub struct RedisSettings {
    pub redis_address: Option<String>,
    pub redis_sentinel_addresses: Option<String>,
    pub redis_password: Option<String>,
    #[serde(default = "default_redis_db")]
    pub redis_db: Option<String>,
    #[serde(default = "default_redis_sentinel_master")]
    pub redis_sentinel_master: Option<String>,
}

pub async fn connect(settings: RedisSettings) -> Result<Client, RhiaqeyError> {
    let password = settings.redis_password.unwrap();

    let connect_uri = match settings.redis_address {
        None => format!(
            "redis+sentinel://:{}@{}/{}?sentinel_password={}",
            password.clone(),
            settings.redis_sentinel_addresses.unwrap(),
            settings.redis_sentinel_master.unwrap(),
            password.clone()
        ),
        Some(address) => format!(
            "redis://:{}@{}",
            password,
            address
        ),
    };

    let client = Client::connect(connect_uri).await.map_err(|x| x.to_string())?;
    Ok(client)
}

pub async fn connect_and_ping(config: RedisSettings) -> Result<Client, RhiaqeyError> {
    let redis_connection = connect(config).await?;

    let result: String = redis_connection
        .clone()
        .ping(PingOptions::default().message("hello"))
        .await?;

    if result != "hello" {
        return Err("ping failed".to_string().into());
    }

    Ok(redis_connection)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RhiaqeyBufVec(#[serde(deserialize_with = "deserialize_byte_buf")] pub Vec<u8>);
impl PrimitiveResponse for RhiaqeyBufVec {}
