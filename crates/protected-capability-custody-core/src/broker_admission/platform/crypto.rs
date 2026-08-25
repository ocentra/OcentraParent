#[cfg(windows)]
use crate::platform::identity::DatabaseIdentity;
#[cfg(windows)]
use crate::platform::PlatformError;
#[cfg(windows)]
use zeroize::Zeroizing;

#[cfg(windows)]
const STATE_ENTROPY_DOMAIN: &[u8] = b"ocentra.pcc.state.v1";
#[cfg(windows)]
const RECORD_ENTROPY_DOMAIN: &[u8] = b"ocentra.pcc.record.v1";
#[cfg(windows)]
const REGISTRY_VALUE_ENTROPY_DOMAIN: &[u8] = b"ocentra.pcc.registry-value.v1";

#[cfg(windows)]
fn encrypt(plaintext: &[u8], entropy: &[u8]) -> Result<Vec<u8>, PlatformError> {
    match windows_dpapi::encrypt_data(plaintext, windows_dpapi::Scope::Machine, Some(entropy)) {
        Ok(sealed) => Ok(sealed),
        Err(error) => {
            drop(error);
            Err(PlatformError::Unavailable)
        }
    }
}

#[cfg(windows)]
fn decrypt(sealed: &[u8], entropy: &[u8]) -> Result<Vec<u8>, PlatformError> {
    match windows_dpapi::decrypt_data(sealed, windows_dpapi::Scope::Machine, Some(entropy)) {
        Ok(plaintext) => Ok(plaintext),
        Err(error) => {
            drop(error);
            Err(PlatformError::Tampered)
        }
    }
}

#[cfg(windows)]
pub(super) fn encrypt_state(registry_id: &str, plaintext: &[u8]) -> Result<Vec<u8>, PlatformError> {
    let entropy = bound_entropy(registry_id, STATE_ENTROPY_DOMAIN, &[])?;
    encrypt(plaintext, entropy.as_ref())
}

#[cfg(windows)]
pub(super) fn decrypt_state(registry_id: &str, sealed: &[u8]) -> Result<Vec<u8>, PlatformError> {
    let entropy = bound_entropy(registry_id, STATE_ENTROPY_DOMAIN, &[])?;
    decrypt(sealed, entropy.as_ref())
}

#[cfg(windows)]
pub(super) fn protect_record(
    registry_id: &str,
    lookup_digest: &[u8; 32],
    database_identity: DatabaseIdentity,
    plaintext: &[u8],
) -> Result<Vec<u8>, PlatformError> {
    let entropy = record_entropy(registry_id, lookup_digest, database_identity)?;
    encrypt(plaintext, entropy.as_ref())
}

#[cfg(windows)]
pub(super) fn unprotect_record(
    registry_id: &str,
    lookup_digest: &[u8; 32],
    database_identity: DatabaseIdentity,
    sealed: &[u8],
) -> Result<Vec<u8>, PlatformError> {
    let entropy = record_entropy(registry_id, lookup_digest, database_identity)?;
    decrypt(sealed, entropy.as_ref())
}

#[cfg(windows)]
pub(super) fn encrypt_registry_value(
    registry_id: &str,
    context: &[u8],
    plaintext: &[u8],
) -> Result<Vec<u8>, PlatformError> {
    let entropy = bound_entropy(registry_id, REGISTRY_VALUE_ENTROPY_DOMAIN, context)?;
    encrypt(plaintext, entropy.as_ref())
}

#[cfg(windows)]
pub(super) fn decrypt_registry_value(
    registry_id: &str,
    context: &[u8],
    sealed: &[u8],
) -> Result<Vec<u8>, PlatformError> {
    let entropy = bound_entropy(registry_id, REGISTRY_VALUE_ENTROPY_DOMAIN, context)?;
    decrypt(sealed, entropy.as_ref())
}

#[cfg(windows)]
fn record_entropy(
    registry_id: &str,
    lookup_digest: &[u8; 32],
    database_identity: DatabaseIdentity,
) -> Result<Zeroizing<Vec<u8>>, PlatformError> {
    let mut context = Vec::with_capacity(32 + database_identity.as_bytes().len());
    context.extend_from_slice(lookup_digest);
    context.extend_from_slice(database_identity.as_bytes());
    bound_entropy(registry_id, RECORD_ENTROPY_DOMAIN, &context)
}

#[cfg(windows)]
fn bound_entropy(
    registry_id: &str,
    domain: &[u8],
    context: &[u8],
) -> Result<Zeroizing<Vec<u8>>, PlatformError> {
    let secret = crate::broker_admission::platform::secret::load_or_create(registry_id)?;
    let mut entropy = Zeroizing::new(Vec::with_capacity(
        domain
            .len()
            .saturating_add(secret.len())
            .saturating_add(registry_id.len())
            .saturating_add(context.len()),
    ));
    entropy.extend_from_slice(domain);
    entropy.extend_from_slice(secret.as_slice());
    entropy.extend_from_slice(registry_id.as_bytes());
    entropy.extend_from_slice(context);
    Ok(entropy)
}
