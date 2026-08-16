use base64::prelude::{Engine as _, BASE64_URL_SAFE_NO_PAD};
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Key, XChaCha20Poly1305, XNonce,
};
use sha2::{Digest, Sha256};

use crate::journal_error::JournalError;

pub const JOURNAL_KEY_BYTES: usize = 32;

#[derive(Clone)]
pub struct JournalKey([u8; JOURNAL_KEY_BYTES]);

impl JournalKey {
    pub fn generate() -> Self {
        let key = XChaCha20Poly1305::generate_key(&mut OsRng);
        Self(key.into())
    }

    pub fn from_bytes(bytes: [u8; JOURNAL_KEY_BYTES]) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; JOURNAL_KEY_BYTES] {
        &self.0
    }
}

pub struct EncryptedPayload {
    pub nonce: String,
    pub ciphertext: String,
    pub digest: String,
}

pub fn encrypt_payload(
    key: &JournalKey,
    plaintext: &[u8],
) -> Result<EncryptedPayload, JournalError> {
    let cipher = cipher_from_key(key);
    let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);
    let ciphertext = cipher
        .encrypt(&nonce, plaintext)
        .map_err(|_error| JournalError::Crypto)?;
    let digest = Sha256::digest(plaintext);

    Ok(EncryptedPayload {
        nonce: BASE64_URL_SAFE_NO_PAD.encode(nonce.as_slice()),
        ciphertext: BASE64_URL_SAFE_NO_PAD.encode(ciphertext),
        digest: BASE64_URL_SAFE_NO_PAD.encode(digest),
    })
}

pub fn decrypt_payload(
    key: &JournalKey,
    nonce: &str,
    ciphertext: &str,
) -> Result<Vec<u8>, JournalError> {
    let nonce_bytes = BASE64_URL_SAFE_NO_PAD.decode(nonce)?;
    if nonce_bytes.len() != ocentra_parent_agent_protocol::constants::journal::XCHACHA20_NONCE_BYTES
    {
        return Err(JournalError::Crypto);
    }
    let ciphertext_bytes = BASE64_URL_SAFE_NO_PAD.decode(ciphertext)?;
    let cipher = cipher_from_key(key);
    cipher
        .decrypt(XNonce::from_slice(&nonce_bytes), ciphertext_bytes.as_ref())
        .map_err(|_error| JournalError::Crypto)
}

fn cipher_from_key(key: &JournalKey) -> XChaCha20Poly1305 {
    XChaCha20Poly1305::new(Key::from_slice(&key.0))
}
