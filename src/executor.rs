use crate::env::Env;
use crate::pubsub::RPCMessage;
use crate::redis::{connect_and_ping_async, RhiaqeyBufVec};
use crate::redis_rs::connect_and_ping;
use crate::security::SecurityKey;
use crate::stream::StreamMessage;
use crate::{security, topics, result::RhiaqeyResult};
use log::{debug, info, trace};
use redis::Commands;
use rhiaqey_sdk_rs::channel::{Channel, ChannelList};
use rhiaqey_sdk_rs::message::MessageValue;
use rustis::client::{Client, PubSubMessage, PubSubStream};
use rustis::commands::{
    PubSubCommands, StreamCommands, StringCommands, XAddOptions, XTrimOperator, XTrimOptions,
};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

pub struct Executor {
    env: Arc<Env>,
    redis: Arc<Mutex<Client>>,
    redis_rs: Arc<std::sync::Mutex<redis::Connection>>,
    channels: Arc<RwLock<Vec<Channel>>>,
    security: Arc<Mutex<SecurityKey>>,
}

#[derive(Default, Clone, Debug)]
pub struct ExecutorPublishOptions {
    pub trim_threshold: Option<i64>,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
struct PublisherChannel {
    pub name: String,
    pub channels: Vec<String>,
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

    pub async fn set_channels_async(&mut self, channels: Vec<Channel>) {
        let mut locked_channels = self.channels.write().await;
        *locked_channels = channels;
    }

    pub async fn get_channel_count_async(&self) -> usize {
        self.channels.read().await.len()
    }

    fn load_key(config: &Env, client: &mut redis::Connection) -> RhiaqeyResult<SecurityKey> {
        let namespace = config.namespace.clone();
        let security_key = topics::security_key(namespace);
        let security_str: String = client.get(security_key.clone()).unwrap_or(String::from(""));

        let mut security = serde_json::from_str::<SecurityKey>(security_str.as_str())?;

        security.key = config.decrypt(security.key)?;
        security.no_once = config.decrypt(security.no_once)?;

        debug!("security keys loaded");

        Ok(security)
    }

    pub async fn read_channels_async(&self) -> RhiaqeyResult<Vec<Channel>> {
        debug!("reading all assigned channels");

        // calculate channels key
        let all_channels_key = topics::hub_channels_key(self.get_namespace());
        trace!("all channels key {}", all_channels_key);

        let client = self.redis.lock().await;

        // get all channels in the system
        let all_channels_result: String = client.get(all_channels_key).await?;
        trace!("got channels {}", all_channels_result);

        let all_channels: ChannelList =
            serde_json::from_str(all_channels_result.as_str()).unwrap_or(ChannelList::default());
        trace!("got all channels result {:?}", all_channels);

        let publisher_channels_key =
            topics::publisher_channels_key(self.get_namespace(), self.env.name.clone());

        let publisher_channels_result: String = client.get(publisher_channels_key).await?;
        trace!("got publisher channels {}", publisher_channels_result);

        let all_publisher_channels: PublisherChannel =
            serde_json::from_str(publisher_channels_result.as_str()).unwrap_or(PublisherChannel {
                name: self.get_name(),
                channels: vec![],
            });
        trace!(
            "got all publisher channels result {:?}",
            all_publisher_channels
        );

        let channels = all_channels
            .channels
            .iter()
            .filter(|x| all_publisher_channels.channels.iter().any(|y| x.name.eq(y)))
            .cloned()
            .collect::<Vec<_>>();
        debug!("found {} channel(s) for publisher", channels.len());

        Ok(channels)
    }

    pub async fn read_settings_async<T: DeserializeOwned + Default + Debug>(
        &self,
    ) -> RhiaqeyResult<T> {
        let settings_key = topics::publisher_settings_key(self.get_namespace(), self.get_name());

        let result: RhiaqeyBufVec = self.redis.lock().await.get(settings_key).await?;
        debug!("encrypted settings retrieved");

        let keys = self.security.lock().await;

        let data = security::aes_decrypt(
            keys.no_once.as_slice(),
            keys.key.as_slice(),
            result.0.as_slice(),
        )?;

        let settings = MessageValue::Binary(data).decode::<T>()?;
        debug!("decrypted data decoded into settings");

        Ok(settings)
    }

