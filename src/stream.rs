use anyhow::Context;
use rhiaqey_sdk_rs::gateway::GatewayMessage;
use rhiaqey_sdk_rs::message::MessageValue;
use rhiaqey_sdk_rs::producer::ProducerMessage;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum StreamMessageDataType {
    Data = 0, // sent data from hub to client
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct StreamMessage {
    // type of data we are sending to user
    #[serde(rename = "typ")]
    pub data_type: u8,

    // source channel
    #[serde(rename = "chn")]
    pub channel: String,

    #[serde(rename = "key")]
    pub key: String,

    // Any value
    #[serde(rename = "val")]
    pub value: MessageValue,

    // If timestamp is provided there will a check in timestamps.
    // If the latest entry in a database is older than the message,
    // then we do not store the new message
    #[serde(rename = "tms", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<u64>,

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

    // target specific user ids
    #[serde(rename = "uid", skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,

    // hub_id is actually hub id. useful for debugging
    #[serde(rename = "hid", skip_serializing_if = "Option::is_none")]
    pub hub_id: Option<String>,

    // gateway or producer id, useful for debugging
    #[serde(rename = "pid", skip_serializing_if = "Option::is_none")]
    pub publisher_id: Option<String>,
}

impl StreamMessage {
    pub fn ser_to_string(&self) -> anyhow::Result<String> {
        serde_json::to_string(self).context("failed to serialize")
    }

    pub fn der_from_string(message: &str) -> anyhow::Result<StreamMessage> {
        serde_json::from_str::<StreamMessage>(message).context("failed to deserialize")
    }
}

impl From<ProducerMessage> for StreamMessage {
    fn from(value: ProducerMessage) -> Self {
        StreamMessage {
            data_type: StreamMessageDataType::Data as u8,
            key: value.key,
            value: value.value,
            timestamp: value.timestamp,
            tag: value.tag,
            category: value.category,
            size: value.size,
            channel: String::from(""),
            user_ids: value.user_ids,
            client_ids: value.client_ids,
            hub_id: None,
            publisher_id: None,
        }
    }
}

impl From<GatewayMessage> for StreamMessage {
    fn from(value: GatewayMessage) -> Self {
        StreamMessage {
            data_type: StreamMessageDataType::Data as u8,
            key: value.key,
            value: value.value,
            timestamp: value.timestamp,
            tag: value.tag,
            category: value.category,
            size: value.size,
            channel: String::from(""),
            user_ids: value.user_ids,
            client_ids: value.client_ids,
            hub_id: None,
            publisher_id: None,
        }
    }
}
