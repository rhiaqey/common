use rhiaqey_sdk::channel::ChannelList;
use serde::Deserialize;
use crate::redis::RedisSettings;

#[derive(Deserialize, Default, Clone, Debug)]
pub struct KubernetesEnv {
    pub k8s_pod_uid: Option<String>,
    pub k8s_pod_name: Option<String>,
    pub k8s_pod_namespace: Option<String>,
    pub k8s_pod_ip: Option<String>,
    pub k8s_pod_service_account: Option<String>,
    pub k8s_node_name: Option<String>,
    pub k8s_node_ip: Option<String>,
}

fn default_debug() -> bool {
    false
}

fn default_public_port() -> u16 {
    3000
}

fn default_private_port() -> u16 {
    3001
}

fn default_host() -> String {
    String::from("localhost")
}

// fn default_size() -> ClusterSize {
//    ClusterSize::Small
// }

fn default_topic_suffix() -> String { String::from("out") }

#[derive(Deserialize, Clone, Debug)]
pub struct Env {

    pub id: String,

    pub name: String,

    #[serde(default = "default_debug")]
    pub debug: bool,

    pub namespace: String,

    #[serde(flatten)]
    pub k8s: KubernetesEnv,

    #[serde(default = "default_public_port")]
    pub public_port: u16,

    #[serde(default = "default_private_port")]
    pub private_port: u16,

    // pub cluster: Uuid,

    // pub user: Uuid,

    // pub artifact: Uuid,

    // pub deployment: Uuid,

    // pub secret: String,

    #[serde(default = "default_topic_suffix")]
    pub topic_suffix: String,

    // #[serde(rename = "type")]
    // pub deployment_type: DeploymentType,

    #[serde(default = "default_host")]
    pub host: String,

    // #[serde(default = "default_size")]
    // pub size: ClusterSize,

    #[serde(flatten)]
    pub redis: RedisSettings,

    #[serde(default)]
    pub channels: ChannelList,

}

pub fn parse_env() -> Env {
    match envy::from_env::<Env>() {
        Ok(env) => env,
        Err(e) => panic!("env failed to parse: {}", e),
    }
}
