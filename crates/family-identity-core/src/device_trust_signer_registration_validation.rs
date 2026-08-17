use std::fmt::Write as _;

use ed25519_dalek::VerifyingKey;
use sha2::{Digest, Sha256};

use crate::device_trust_lifecycle::DeviceTrustLifecycleError;

const ACTIVE: &str = "active";
const REVOKED: &str = "revoked";

pub(crate) struct ValidatedSignerKey {
    pub(crate) public_key: [u8; 32],
    pub(crate) key_id: String,
    pub(crate) sha256: String,
}

pub(crate) struct PersistedSignerValidation<'a> {
    pub(crate) family_id: &'a str,
    pub(crate) trust_subject: &'a str,
    pub(crate) parent_device_id: &'a str,
    pub(crate) child_device_id: &'a str,
    pub(crate) installation_id: &'a str,
    pub(crate) signer_public_key: &'a [u8],
    pub(crate) signer_key_id: &'a str,
    pub(crate) signer_key_sha256: &'a str,
    pub(crate) registration_receipt: &'a str,
    pub(crate) parent_presence_receipt: &'a str,
    pub(crate) parent_intent_digest: &'a str,
    pub(crate) parent_route_id: &'a str,
    pub(crate) credential_id: &'a str,
    pub(crate) credential_algorithm: i64,
    pub(crate) credential_sign_count: i64,
    pub(crate) lifecycle_generation: i64,
    pub(crate) installation_binding_generation: i64,
    pub(crate) authority_generation: i64,
    pub(crate) registration_state: &'a str,
}

pub(crate) fn validate_signer_key(
    value: &[u8],
) -> Result<ValidatedSignerKey, DeviceTrustLifecycleError> {
    let public_key: [u8; 32] = value
        .try_into()
        .map_err(|_error| DeviceTrustLifecycleError::InvalidSignerKey)?;
    let verifying_key = VerifyingKey::from_bytes(&public_key)
        .map_err(|_error| DeviceTrustLifecycleError::InvalidSignerKey)?;
    if verifying_key.is_weak() {
        return Err(DeviceTrustLifecycleError::InvalidSignerKey);
    }
    let digest = Sha256::digest(public_key);
    let key_id_bytes = digest
        .get(..16)
        .ok_or(DeviceTrustLifecycleError::InvalidSignerKey)?;
    Ok(ValidatedSignerKey {
        public_key,
        key_id: hex_encode(key_id_bytes)?,
        sha256: hex_encode(&digest)?,
    })
}

pub(crate) fn validate_persisted_signer(
    row: &PersistedSignerValidation<'_>,
) -> Result<[u8; 32], DeviceTrustLifecycleError> {
    validate_canonical_identity(row.family_id)?;
    validate_canonical_identity(row.trust_subject)?;
    validate_canonical_identity(row.parent_device_id)?;
    validate_canonical_identity(row.child_device_id)?;
    validate_canonical_identity(row.installation_id)?;
    validate_lower_hex(row.signer_key_id, 32)?;
    validate_lower_hex(row.signer_key_sha256, 64)?;
    validate_lower_hex(row.registration_receipt, 64)?;
    validate_receipt(row.parent_presence_receipt)?;
    validate_digest(row.parent_intent_digest)?;
    validate_canonical_identity(row.parent_route_id)?;
    validate_credential_id(row.credential_id)?;
    if row.credential_algorithm != -8 || row.credential_sign_count < 0 {
        return Err(DeviceTrustLifecycleError::Unavailable);
    }
    if row.lifecycle_generation <= 0
        || row.installation_binding_generation <= 0
        || row.authority_generation <= 0
        || (row.registration_state != ACTIVE && row.registration_state != REVOKED)
    {
        return Err(DeviceTrustLifecycleError::Unavailable);
    }
    let signer = validate_signer_key(row.signer_public_key)?;
    if signer.key_id != row.signer_key_id || signer.sha256 != row.signer_key_sha256 {
        return Err(DeviceTrustLifecycleError::Unavailable);
    }
    Ok(signer.public_key)
}

pub(crate) fn validate_receipt(value: &str) -> Result<(), DeviceTrustLifecycleError> {
    validate_lower_hex(value, 64)
}

pub(crate) fn validate_digest(value: &str) -> Result<(), DeviceTrustLifecycleError> {
    validate_lower_hex(value, 64)
}

pub(crate) fn validate_credential_id(value: &str) -> Result<(), DeviceTrustLifecycleError> {
    (value.len() <= 512
        && !value.is_empty()
        && value.trim() == value
        && value.is_ascii()
        && value.bytes().all(|byte| (0x21..=0x7e).contains(&byte)))
    .then_some(())
    .ok_or(DeviceTrustLifecycleError::InvalidIdentity)
}

pub(crate) fn validate_canonical_identity(value: &str) -> Result<(), DeviceTrustLifecycleError> {
    (value.len() <= 256
        && !value.is_empty()
        && value.trim() == value
        && value.is_ascii()
        && value.bytes().all(|byte| (0x21..=0x7e).contains(&byte)))
    .then_some(())
    .ok_or(DeviceTrustLifecycleError::InvalidIdentity)
}

pub(crate) fn random_receipt() -> Result<String, DeviceTrustLifecycleError> {
    let mut entropy = [0_u8; 32];
    getrandom::fill(&mut entropy).map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    hex_encode(&entropy)
}

fn validate_lower_hex(value: &str, expected_len: usize) -> Result<(), DeviceTrustLifecycleError> {
    (value.len() == expected_len
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)))
    .then_some(())
    .ok_or(DeviceTrustLifecycleError::Unavailable)
}

fn hex_encode(bytes: &[u8]) -> Result<String, DeviceTrustLifecycleError> {
    let mut encoded = String::with_capacity(bytes.len().saturating_mul(2));
    for byte in bytes {
        write!(&mut encoded, "{byte:02x}")
            .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    }
    Ok(encoded)
}
