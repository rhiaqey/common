use crate::{error::RhiaqeyError, redis::RedisSettings};

pub fn connect(settings: RedisSettings) -> Result<redis::Client, RhiaqeyError> {
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

    let client = redis::Client::open(connect_uri)?;
    Ok(client)
}

pub fn connect_and_ping(settings: RedisSettings) -> Result<redis::Client, RhiaqeyError> {
    let client = connect(settings)?;
    let mut connection = client.get_connection()?;
    let result: String = redis::cmd("PING").query(&mut connection)?;
    if result != "PONG" {
        return Err("ping failed".into());
    }

    Ok(client)
}
