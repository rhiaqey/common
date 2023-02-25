use rhiaqey_sdk::channel::Channel;
use rhiaqey_sdk::message::MessageValue;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ClientMessageDataType {
    ClientConnection = 0,           // sent by the hub to the client with unique client id
    ClientChannelSubscription = 1,  // set by the hub to the client when they subscribe to a channel
    Data = 10,                      // sent data from hub to client
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClientMessageValueClientConnection {
    pub client_id: String,
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
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClientMessage {
    // type of data we are sending to user
    #[serde(rename = "typ")]
    pub data_type: u8,

    // source channel
    #[serde(rename = "chn", skip_serializing_if = "String::is_empty")]
    pub channel: String,

    #[serde(rename = "key", skip_serializing_if = "String::is_empty")]
    pub key: String,

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
