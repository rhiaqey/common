use log::warn;
use serde::de::DeserializeOwned;

pub fn parse_settings<S: DeserializeOwned + Default>() -> Option<S> {
    match std::env::var("SETTINGS").map(|var| serde_json::from_str::<S>(var.as_str()).unwrap()) {
        Ok(settings) => Some(settings),
        Err(err) => {
            warn!("error parsing settings: {}", err);
            None
        }
    }
}
