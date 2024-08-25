pub fn publishers_to_hub_stream_topic<S: AsRef<str>>(namespace: S, channel: S) -> String {
    format!(
        "{}:hub:channels:{}:raw",
        namespace.as_ref(),
        channel.as_ref()
    )
}

pub fn events_pubsub_topic<S: AsRef<str>>(namespace: S) -> String {
    format!("{}:hub:streams:pubsub:events", namespace.as_ref(),)
}

pub fn hub_raw_to_hub_clean_pubsub_topic<S: AsRef<str>>(namespace: S) -> String {
    format!("{}:hub:streams:pubsub:clean", namespace.as_ref())
}

pub fn hub_to_publisher_pubsub_topic<S: AsRef<str>>(namespace: S, publisher_name: S) -> String {
    format!(
        "{}:publishers:{}:streams:pubsub",
        namespace.as_ref(),
        publisher_name.as_ref()
    )
}

pub fn hub_channel_snapshot_topic<S: AsRef<str>>(
    namespace: S,
    channel: S,
    category: S,
    key: S,
) -> String {
    format!(
        "{}:hub:channels:{}:snapshot:{}:{}",
        namespace.as_ref(),
        channel.as_ref(),
        category.as_ref(),
        key.as_ref(),
    )
}

pub fn hub_channels_key<S: AsRef<str>>(namespace: S) -> String {
    format!("{}:hub:channels", namespace.as_ref())
}

pub fn publisher_channels_key<S: AsRef<str>>(namespace: S, publisher_name: S) -> String {
    format!(
        "{}:publishers:{}:channels",
        namespace.as_ref(),
        publisher_name.as_ref()
    )
}

pub fn hub_settings_key<S: AsRef<str>>(namespace: S) -> String {
    format!("{}:hub:settings", namespace.as_ref())
}

pub fn hub_schema_key<S: AsRef<str>>(namespace: S) -> String {
    format!("{}:hub:schema", namespace.as_ref())
}

pub fn publisher_settings_key<S: AsRef<str>>(namespace: S, publisher_name: S) -> String {
    format!(
        "{}:publishers:{}:settings",
        namespace.as_ref(),
        publisher_name.as_ref()
    )
}

pub fn publisher_schema_key<S: AsRef<str>>(namespace: S, publisher_name: S) -> String {
    format!(
        "{}:publishers:{}:schema",
        namespace.as_ref(),
        publisher_name.as_ref(),
    )
}

pub fn security_key<S: AsRef<str>>(namespace: S) -> String {
    format!("{}:security", namespace.as_ref())
}

pub fn publisher_channels_snapshot<S: AsRef<str>>(
    namespace: S,
    publisher_name: S,
    key: S,
    category: S,
) -> String {
    format!(
        "{}:keys:{}:{}",
        publisher_channels_key(namespace, publisher_name),
        key.as_ref(),
        category.as_ref()
    )
}
