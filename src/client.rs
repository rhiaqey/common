use std::borrow::Cow;
use rhiaqey_sdk_rs::channel::Channel;
use rhiaqey_sdk_rs::message::MessageValue;
use serde::{Deserialize, Serialize};
use crate::stream::StreamMessage;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ClientMessageDataType {
    ClientConnection = 0,           // sent by the hub to the client with unique client id
    ClientChannelSubscription = 1,  // set by the hub to the client when they subscribe to a channel
    Data = 10,                      // sent data from hub to client
    Ping = 100,                     // sent by the hub to keep the client connect alive
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClientMessageValueClientConnection {
    pub client_id: String,
    pub hub_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClientMessageValueClientChannelSubscription {
    pub channel: Channel
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum ClientMessageValue {
    ClientConnection(ClientMessageValueClientConnection),
    ClientChannelSubscription(ClientMessageValueClientChannelSubscription),
    Data(MessageValue),
    Ping(u64)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClientMessage {
    // type of data we are sending to user
    #[serde(rename = "typ")]
    pub data_type: u8,

    // source channel
    #[serde(rename = "chn")]
    pub channel: Cow<'static, str>,

    #[serde(rename = "key")]
    pub key: Cow<'static, str>,

    // Any value
    #[serde(rename = "val")]
    pub value: ClientMessageValue,

    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,

    // Extra grouping
    #[serde(rename = "cat", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,

    // hub_id is actually hub id. useful for debugging
    #[serde(rename = "hid", skip_serializing_if = "Option::is_none")]
    pub hub_id: Option<String>,

    // gateway or producer id, useful for debugging
    #[serde(rename = "pid", skip_serializing_if = "Option::is_none")]
    pub publisher_id: Option<String>,
}

impl From<StreamMessage> for ClientMessage {
    fn from(value: StreamMessage) -> Self {
        ClientMessage{
            data_type: ClientMessageDataType::Data as u8,
            channel: value.channel.into(),
            key: value.key.into(),
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
        ClientMessage{
            data_type: ClientMessageDataType::Data as u8,
            channel: value.channel.clone().into(),
            key: value.key.clone().into(),
            value: ClientMessageValue::Data(value.value.clone()),
            tag: value.tag.clone(),
            category: value.category.clone(),
            hub_id: value.hub_id.clone(),
            publisher_id: value.publisher_id.clone(),
        }
    }
}
