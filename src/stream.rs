use rhiaqey_sdk::message::MessageValue;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum StreamMessageDataType {
    Default = 0,      // Do nothing
    Connected = 1,    // sent by the hub to the client with session id
    Disconnected = 2, // sent by the hub to the client to notify to close the connection
    Subscribe = 3,    // sent by the client to the hub in order to subscribe to a channel
    Subscribed = 4,   // sent by the hub to the client to notify for the subscription
    Unsubscribe = 5,  // sent by the client to hub in order to unsubscribe from channel
    Unsubscribed = 6, // sent by the hub to the client to confirm unsubscription
    Data = 7,         // sent data from hub to client
    PartialData = 8,  // sent by the hub to the client with partial data (used for partial updates)
    Ping = 9,         // sent by the client
    Pong = 10,        // sent by hub as a response to Ping
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StreamMessage {
    // type of data we are sending to user
    #[serde(rename = "typ")]
    pub data_type: StreamMessageDataType,

    // source channel
    #[serde(rename = "chn")]
    pub channel: String,

    #[serde(rename = "key")]
    pub key: String,

    // Any value
    #[serde(rename = "val")]
    pub value: MessageValue,

    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,

    // Extra grouping
    #[serde(rename = "cat", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,

    #[serde(rename = "siz", skip_serializing_if = "Option::is_none")]
    pub size: Option<usize>,

    // target specific client ids
    #[serde(rename = "cid", skip_serializing_if = "Option::is_none")]
    pub client_ids: Option<Vec<String>>,

    // target specific group ids
    #[serde(rename = "gid", skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,

    // target specific user ids
    #[serde(rename = "uid", skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,

    // If timestamp is provided there will a check in timestamps. If latest entry in database is
    // older than the message then we do not store the new message
    #[serde(rename = "tms", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<u64>,

    // hub_id is actually hub id. useful for debugging
    #[serde(rename = "hid", skip_serializing_if = "Option::is_none")]
    pub hub_id: Option<String>,

    // gateway or producer id, useful for debugging
    #[serde(rename = "pid", skip_serializing_if = "Option::is_none")]
    pub publisher_id: Option<String>,
}
