use crate::redis::RedisMode;
use crate::redis::RedisSettings;
use anyhow::{bail, Context};
use redis::sentinel::{Sentinel, SentinelNodeConnectionInfo};
use redis::{Client, ProtocolVersion, RedisConnectionInfo};

pub fn connect(settings: &RedisSettings) -> anyhow::Result<Client> {
    let client = match settings.redis_mode {
        RedisMode::Standalone => {
            let connect_uri = if let Some(password) = settings.get_password() {
                format!(
                    "redis://:{}@{}/{}",
                    password,
                    settings
                        .redis_address
                        .clone()
                        .unwrap_or(String::from("localhost:6379")),
                    settings.get_db()
                )
            } else {
                format!(
                    "redis://{}/{}",
                    settings
                        .redis_address
                        .clone()
                        .unwrap_or(String::from("localhost:6379")),
                    settings.get_db()
                )
            };

            Client::open(connect_uri)
        }
        RedisMode::Sentinel => {
            let nodes = settings.get_sentinel_nodes();
            let master_name = settings.get_sentinel_master_name();
            let db = settings.get_db();
            let mut sentinel = Sentinel::build(nodes).context("failed to build sentinel")?;

            let mut info = RedisConnectionInfo::default()
                .set_protocol(ProtocolVersion::RESP3)
                .set_db(db as i64);

            if let Some(password) = settings.get_password() {
                info = info.set_password(password);
            }

            sentinel.master_for(
                master_name.as_str(),
                Some(&SentinelNodeConnectionInfo::default().set_redis_connection_info(info)),
            )
        }
    };

    match client {
        Ok(client) => Ok(client),
        Err(err) => Err(err.into()),
    }
}

pub fn connect_and_ping(settings: &RedisSettings) -> anyhow::Result<Client> {
    let client = connect(settings).context("failed to connect to redis")?;

    let mut connection = client
        .get_connection()
        .context("failed to acquire connection")?;
    let result: String = redis::cmd("PING")
        .query(&mut connection)
        .context("failed to send PING")?;
    if result != "PONG" {
        bail!("ping failed");
    }

    Ok(client)
}
