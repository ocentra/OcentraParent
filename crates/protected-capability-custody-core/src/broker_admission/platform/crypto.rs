#[cfg(windows)]
use crate::platform::identity::DatabaseIdentity;
#[cfg(windows)]
use crate::platform::PlatformError;

#[cfg(windows)]
const STATE_ENTROPY_DOMAIN: &[u8] = b"ocentra.pcc.state.v1";
#[cfg(windows)]
const RECORD_ENTROPY_DOMAIN: &[u8] = b"ocentra.pcc.record.v1";

#[cfg(windows)]
pub(super) fn encrypt(plaintext: &[u8], entropy: &[u8]) -> Result<Vec<u8>, PlatformError> {
    match windows_dpapi::encrypt_data(plaintext, windows_dpapi::Scope::User, Some(entropy)) {
        Ok(sealed) => Ok(sealed),
        Err(error) => {
            drop(error);
            Err(PlatformError::Unavailable)
        }
    }
}

#[cfg(windows)]
pub(super) fn decrypt(sealed: &[u8], entropy: &[u8]) -> Result<Vec<u8>, PlatformError> {
    match windows_dpapi::decrypt_data(sealed, windows_dpapi::Scope::User, Some(entropy)) {
        Ok(plaintext) => Ok(plaintext),
        Err(error) => {
            drop(error);
            Err(PlatformError::Tampered)
        }
    }
}

#[cfg(windows)]
pub(super) fn encrypt_state(registry_id: &str, plaintext: &[u8]) -> Result<Vec<u8>, PlatformError> {
    encrypt(plaintext, &state_entropy(registry_id))
}

#[cfg(windows)]
pub(super) fn decrypt_state(registry_id: &str, sealed: &[u8]) -> Result<Vec<u8>, PlatformError> {
    decrypt(sealed, &state_entropy(registry_id))
}

#[cfg(windows)]
pub(super) fn protect_record(
    registry_id: &str,
    lookup_digest: &[u8; 32],
    database_identity: DatabaseIdentity,
    plaintext: &[u8],
) -> Result<Vec<u8>, PlatformError> {
    encrypt(
        plaintext,
        &record_entropy(registry_id, lookup_digest, database_identity),
    )
}

#[cfg(windows)]
pub(super) fn unprotect_record(
    registry_id: &str,
    lookup_digest: &[u8; 32],
    database_identity: DatabaseIdentity,
    sealed: &[u8],
) -> Result<Vec<u8>, PlatformError> {
    decrypt(
        sealed,
        &record_entropy(registry_id, lookup_digest, database_identity),
    )
}

#[cfg(windows)]
fn state_entropy(registry_id: &str) -> Vec<u8> {
    let mut entropy = STATE_ENTROPY_DOMAIN.to_vec();
    entropy.extend_from_slice(registry_id.as_bytes());
    entropy
}

#[cfg(windows)]
fn record_entropy(
    registry_id: &str,
    lookup_digest: &[u8; 32],
    database_identity: DatabaseIdentity,
) -> Vec<u8> {
    let mut entropy = RECORD_ENTROPY_DOMAIN.to_vec();
    entropy.extend_from_slice(registry_id.as_bytes());
    entropy.extend_from_slice(lookup_digest);
    entropy.extend_from_slice(database_identity.as_bytes());
    entropy
}
