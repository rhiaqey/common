use serde::{Deserialize, Serialize};

use aes_gcm_siv::{
    aead::{Aead, KeyInit},
    Aes256GcmSiv,
    Nonce, // Or `Aes128GcmSiv`
};
use anyhow::Context;
use rand::distr::Alphanumeric;
use rand::Rng;

pub fn generate_key() -> Vec<u8> {
    let key = Aes256GcmSiv::generate_key().unwrap();
    key.to_vec()
}

pub fn generate_nonce() -> Vec<u8> {
    let s: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(12)
        .map(char::from)
        .collect();
    s.into_bytes()
}

pub fn aes_encrypt(nonce: &[u8], key: &[u8], data: &[u8]) -> anyhow::Result<Vec<u8>> {
    let cipher =
        Aes256GcmSiv::new_from_slice(key).context("failed to create aes256gcm from slice")?;

    #[allow(deprecated)]
    let result = cipher
        .encrypt(Nonce::from_slice(nonce), data)
        .context("failed to encrypt")?;

    Ok(result)
}

pub fn aes_decrypt(nonce: &[u8], key: &[u8], data: &[u8]) -> anyhow::Result<Vec<u8>> {
    let cipher =
        Aes256GcmSiv::new_from_slice(key).context("failed to create aes256gcm from slice")?;

    #[allow(deprecated)]
    let result = cipher
        .decrypt(Nonce::from_slice(nonce), data)
        .context("failed to decrypt")?;

    Ok(result)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SecurityKey {
    pub no_once: Vec<u8>,
    pub key: Vec<u8>,
}

impl Default for SecurityKey {
    fn default() -> Self {
        let no_once = generate_nonce();
        let key = generate_key();
        (no_once, key).into()
    }
}

impl From<(Vec<u8>, Vec<u8>)> for SecurityKey {
    fn from(message: (Vec<u8>, Vec<u8>)) -> Self {
        SecurityKey {
            no_once: message.0,
            key: message.1,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::security::{aes_decrypt, aes_encrypt, generate_key, generate_nonce};

    #[test]
    fn can_encrypt() {
        let key = generate_key();
        let nonce = generate_nonce();
        let data = b"welcome to my nightmare";
        let result = aes_encrypt(nonce.as_slice(), key.as_slice(), data);
        assert!(result.is_ok());
    }

    #[test]
    fn can_decrypt() {
        let key = generate_key();
        let nonce = generate_nonce();
        let data = b"welcome to my nightmare";
        let result = aes_encrypt(nonce.as_slice(), key.as_slice(), data).unwrap();
        let decrypted = aes_decrypt(nonce.as_slice(), key.as_slice(), result.as_slice());
        assert!(decrypted.is_ok());
        assert_eq!(data, decrypted.unwrap().as_slice());
    }
}
