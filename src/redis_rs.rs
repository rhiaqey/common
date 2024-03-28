use redis::{Client, RedisConnectionInfo};
use redis::sentinel::{Sentinel, SentinelNodeConnectionInfo};
use crate::{redis::RedisSettings, result::RhiaqeyResult};
use crate::redis::RedisMode;

pub fn connect(settings: &RedisSettings) -> RhiaqeyResult<Client> {
    let client = match settings.redis_mode {
        RedisMode::Standalone => {
            let connect_uri = if let Some(password) = settings.get_password() {
                format!("redis://:{}@{}/{}",
                        password,
                        settings.redis_address.clone().unwrap_or(String::from("localhost:6379")),
                        settings.get_db())
            } else {
                format!("redis://{}/{}",
                        settings.redis_address.clone().unwrap_or(String::from("localhost:6379")),
                        settings.get_db())
            };

            Client::open(connect_uri)
        }
        RedisMode::Sentinel => {
            let nodes = settings.get_sentinel_nodes();
            let master_name = settings.get_sentinel_master_name();
            let db = settings.get_db();
            let mut sentinel = Sentinel::build(nodes)?;
            sentinel.master_for(
                master_name.as_str(),
                Some(&SentinelNodeConnectionInfo {
                    tls_mode: None,
                    redis_connection_info: Some(RedisConnectionInfo {
                        db: db as i64,
                        username: None,
                        password: settings.get_password()
                    }),
                }),
            )
        }
    };

    match client {
        Ok(client) => Ok(client),
        Err(err) => Err(err.into())
    }
}

pub fn connect_and_ping(settings: &RedisSettings) -> RhiaqeyResult<Client> {
    let client = connect(settings)?;

    let mut connection = client.get_connection()?;
    let result: String = redis::cmd("PING").query(&mut connection)?;
    if result != "PONG" {
        return Err("ping failed".into());
    }

    Ok(client)
}
