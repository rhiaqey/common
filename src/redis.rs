use anyhow::{bail, Context};
use rustis::client::Client;
use rustis::commands::{ConnectionCommands, PingOptions};
use rustis::resp::{deserialize_byte_buf, PrimitiveResponse};
use serde::{Deserialize, Serialize};

fn default_redis_db() -> String {
    String::from("0")
}

fn default_redis_sentinel_master() -> String {
    String::from("mymaster")
}

#[derive(Deserialize, PartialEq, Eq, Default, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum RedisMode {
    #[default]
    Standalone,
    Sentinel,
}

#[derive(Deserialize, Default, Clone, Debug)]
pub struct RedisSettings {
    #[serde(default = "RedisMode::default")]
    pub redis_mode: RedisMode,
    pub redis_address: Option<String>,
    pub redis_sentinel_addresses: Option<String>,
    pub redis_password: Option<String>,
    #[serde(default = "default_redis_db")]
    pub redis_db: String,
    #[serde(default = "default_redis_sentinel_master")]
    pub redis_sentinel_master: String,
}

impl RedisSettings {
    pub fn is_standalone_mode(&self) -> bool {
        self.redis_mode == RedisMode::Standalone
    }

    pub fn is_sentinel_mode(&self) -> bool {
        self.redis_mode == RedisMode::Sentinel && !self.get_sentinel_nodes().is_empty()
    }

    pub fn get_db(&self) -> i32 {
        self.redis_db.parse::<i32>().unwrap_or(0)
    }

    pub fn get_password(&self) -> Option<String> {
        self.redis_password.clone()
    }

    pub fn get_sentinel_master_name(&self) -> String {
        self.redis_sentinel_master.clone()
    }

    pub fn get_sentinel_nodes(&self) -> Vec<String> {
        if let Some(nodes_str) = self.redis_sentinel_addresses.clone() {
            return nodes_str
                .split(',')
                .map(|x| {
                    if x.starts_with("redis://") {
                        x.to_string()
                    } else {
                        format!("redis://{}", x)
                    }
                })
                .collect();
        }

        vec![]
    }
}

pub async fn connect_async(settings: RedisSettings) -> anyhow::Result<Client> {
    let address = settings.redis_address.unwrap();

    let connect_uri = match settings.redis_password {
        None => format!("redis://{}", address),
        Some(password) => format!("redis://:{}@{}", password, address),
    };

    let client = Client::connect(connect_uri)
        .await
        .context("failed to connect async to redis")?;
    Ok(client)
}

pub async fn connect_and_ping_async(config: RedisSettings) -> anyhow::Result<Client> {
    let redis_connection = connect_async(config)
        .await
        .context("failed to connect async to redis")?;

    let result: String = redis_connection
        .clone()
        .ping(PingOptions::default().message("hello"))
        .await
        .context("failed to send PING to redis")?;
    if result != "hello" {
        bail!("ping failed");
    }

    Ok(redis_connection)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RhiaqeyBufVec(#[serde(deserialize_with = "deserialize_byte_buf")] pub Vec<u8>);
impl PrimitiveResponse for RhiaqeyBufVec {}

impl From<Vec<u8>> for RhiaqeyBufVec {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_db() {
        let config = RedisSettings {
            redis_db: String::from("2"),
            ..Default::default()
        };
        assert_eq!(config.get_db(), 2)
    }

    #[test]
    fn get_db_fallback() {
        let config = RedisSettings {
            ..Default::default()
        };
        assert_eq!(config.get_db(), 0)
    }

    #[test]
    fn get_master_name() {
        let config = RedisSettings {
            redis_sentinel_master: String::from("some_weird_name"),
            ..Default::default()
        };
        assert_eq!(
            config.get_sentinel_master_name(),
            "some_weird_name".to_string()
        )
    }

    #[test]
    fn get_master_name_fallback() {
        let config = RedisSettings {
            ..Default::default()
        };
        assert_eq!(config.get_sentinel_master_name(), String::from(""))
    }

    #[test]
    fn get_sentinel_node() {
        let config = RedisSettings {
            redis_sentinel_addresses: Some("redis://localhost:3001".to_string()),
            ..Default::default()
        };
        assert_eq!(config.get_sentinel_nodes(), vec!["redis://localhost:3001"])
    }

    #[test]
    fn get_sentinel_node_normalized() {
        let config = RedisSettings {
            redis_sentinel_addresses: Some("localhost:3001".to_string()),
            ..Default::default()
        };
        assert_eq!(config.get_sentinel_nodes(), vec!["redis://localhost:3001"])
    }

    #[test]
    fn get_sentinel_nodes() {
        let config = RedisSettings {
            redis_sentinel_addresses: Some(
                "redis://localhost:3001,redis://localhost:3002".to_string(),
            ),
            ..Default::default()
        };
        assert_eq!(
            config.get_sentinel_nodes(),
            vec!["redis://localhost:3001", "redis://localhost:3002"]
        )
    }

    #[test]
    fn get_sentinel_nodes_normalized() {
        let config = RedisSettings {
            redis_sentinel_addresses: Some("localhost:3001/,localhost:3002".to_string()),
            ..Default::default()
        };
        assert_eq!(
            config.get_sentinel_nodes(),
            vec!["redis://localhost:3001/", "redis://localhost:3002"]
        )
    }

    #[test]
    fn get_sentinel_nodes_normalized_mix() {
        let config = RedisSettings {
            redis_sentinel_addresses: Some(
                "localhost:3001/,redis://localhost:3002,redis://localhost:3005/0".to_string(),
            ),
            ..Default::default()
        };
        assert_eq!(
            config.get_sentinel_nodes(),
            vec![
                "redis://localhost:3001/",
                "redis://localhost:3002",
                "redis://localhost:3005/0"
            ]
        )
    }
}
