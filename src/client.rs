use rhiaqey_sdk::message::MessageValue;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ClientMessageDataType {
    ClientConnect = 0, // sent by the hub to the client with session id
    Data = 1,          // sent data from hub to client
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClientMessageValueClientConnected {
    pub client_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ClientMessageValue {
    ClientConnected(ClientMessageValueClientConnected),
    Data(MessageValue),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClientMessage {
    // type of data we are sending to user
    #[serde(rename = "typ")]
    pub data_type: u8,

    // source channel
    #[serde(rename = "chn")]
    pub channel: String,

    #[serde(rename = "key")]
    pub key: String,

    // Any value
    #[serde(flatten, rename = "val")]
    pub value: ClientMessageValue,

    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,

    // Extra grouping
    #[serde(rename = "cat", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,

    #[serde(rename = "siz", skip_serializing_if = "Option::is_none")]
    pub size: Option<usize>,

    // hub_id is actually hub id. useful for debugging
    #[serde(rename = "hid", skip_serializing_if = "Option::is_none")]
    pub hub_id: Option<String>,

    // gateway or producer id, useful for debugging
    #[serde(rename = "pid", skip_serializing_if = "Option::is_none")]
    pub publisher_id: Option<String>,
}
