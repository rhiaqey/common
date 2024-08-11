use crate::stream::StreamMessage;
use anyhow::Context;
use rhiaqey_sdk_rs::channel::Channel;
use rhiaqey_sdk_rs::message::MessageValue;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ClientMessageDataType {
    ClientConnection = 0, // sent by the hub to the client with unique client id
    ClientChannelSubscription = 1, // set by the hub to the client when they subscribe to a channel
    Data = 10,            // sent data from hub to client
    Ping = 100,           // sent by the hub to keep the client connect alive
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClientMessageValueClientConnection {
    pub client_id: String,
    pub hub_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClientMessageValueClientChannelSubscription {
    pub channel: Channel,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ClientMessageValue {
    ClientConnection(ClientMessageValueClientConnection),
    ClientChannelSubscription(ClientMessageValueClientChannelSubscription),
    Data(MessageValue),
    Ping(u64),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClientMessage {
    #[serde(rename = "d", alias = "typ")]
    pub data_type: u8,

    #[serde(rename = "c", alias = "chn", skip_serializing_if = "String::is_empty")]
    pub channel: String,

    #[serde(rename = "k", alias = "key", skip_serializing_if = "String::is_empty")]
    pub key: String,

    #[serde(rename = "v", alias = "val")]
    pub value: ClientMessageValue,

    #[serde(rename = "t", alias = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,

    // Extra grouping
    #[serde(rename = "g", alias = "cat", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,

    // hub_id is actually hub id. useful for debugging
    #[serde(rename = "h", alias = "hid", skip_serializing_if = "Option::is_none")]
    pub hub_id: Option<String>,

    // gateway or producer id, useful for debugging
    #[serde(rename = "p", alias = "pid", skip_serializing_if = "Option::is_none")]
    pub publisher_id: Option<String>,
}

impl From<StreamMessage> for ClientMessage {
    fn from(value: StreamMessage) -> Self {
        ClientMessage {
            data_type: ClientMessageDataType::Data as u8,
            channel: value.channel,
            key: value.key,
            value: ClientMessageValue::Data(value.value),
            tag: value.tag,
            category: value.category,
            hub_id: value.hub_id,
            publisher_id: value.publisher_id,
        }
    }
}

impl From<&StreamMessage> for ClientMessage {
    fn from(value: &StreamMessage) -> Self {
        ClientMessage {
            data_type: ClientMessageDataType::Data as u8,
            channel: value.channel.clone(),
            key: value.key.clone(),
            value: ClientMessageValue::Data(value.value.clone()),
            tag: value.tag.clone(),
            category: value.category.clone(),
            hub_id: value.hub_id.clone(),
            publisher_id: value.publisher_id.clone(),
        }
    }
}

impl ClientMessage {
    pub fn ser_to_string(&self) -> anyhow::Result<String> {
        serde_json::to_string(self).context("failed to serialize to string")
    }

    pub fn ser_to_binary(&self) -> anyhow::Result<Vec<u8>> {
        rmp_serde::to_vec_named(self).context("failed to serialize to binary")
    }
}

#[cfg(test)]
mod tests {
    use crate::client::{ClientMessage, ClientMessageDataType, ClientMessageValue};
    use rhiaqey_sdk_rs::message::MessageValue;

    #[test]
    fn can_serialize() {
        let client_message = ClientMessage {
            data_type: ClientMessageDataType::Data as u8,
            channel: "channel_1".to_string(),
            key: "key_1".to_string(),
            value: ClientMessageValue::Data(MessageValue::Text(String::from("some text"))),
            tag: None,
            category: None,
            hub_id: None,
            publisher_id: None,
        };

        let serialized = serde_json::to_string(&client_message).unwrap_or_default();
        assert!(!serialized.is_empty());
        assert!(serialized.contains("\"k\""));
        assert!(serialized.contains("\"key_1\""));
    }

    #[test]
    fn can_deserialize() {
        let serialized_message =
            "{\"d\":10,\"c\":\"channel_1\",\"k\":\"key_1\",\"v\":\"some text\"}";
        let client_message = serde_json::from_str::<ClientMessage>(serialized_message);
        let data_type = ClientMessageDataType::Data as u8;
        let value = ClientMessageValue::Data(MessageValue::Text(String::from("some text")));
        assert!(client_message.is_ok());
        let client_message = client_message.unwrap();
        assert_eq!(client_message.data_type, data_type);
        assert_eq!(client_message.channel, "channel_1");
        assert_eq!(client_message.key, "key_1");
        assert_eq!(client_message.value, value);
        assert_eq!(client_message.tag, None);
        assert_eq!(client_message.category, None);
        assert_eq!(client_message.hub_id, None);
        assert_eq!(client_message.publisher_id, None);
    }

    #[test]
    fn aliases_work_during_deserialization() {
        let serialized_message =
            "{\"d\":10,\"chn\":\"channel_1\",\"key\":\"key_1\",\"val\":\"some text 2\"}";
        let client_message = serde_json::from_str::<ClientMessage>(serialized_message);
        let data_type = ClientMessageDataType::Data as u8;
        let value = ClientMessageValue::Data(MessageValue::Text(String::from("some text 2")));
        assert!(client_message.is_ok());
        let client_message = client_message.unwrap();
        assert_eq!(client_message.data_type, data_type);
        assert_eq!(client_message.channel, "channel_1");
        assert_eq!(client_message.key, "key_1");
        assert_eq!(client_message.value, value);
        assert_eq!(client_message.tag, None);
        assert_eq!(client_message.category, None);
        assert_eq!(client_message.hub_id, None);
        assert_eq!(client_message.publisher_id, None);
    }
}
