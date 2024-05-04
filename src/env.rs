use crate::redis::RedisSettings;
use anyhow::{bail, Context};
use log::{debug, trace, warn};
use rsa::pkcs1::DecodeRsaPrivateKey;
use rsa::pkcs1::DecodeRsaPublicKey;
use rsa::{Oaep, RsaPrivateKey, RsaPublicKey};
use rusty_ulid::generate_ulid_string;
use serde::Deserialize;
use std::{fs, process};

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

fn default_id() -> String {
    generate_ulid_string()
}

fn default_name() -> String {
    format!("process-{}", process::id())
}

fn default_namespace() -> String {
    String::from("rhiaqey")
}

#[derive(Deserialize, Clone, Debug)]
pub struct Env {
    /// Each instance will have a different id
    #[serde(default = "default_id")]
    id: String,

    /// All deployment pods will have the same name
    #[serde(default = "default_name")]
    name: String,

    /// Namespace of the k8s installation
    #[serde(default = "default_namespace")]
    namespace: String,

    /// Optional. If not set, no encryption will be applied
    private_key: Option<String>,

    /// Optional. If not set, no decryption will be possible
    public_key: Option<String>,

    /// Optional since k8s is not required
    // #[serde(flatten)]
    // k8s: Option<KubernetesEnv>,

    /// The public-facing port that is only useful for gateways
    #[serde(default = "default_public_port")]
    public_port: Option<u16>,

    /// Internal port for all http interactions
    #[serde(default = "default_private_port")]
    private_port: Option<u16>,

    #[serde(flatten)]
    pub redis: RedisSettings,
}

impl Env {
    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_namespace(&self) -> String {
        self.namespace.clone()
    }

    pub fn get_private_port(&self) -> u16 {
        self.private_port.unwrap_or(default_private_port().unwrap())
    }

    pub fn get_public_port(&self) -> u16 {
        self.public_port.unwrap_or(default_public_port().unwrap())
    }

    pub fn encrypt(&self, data: Vec<u8>) -> anyhow::Result<Vec<u8>> {
        trace!("encrypting data with public key: {}", self.public_key.is_some());

        if self.public_key.is_none() {
            bail!("no public key was found");
        }

        let Some(public_key_optional) = self.public_key.as_ref() else {
            bail!("failed to obtain public key")
        };

        let mut public_key_result = fs::read_to_string(public_key_optional);
        if let Err(err) = public_key_result {
            warn!("public key read from path error {err}");
            warn!("setting public key from env");
            public_key_result = Ok(public_key_optional.clone());
        }

        let public_key = public_key_result.context("fail to read public key")?;

        let rsa_public_key =
            RsaPublicKey::from_pkcs1_pem(&public_key).context("failed to create rsa public key")?;

        trace!("RSA public key is ready");

        let mut rng = rand::thread_rng();
        let padding = Oaep::new::<sha2::Sha256>();
        let enc_data = rsa_public_key
            .encrypt(&mut rng, padding, data.as_slice())
            .context("failed to encrypt data")?;

        trace!("data encrypted");

        Ok(enc_data)
    }

    pub fn decrypt(&self, data: Vec<u8>) -> anyhow::Result<Vec<u8>> {
        trace!("decrypting data with private key: {}", self.private_key.is_some());

        if self.private_key.is_none() {
            bail!("no private key was found");
        }

        let Some(private_key_optional) = self.private_key.as_ref() else {
            bail!("failed to obtain private_key key");
        };

        let mut private_key_result = fs::read_to_string(private_key_optional);
        if let Err(err) = private_key_result {
            warn!("private key read from path error {err}");
            debug!("setting private key from env");
            private_key_result = Ok(private_key_optional.to_string());
        }

        let private_key = private_key_result.context("failed to ready private key")?;

        let rsa_private_key = RsaPrivateKey::from_pkcs1_pem(&private_key)
            .context("failed to create rsa private key")?;

        trace!("RSA private key is ready");

        let padding = Oaep::new::<sha2::Sha256>();
        let dec_data = rsa_private_key
            .decrypt(padding, data.as_slice())
            .context("failed to decrypt data")?;

        trace!("data decrypted");

        Ok(dec_data)
    }
}

pub fn parse_env() -> Env {
    match envy::from_env::<Env>() {
        Ok(env) => env,
        Err(e) => panic!("env failed to parse: {}", e),
    }
}
