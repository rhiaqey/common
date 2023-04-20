use log::{debug, info, trace, warn};
use rhiaqey_sdk::channel::{Channel, ChannelList};
use rustis::client::{Client, PubSubMessage, PubSubStream};
use rustis::commands::{PubSubCommands, StreamCommands, StringCommands, XAddOptions, XTrimOperator, XTrimOptions};
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use std::sync::Arc;
use rhiaqey_sdk::message::MessageValue;
use tokio::sync::{Mutex, RwLock};
use crate::env::Env;
use crate::error::RhiaqeyError;
use crate::pubsub::RPCMessage;
use crate::redis::{connect_and_ping, RhiaqeyBufVec};
use crate::stream::{StreamMessage};
use crate::topics;

pub struct Executor {
    env: Arc<Env>,
    channels: Arc<RwLock<Vec<Channel>>>,
    redis: Arc<Mutex<Option<Client>>>,
}

impl Executor {
    pub fn get_id(&self) -> String {
        self.env.id.clone()
    }

    pub fn get_name(&self) -> String {
        self.env.name.clone()
    }

    pub fn get_public_port(&self) -> u16 {
        self.env.public_port.unwrap()
    }

    pub fn get_private_port(&self) -> u16 {
        self.env.private_port.unwrap()
    }

    pub fn get_namespace(&self) -> String {
        self.env.namespace.clone()
    }

    pub async fn set_channels(&mut self, channels: Vec<Channel>) {
        let mut locked_channels = self.channels.write().await;
        *locked_channels = channels;
    }

    pub async fn get_channel_count(&self) -> usize {
        self.channels.read().await.len()
    }

    pub async fn read_channels(&self) -> Vec<Channel> {
        let channels_key =
            topics::publisher_channels_key(self.env.namespace.clone(), self.env.name.clone());

        let result: String = self
            .redis
            .lock()
            .await
            .as_mut()
            .unwrap()
            .get(channels_key.clone())
            .await
            .unwrap();

        let channel_list: ChannelList =
            serde_json::from_str(result.as_str()).unwrap_or(ChannelList::default());

        debug!(
            "channels from {} retrieved {:?}",
            channels_key, channel_list
        );

        channel_list.channels
    }

    pub async fn read_settings<T: DeserializeOwned + Default + Debug>(&self) -> Result<T, RhiaqeyError> {
        let settings_key =
            topics::publisher_settings_key(self.get_namespace(), self.get_name());

        let result: RhiaqeyBufVec = self
            .redis
            .lock()
            .await
            .as_mut()
            .unwrap()
            .get(settings_key)
            .await?;
        debug!("encrypted settings retrieved");

        let data = self.env.decrypt(result.0)?;
        debug!("raw data decrypted");

        let settings = MessageValue::Binary(data).decode::<T>()?;
        debug!("decrypted data decoded into settings");

        Ok(settings)
    }

    pub async fn setup(config: Env) -> Result<Executor, String> {
        let redis_connection = connect_and_ping(config.redis.clone()).await;
        if redis_connection.is_none() {
            return Err("failed to connect to redis".to_string());
        }

        let mut executor = Executor {
            env: Arc::from(config),
            channels: Arc::from(RwLock::new(vec![])),
            redis: Arc::new(Mutex::new(redis_connection)),
        };

        let channels = executor.read_channels().await;
        executor.set_channels(channels).await;

        Ok(executor)
    }

    pub fn extract_pubsub_message(&mut self, message: PubSubMessage) -> Option<RPCMessage> {
        trace!("handle pubsub message");
        if let Ok(data) = serde_json::from_slice::<RPCMessage>(message.payload.as_slice()) {
            trace!("pubsub message contains an RPC message {:?}", data);
            // self.handle_rpc_message(data).await;
            Some(data)
        } else {
            None
        }
    }

    pub async fn create_hub_to_publishers_pubsub(&mut self) -> Option<PubSubStream> {
        let client = connect_and_ping(self.env.redis.clone()).await;
        if client.is_none() {
            warn!("failed to connect with ping");
            return None;
        }

        let key = topics::hub_to_publisher_pubsub_topic(
            self.env.namespace.clone(),
            self.env.name.clone(),
        );

        let stream = client.unwrap().subscribe(key.clone()).await.unwrap();

        Some(stream)
    }

    pub async fn publish(&self, message: impl Into<StreamMessage>) {
        info!("publishing message to the channels");

        let mut stream_msg: StreamMessage = message.into();

        // if self.is_debug() {
        stream_msg.publisher_id = Some(self.env.id.clone());
        // }

        let tag = stream_msg.tag.clone().unwrap_or(String::from(""));

        for channel in self.channels.read().await.iter() {
            stream_msg.channel = channel.name.to_string();

            if stream_msg.size.is_none() {
                stream_msg.size = Some(channel.size);
            }

            let topic = topics::publishers_to_hub_stream_topic(
                self.env.namespace.clone(),
                channel.name.clone(),
            );

            let xadd_options = XAddOptions::default();
            let trim_options = XTrimOptions::max_len(
                XTrimOperator::Approximately,
                // channel.size as i64,
                10000
            );

            info!(
                "publishing message channel={}, max_len={}, topic={}, timestamp={:?}",
                channel.name, channel.size, topic, stream_msg.timestamp,
            );

            let tms = stream_msg.timestamp.unwrap_or(0);

            if let Ok(data) = serde_json::to_string(&stream_msg) {
                let id: String = self
                    .redis
                    .lock()
                    .await
                    .as_mut()
                    .unwrap()
                    .xadd(
                        topic.clone(),
                        "*",
                        [("raw", data.clone()), ("tag", tag.clone()), ("tms", format!("{}", tms))],
                        xadd_options.trim_options(trim_options),
                        // XAddOptions::default()
                    )
                    .await
                    .unwrap();
                debug!(
                    "sent message {} to channel {} in topic {}",
                    id, channel.name, topic
                );
            }
        }
    }
}
