use crate::{error::RhiaqeyError, redis::RedisSettings};

pub fn connect(settings: RedisSettings) -> Result<redis::Connection, RhiaqeyError> {
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
    let connection = client.get_connection()?;
    Ok(connection)
}

pub fn connect_and_ping(settings: RedisSettings) -> Result<redis::Connection, RhiaqeyError> {
    let mut connection = connect(settings)?;
    let result: String = redis::cmd("PING").query(&mut connection)?;
    if result != "PONG" {
        return Err("ping failed".into());
    }

    Ok(connection)
}
