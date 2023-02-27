use crate::stream::StreamMessage;
use rhiaqey_sdk::channel::ChannelList;
use rhiaqey_sdk::message::MessageValue;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum RPCMessageData {
    NotifyClients(StreamMessage),
    AssignChannels(ChannelList),
    UpdateSettings(MessageValue)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RPCMessage {
    pub data: RPCMessageData,
}
