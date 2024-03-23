use crate::{redis::RedisSettings, RhiaqeyResult};

pub fn connect(settings: &RedisSettings) -> RhiaqeyResult<redis::Client> {
    let password = settings.redis_password.as_ref().unwrap();

    let connect_uri = match settings.redis_address.as_ref() {
        None => format!(
            "redis+sentinel://{}@{}/{}?sentinel_password={}",
            password,
            settings.redis_sentinel_addresses.as_ref().unwrap(),
            settings.redis_sentinel_master.as_ref().unwrap(),
            password,
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

pub fn connect_and_ping(settings: &RedisSettings) -> RhiaqeyResult<redis::Client> {
    let client = connect(settings)?;

    let mut connection = client.get_connection()?;
    let result: String = redis::cmd("PING").query(&mut connection)?;
    if result != "PONG" {
        return Err("ping failed".into());
    }

    Ok(client)
}
