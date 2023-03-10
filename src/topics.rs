pub fn publishers_to_hub_stream_topic(namespace: String, channel: String) -> String {
    format!("{}:hub:channels:{}:raw", namespace, channel)
}

pub fn hub_raw_to_hub_clean_pubsub_topic(namespace: String) -> String {
    format!("{}:hub:streams:pubsub:clean", namespace)
}

pub fn hub_to_publisher_pubsub_topic(namespace: String, publisher_name: String) -> String {
    format!("{}:publishers:{}:streams:pubsub", namespace, publisher_name)
}

pub fn hub_channel_snapshot_topic(namespace: String, channel: String, key: String, category: String) -> String {
    format!("{}:hub:channels:{}:snapshot:{}:{}", namespace, channel, key, category)
}

pub fn hub_channels_key(namespace: String) -> String {
    format!("{}:hub:channels", namespace)
}

pub fn publisher_channels_key(namespace: String, publisher_name: String) -> String {
    format!("{}:publishers:{}:channels", namespace, publisher_name)
}

pub fn publisher_settings_key(namespace: String, publisher_name: String) -> String {
    format!("{}:publishers:{}:settings", namespace, publisher_name)
}

pub fn publisher_channels_snapshot(
    namespace: String,
    publisher_name: String,
    key: String,
    category: String,
) -> String {
    format!(
        "{}:keys:{}:{}",
        publisher_channels_key(namespace, publisher_name),
        key,
        category
    )
}
