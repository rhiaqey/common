use rhiaqey_sdk::channel::ChannelList;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum RPCMessageType {
    AssignChannels(ChannelList)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RPCMessage {
    pub data: RPCMessageType,
}
