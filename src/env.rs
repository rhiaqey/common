use crate::error::RhiaqeyError;
use crate::redis::RedisSettings;
use crate::result::RhiaqeyResult;
use log::{debug, trace, warn};
use rsa::pkcs1::DecodeRsaPrivateKey;
use rsa::pkcs1::DecodeRsaPublicKey;
use rsa::{Oaep, RsaPrivateKey, RsaPublicKey};
use serde::Deserialize;
use std::fs;

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

    /// Optional. If not set, no encryption will be applied
    pub private_key: Option<String>,

    /// Optional. If not set, no decryption will be possible
    pub public_key: Option<String>,

    /// Optional since k8s is not required
    #[serde(flatten)]
    pub k8s: Option<KubernetesEnv>,

    /// The public-facing port that is only useful for gateways
    #[serde(default = "default_public_port")]
    pub public_port: Option<u16>,

    /// Internal port for all http interactions
    #[serde(default = "default_private_port")]
    pub private_port: Option<u16>,

    #[serde(flatten)]
    pub redis: RedisSettings,
}

impl Env {
    pub fn encrypt(&self, data: Vec<u8>) -> RhiaqeyResult<Vec<u8>> {
        if self.public_key.is_none() {
            trace!("no public key was found");
            return Ok(data);
        }

        let public_key_optional = self
            .public_key
            .as_ref()
            .ok_or(RhiaqeyError::from("failed to obtain public key"))?;

        let mut public_key_result = fs::read_to_string(public_key_optional);
        if let Err(err) = public_key_result {
            warn!("public key read from path error {err}");
            debug!("setting public key from env");
            public_key_result = Ok(public_key_optional.clone());
        }

        let public_key = public_key_result?;

        let rsa_public_key = RsaPublicKey::from_pkcs1_pem(&public_key)
            .map_err(|x| RhiaqeyError::from(x.to_string()))?;

        trace!("RSA public key is ready");

        let mut rng = rand::thread_rng();
        let padding = Oaep::new::<sha2::Sha256>();
        let enc_data = rsa_public_key
            .encrypt(&mut rng, padding, data.as_slice())
            .map_err(|x| RhiaqeyError::from(x.to_string()))?;

        trace!("data encrypted");

        Ok(enc_data)
    }

    pub fn decrypt(&self, data: Vec<u8>) -> RhiaqeyResult<Vec<u8>> {
        if self.private_key.is_none() {
            trace!("no private key was found");
            return Ok(data);
        }

        let private_key_optional = self
            .private_key
            .as_ref()
            .ok_or(RhiaqeyError::from("failed to obtain private_key key"))?;

        let mut private_key_result = fs::read_to_string(private_key_optional);
        if let Err(err) = private_key_result {
            warn!("private key read from path error {err}");
            debug!("setting private key from env");
            private_key_result = Ok(private_key_optional.to_string());
        }

        let private_key = private_key_result?;

        let rsa_private_key = RsaPrivateKey::from_pkcs1_pem(&private_key)
            .map_err(|x| RhiaqeyError::from(x.to_string()))?;

        trace!("RSA private key is ready");

        let padding = Oaep::new::<sha2::Sha256>();
        let dec_data = rsa_private_key
            .decrypt(padding, data.as_slice())
            .map_err(|x| RhiaqeyError::from(x.to_string()))?;

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
