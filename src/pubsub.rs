use crate::stream::StreamMessage;
use crate::RhiaqeyResult;
use rhiaqey_sdk_rs::channel::{Channel, ChannelList};
use serde::{Deserialize, Serialize};

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
    pub schema: serde_json::Value,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum RPCMessageData {
    RegisterPublisher(PublisherRegistrationMessage),
    // this comes from hub raw to hub clean
    NotifyClients(StreamMessage),
    // this goes from hub to hub to notify them all to reload
    UpdateHubSettings(),
    // this goes from hub to publishers
    UpdatePublisherSettings(),
    // create channels from http admin
    CreateChannels(Vec<Channel>),
    // delete channels from http admin
    DeleteChannels(Vec<Channel>),
    // empty channel content from http admin
    PurgeChannel(String),
    // this goes from hub to all publishers
    AssignChannels(ChannelList),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RPCMessage {
    pub data: RPCMessageData,
}

impl RPCMessage {
    pub fn ser_to_string(&self) -> RhiaqeyResult<String> {
        serde_json::to_string(self).map_err(|x| x.into())
    }

    pub fn der_from_string(message: &str) -> RhiaqeyResult<RPCMessage> {
        serde_json::from_str::<RPCMessage>(message).map_err(|x| x.into())
    }
}
