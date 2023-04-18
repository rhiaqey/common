use crate::stream::StreamMessage;
use rhiaqey_sdk::channel::ChannelList;
use serde::{Deserialize, Serialize};
use crate::error::RhiaqeyError;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum RPCMessageData {
    NotifyClients(StreamMessage),
    AssignChannels(ChannelList),
    UpdateSettings()
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RPCMessage {
    pub data: RPCMessageData,
}

impl RPCMessage {
    pub fn to_string(&self) -> Result<String, RhiaqeyError> {
        serde_json::to_string(self).map_err(|e|
            RhiaqeyError{ code: None, message: e.to_string(), error: Some(Box::new(e)) })
    }

    pub fn from_string(message: &str) -> Result<RPCMessage, RhiaqeyError> {
        serde_json::from_str::<RPCMessage>(message).map_err(|e|
            RhiaqeyError{ code: None, message: e.to_string(), error: Some(Box::new(e)) })
    }
}
