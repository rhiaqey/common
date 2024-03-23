use std::fmt::Debug;
use crate::{redis::RedisSettings, RhiaqeyResult};

#[derive(Debug, Default)]
pub struct FailOverRedisClient {
    pub db: String,
    pub master_name: String,
    pub master_uri: Option<String>,
    pub master_client: Option<redis::Client>,
    pub sentinel_uri: Option<String>,
    pub sentinel_client: Option<redis::Client>,
}

impl FailOverRedisClient {
    pub fn get_connection(&self) -> RhiaqeyResult<redis::Connection> {
        // check if we have a master connection
        if let Some(client) = self.master_client.as_ref() {
            let connection = client.get_connection();
            // get connection
            if let Ok(mut conn) = connection {
                // send ping to verify connectivity
                let result: String = redis::cmd("PING").query(&mut conn)?;
                // receive PONG as response
                if result == "PONG" {
                    // connection is healthy
                    return Ok(conn);
                }
            }
        }

        // fail over to sentinel
        if let Some(client) = self.sentinel_client.as_ref() {
            let connection = client.get_connection();
            // get connection
            if let Ok(mut conn) = connection {
                // send ping to verify connectivity
                let result: String = redis::cmd("PING").query(&mut conn)?;
                // receive PONG as response
                if result == "PONG" {
                    // connection is healthy
                    return Ok(conn);
                }
            }
        }

        Err("failed to obtain connection".into())
    }
}

pub fn connect(settings: &RedisSettings) -> RhiaqeyResult<FailOverRedisClient> {
    let Some(address) = &settings.redis_address else {
        return Err("could not find redis_address parameter".into())
    };

    // there is always a master name
    let db = &settings.redis_db.clone().unwrap();
    let master_name = &settings.redis_sentinel_master.clone().unwrap();

    // create a default client
    let mut client: FailOverRedisClient = FailOverRedisClient {
        db: db.clone(),
        master_name: master_name.clone(), ..Default::default()
    };

    // calculate a master uri
    let master_uri = match &settings.redis_password {
        None => format!("redis://{address}/{db}"),
        Some(password) => format!("redis://:{password}@{address}/{db}")
    };

    // try to connect using master uri
    if let Ok(master_client) = redis::Client::open(master_uri.clone()) {
        client.master_uri = Some(master_uri);
        client.master_client = Some(master_client);
    }

    // if there are sentinel addresses, we can try to connect to sentinel as well
    if let Some(addresses) = &settings.redis_sentinel_addresses {
        // calculate sentinel uri
        let sentinel_uri = if let Some(password) = &settings.redis_password {
            format!("redis://:{password}@{addresses}/{db}#{master_name}")
        } else {
            format!("redis://{addresses}/{db}#{master_name}")
        };

        // try to connect using sentinel uri
        if let Ok(sentinel_client) = redis::Client::open(sentinel_uri.clone()) {
            client.sentinel_uri = Some(sentinel_uri);
            client.sentinel_client = Some(sentinel_client);
        }
    }

    // return final client
    Ok(client)
}

pub fn connect_and_ping(settings: &RedisSettings) -> RhiaqeyResult<FailOverRedisClient> {
    let client = connect(settings)?;

    let mut connection = client.get_connection()?;
    let result: String = redis::cmd("PING").query(&mut connection)?;
    if result != "PONG" {
        return Err("ping failed".into());
    }

    Ok(client)
}
