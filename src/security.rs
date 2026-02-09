use serde::{Deserialize, Serialize};

use aes_gcm_siv::{
    Aes256GcmSiv, Key,
    Nonce, // Or `Aes128GcmSiv`
    aead::{Aead, KeyInit},
};
use rand::RngExt;
use rand::distr::Alphanumeric;

pub fn generate_key() -> Vec<u8> {
    use aes_gcm_siv::aead::KeySizeUser;
    let mut key = vec![0u8; Aes256GcmSiv::key_size()];
    rand::rng().fill(&mut key);
    key
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
    let key = Key::<Aes256GcmSiv>::try_from(key).map_err(|err| anyhow::anyhow!(err))?;
    let cipher = Aes256GcmSiv::new(&key);

    let result = cipher
        .encrypt(&Nonce::try_from(nonce).map_err(|err| anyhow::anyhow!(err))?, data)
        .map_err(|err| anyhow::anyhow!(err))?;

    Ok(result)
}

pub fn aes_decrypt(nonce: &[u8], key: &[u8], data: &[u8]) -> anyhow::Result<Vec<u8>> {
    let key = Key::<Aes256GcmSiv>::try_from(key).map_err(|err| anyhow::anyhow!(err))?;
    let cipher = Aes256GcmSiv::new(&key);

    let result = cipher
        .decrypt(&Nonce::try_from(nonce).map_err(|err| anyhow::anyhow!(err))?, data)
        .map_err(|err| anyhow::anyhow!(err))?;

    Ok(result)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SecurityKey {
    pub nonce: Vec<u8>,
    pub key: Vec<u8>,
}

impl Default for SecurityKey {
    fn default() -> Self {
        let nonce = generate_nonce();
        let key = generate_key();
        (nonce, key).into()
    }
}

impl From<(Vec<u8>, Vec<u8>)> for SecurityKey {
    fn from(message: (Vec<u8>, Vec<u8>)) -> Self {
        SecurityKey {
            nonce: message.0,
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
