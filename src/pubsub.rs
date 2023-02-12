use rhiaqey_sdk::channel::ChannelList;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum RPCMessageData {
    AssignChannels(ChannelList)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RPCMessage {
    pub data: RPCMessageData,
}
