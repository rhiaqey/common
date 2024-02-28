use crate::stream::StreamMessage;
use rhiaqey_sdk_rs::channel::ChannelList;
use serde::{Deserialize, Serialize};
use crate::error::RhiaqeyError;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PublisherRegistrationMessage {
    /// Each pod will have a different id
    pub id: String,

    /// All deployment pods will have the same name
    pub name: String,

    /// Namespace of the k8s installation
    pub namespace: String,

    /// Each publisher must specify a schema
    pub schema: serde_json::Value
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum RPCMessageData {
    RegisterPublisher(PublisherRegistrationMessage),
    // this comes from hub raw to hub clean
    NotifyClients(StreamMessage),
    // this goes from hub to all hubs
    AssignChannels(ChannelList),
    // this goes from hub to publishers
    UpdateSettings()
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RPCMessage {
    pub data: RPCMessageData,
}

impl RPCMessage {
    pub fn serialize(&self) -> Result<String, RhiaqeyError> {
        serde_json::to_string(self).map_err(|x| x.into())
    }

    pub fn deserialize(message: &str) -> Result<RPCMessage, RhiaqeyError> {
        serde_json::from_str::<RPCMessage>(message).map_err(|x| x.into())
    }
}
