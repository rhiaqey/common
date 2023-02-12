pub fn publishers_to_hub_stream_topic(namespace: String, channel: String) -> String {
    format!("{}:hub:channels:{}:raw", namespace, channel)
}

pub fn hub_raw_to_hub_clean_pubsub_topic(namespace: String, channel: String) -> String {
    format!("{}:hub:channels:{}:clean", namespace, channel)
}

pub fn hub_to_publisher_pubsub_topic(namespace: String, publisher_name: String) -> String {
    format!("{}:publishers:{}:streams:pubsub", namespace, publisher_name)
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