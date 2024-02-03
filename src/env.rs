use std::fs;
use rsa::pkcs8::DecodePrivateKey;
use rsa::{Oaep, RsaPrivateKey, RsaPublicKey};
use crate::redis::RedisSettings;
use serde::Deserialize;
use crate::error::RhiaqeyError;

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
    pub private_key: String,

    /// Optional since k8s is not required
    #[serde(flatten)]
    pub k8s: Option<KubernetesEnv>,

    /// The public facing port that is only useful for gateways
    #[serde(default = "default_public_port")]
    pub public_port: Option<u16>,

    /// Internal port for all http interactions
    #[serde(default = "default_private_port")]
    pub private_port: Option<u16>,

    #[serde(flatten)]
    pub redis: RedisSettings,
}

impl Env {
    pub fn encrypt(&self, data: Vec<u8>) -> Result<Vec<u8>, RhiaqeyError> {
        let mut rng = rand::thread_rng();
        let padding = Oaep::new::<sha2::Sha256>();

        // Check if it is a path to a private key by trying to load from filesystem
        let private_key_content = fs::read_to_string(self.private_key.clone())
            .unwrap_or(self.private_key.clone());
        let private_key = private_key_content.as_str();

        let rsa_private_key = RsaPrivateKey::from_pkcs8_pem(private_key)
            .map_err(|x| RhiaqeyError{
                code: None,
                message: x.to_string(),
                error: Some(Box::new(x)),
            }
        )?;

        let rsa_public_key = RsaPublicKey::from(&rsa_private_key);
        let enc_data = rsa_public_key.encrypt(&mut rng, padding, data.as_slice())
            .map_err(|x| RhiaqeyError{
                code: None,
                message: x.to_string(),
                error: Some(Box::new(x))
            }
        )?;

        Ok(enc_data)
    }

    pub fn decrypt(&self, data: Vec<u8>) -> Result<Vec<u8>, RhiaqeyError> {
        let padding = Oaep::new::<sha2::Sha256>();

        let rsa_private_key = RsaPrivateKey::from_pkcs8_pem(self.private_key.as_str())
            .map_err(|x| RhiaqeyError{
                code: None,
                message: x.to_string(),
                error: Some(Box::new(x)),
            }
        )?;

        let dec_data = rsa_private_key.decrypt(padding, data.as_slice())
            .map_err(|x| RhiaqeyError{
                code: None,
                message: x.to_string(),
                error: Some(Box::new(x))
            }
        )?;

        Ok(dec_data)
    }
}

pub fn parse_env() -> Env {
    match envy::from_env::<Env>() {
        Ok(env) => env,
        Err(e) => panic!("env failed to parse: {}", e),
    }
}
