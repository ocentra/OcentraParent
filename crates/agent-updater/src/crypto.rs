use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand_core::{OsRng, RngCore};
use sha2::{Digest, Sha256};

use crate::constants::ED25519_ALGORITHM;
use crate::error::UpdaterError;

pub struct KeyPair {
    pub private_key_base64: String,
    pub public_key_base64: String,
}

pub fn generate_key_pair() -> KeyPair {
    let mut private_key = [0_u8; 32];
    OsRng.fill_bytes(&mut private_key);
    let signing_key = SigningKey::from_bytes(&private_key);
    let verifying_key = signing_key.verifying_key();
    KeyPair {
        private_key_base64: STANDARD.encode(signing_key.to_bytes()),
        public_key_base64: STANDARD.encode(verifying_key.to_bytes()),
    }
}

pub fn derive_public_key(private_key_base64: &str) -> Result<String, UpdaterError> {
    let signing_key = decode_signing_key(private_key_base64)?;
    Ok(STANDARD.encode(signing_key.verifying_key().to_bytes()))
}

pub fn key_id(public_key_base64: &str) -> Result<String, UpdaterError> {
    let public_key = decode_verifying_key(public_key_base64)?;
    let digest = Sha256::digest(public_key.as_bytes());
    Ok(digest[..16]
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect())
}

pub fn sign_bytes(
    payload: &[u8],
    private_key_base64: &str,
) -> Result<(String, String), UpdaterError> {
    let signing_key = decode_signing_key(private_key_base64)?;
    let public_key_base64 = STANDARD.encode(signing_key.verifying_key().to_bytes());
    let signature = signing_key.sign(payload);
    Ok((
        STANDARD.encode(signature.to_bytes()),
        key_id(&public_key_base64)?,
    ))
}

pub fn verify_bytes(
    payload: &[u8],
    signature_base64: &str,
    public_key_base64: &str,
    expected_key_id: &str,
    algorithm: &str,
) -> Result<(), UpdaterError> {
    if algorithm != ED25519_ALGORITHM {
        return Err(UpdaterError::Crypto(format!(
            "unsupported manifest signature algorithm: {algorithm}"
        )));
    }
    let actual_key_id = key_id(public_key_base64)?;
    if actual_key_id != expected_key_id {
        return Err(UpdaterError::Crypto(
            "manifest signature key id does not match trusted key".to_owned(),
        ));
    }
    let verifying_key = decode_verifying_key(public_key_base64)?;
    let signature_bytes = STANDARD.decode(signature_base64).map_err(|error| {
        UpdaterError::Crypto(format!("manifest signature is not valid base64: {error}"))
    })?;
    let signature = Signature::from_slice(&signature_bytes).map_err(|error| {
        UpdaterError::Crypto(format!(
            "manifest signature does not have Ed25519 length: {error}"
        ))
    })?;
    verifying_key.verify(payload, &signature).map_err(|error| {
        UpdaterError::Crypto(format!("manifest signature verification failed: {error}"))
    })
}

fn decode_signing_key(private_key_base64: &str) -> Result<SigningKey, UpdaterError> {
    let bytes = STANDARD.decode(private_key_base64).map_err(|error| {
        UpdaterError::Crypto(format!("private signing key is not valid base64: {error}"))
    })?;
    let key_bytes: [u8; 32] = bytes.try_into().map_err(|error| {
        UpdaterError::Crypto(format!("private signing key must be 32 bytes: {error:?}"))
    })?;
    Ok(SigningKey::from_bytes(&key_bytes))
}

fn decode_verifying_key(public_key_base64: &str) -> Result<VerifyingKey, UpdaterError> {
    let bytes = STANDARD.decode(public_key_base64).map_err(|error| {
        UpdaterError::Crypto(format!("public signing key is not valid base64: {error}"))
    })?;
    let key_bytes: [u8; 32] = bytes.try_into().map_err(|error| {
        UpdaterError::Crypto(format!("public signing key must be 32 bytes: {error:?}"))
    })?;
    VerifyingKey::from_bytes(&key_bytes).map_err(|error| {
        UpdaterError::Crypto(format!("public signing key is not an Ed25519 key: {error}"))
    })
}
