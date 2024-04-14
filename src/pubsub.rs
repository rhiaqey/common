use crate::stream::StreamMessage;
use anyhow::Context;
use rhiaqey_sdk_rs::channel::Channel;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

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
pub struct MetricsMessage {
    /// Each pod will have a different id
    pub id: String,

    /// All deployment pods will have the same name
    pub name: String,

    /// Namespace of the k8s installation
    pub namespace: String,

    /// Each component must provide metrics
    pub metrics: serde_json::Value,
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
    PurgeChannels(Vec<String>),
    // this goes from hub to all publishers
    AssignChannels(Vec<Channel>),
    // this goes from publishers to hub
    Metrics(MetricsMessage),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RPCMessage {
    pub data: RPCMessageData,
}

impl Display for RPCMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.data {
            RPCMessageData::RegisterPublisher(_) => write!(f, "register_publisher"),
            RPCMessageData::NotifyClients(_) => write!(f, "notify_clients"),
            RPCMessageData::UpdateHubSettings() => write!(f, "update_hub_settings"),
            RPCMessageData::UpdatePublisherSettings() => write!(f, "update_publisher_settings"),
            RPCMessageData::CreateChannels(_) => write!(f, "create_channels"),
            RPCMessageData::DeleteChannels(_) => write!(f, "delete_channels"),
            RPCMessageData::PurgeChannels(_) => write!(f, "purge_channels"),
            RPCMessageData::AssignChannels(_) => write!(f, "assign_channels"),
            RPCMessageData::Metrics(_) => write!(f, "metrics"),
        }
    }
}

impl RPCMessage {
    pub fn ser_to_string(&self) -> anyhow::Result<String> {
        serde_json::to_string(self).context("failed to serialize")
    }

    pub fn der_from_string(message: &str) -> anyhow::Result<RPCMessage> {
        serde_json::from_str::<RPCMessage>(message).context("failed to deserialize")
    }
}
