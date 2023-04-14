use crate::redis::RedisSettings;
use serde::Deserialize;

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

fn default_public_port() -> Option<u16> {
    Some(3000)
}

fn default_private_port() -> Option<u16> {
    Some(3001)
}

#[derive(Deserialize, Clone, Debug)]
pub struct Env {
    /// Each pod will have a different id
    pub id: String,

    /// All deployment pods will have the same name
    pub name: String,

    /// Namespace of the k8s installation
    pub namespace: String,

    /// Required
    pub secret: String,

    #[serde(flatten)]
    pub k8s: KubernetesEnv,

    /// The public facing port that is only useful for gateways
    #[serde(default = "default_public_port")]
    pub public_port: Option<u16>,

    /// Internal port for all http interactions
    #[serde(default = "default_private_port")]
    pub private_port: Option<u16>,

    #[serde(flatten)]
    pub redis: RedisSettings,
}

pub fn parse_env() -> Env {
    match envy::from_env::<Env>() {
        Ok(env) => env,
        Err(e) => panic!("env failed to parse: {}", e),
    }
}