    pub async fn setup(config: Env) -> RhiaqeyResult<Executor> {
        let redis_rs_client = connect_and_ping(&config.redis)?;
        let mut redis_rs_connection = redis_rs_client.get_connection()?;
        let security = Self::load_key(&config, &mut redis_rs_connection)?;
        let client = connect_and_ping_async(config.redis.clone()).await?;

        let mut executor = Executor {
            env: Arc::from(config),
            channels: Arc::from(RwLock::new(vec![])),
            redis: Arc::new(Mutex::new(client)),
            redis_rs: Arc::new(std::sync::Mutex::new(redis_rs_connection)),
            security: Arc::new(Mutex::new(security)),
        };

        let channels = executor.read_channels_async().await?;
        executor.set_channels_async(channels).await;

        Ok(executor)
    }

    pub fn extract_pubsub_message(&mut self, message: PubSubMessage) -> Option<RPCMessage> {
        serde_json::from_slice::<RPCMessage>(message.payload.as_slice()).ok()
    }

    pub async fn create_hub_to_publishers_pubsub_async(&mut self) -> RhiaqeyResult<PubSubStream> {
        let client = connect_and_ping_async(self.env.redis.clone()).await?;

        let key = topics::hub_to_publisher_pubsub_topic(
            self.env.namespace.clone(),
            self.env.name.clone(),
        );

        let stream = client.subscribe(key.clone()).await?;

        Ok(stream)
    }

    pub fn rpc(&self, namespace: String, message: RPCMessage) -> RhiaqeyResult<usize> {
        info!("broadcasting rpc message to all hubs");

        let clean_topic = topics::hub_raw_to_hub_clean_pubsub_topic(namespace);

        // Prepare to broadcast to all hubs that we have clean message
        let raw = message.ser_to_string()?;

        let reply: usize = self
            .redis_rs
            .lock()
            .unwrap()
            .publish(clean_topic.clone(), raw)
            .unwrap_or(0);

        trace!(
            "message sent to pubsub {} and received {} as reply",
            clean_topic,
            reply
        );

        Ok(reply)
    }

    pub async fn publish_async(
        &self,
        message: impl Into<StreamMessage>,
        options: ExecutorPublishOptions,
    ) -> RhiaqeyResult<usize> {
        info!("publishing message to all valid channels");

        let mut stream_msg: StreamMessage = message.into();

        // if self.is_debug() {
        stream_msg.publisher_id = Some(self.env.id.clone());
        // }

        let tag = stream_msg.tag.clone().unwrap_or(String::from(""));

        let redis = self.redis.lock().await;
        let channels = self.channels.read().await;

        let channel_size = channels.len();
        if channel_size == 0 {
            return Ok(0);
        }

        for channel in channels.iter() {
            stream_msg.channel = channel.name.to_string();

            if stream_msg.size.is_none() {
                stream_msg.size = Some(channel.size);
            }

            let topic = topics::publishers_to_hub_stream_topic(
                self.env.namespace.clone(),
                channel.name.to_string(),
            );

            info!(
                "publishing message channel={}, max_len={}, topic={}, timestamp={:?}",
                channel.name, channel.size, topic, stream_msg.timestamp,
            );

            let tms = stream_msg.timestamp.unwrap_or(0);

            let xadd_options = XAddOptions::default().trim_options(XTrimOptions::max_len(
                XTrimOperator::Approximately,
                // channel.size as i64,
                options.trim_threshold.unwrap_or(10000),
            ));

            let data = stream_msg.ser_to_string()?;

            let id: String = redis
                .xadd(
                    topic.clone(),
                    "*",
                    [
                        ("raw", data.clone()),
                        ("tag", tag.clone()),
                        ("tms", format!("{}", tms)),
                    ],
                    xadd_options,
                    // XAddOptions::default()
                )
                .await?;

            debug!(
                "sent message {} to channel {} in topic {}",
                id, channel.name, topic
            );
        }

        Ok(channel_size)
    }
}
