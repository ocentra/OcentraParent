#[cfg(windows)]
const AUTHORITY_SECRET_NAME: &str = "authority-secret";
#[cfg(windows)]
const AUTHORITY_SECRET_BYTES: usize = 32;

#[cfg(windows)]
use crate::platform::PlatformError;
#[cfg(windows)]
use zeroize::Zeroizing;

#[cfg(windows)]
pub(super) fn load_or_create(registry_id: &str) -> Result<Zeroizing<Vec<u8>>, PlatformError> {
    match super::registry::read(registry_id, AUTHORITY_SECRET_NAME)? {
        Some(secret) => validate_secret(Zeroizing::new(secret)),
        None => create_and_confirm(registry_id),
    }
}

#[cfg(windows)]
fn create_and_confirm(registry_id: &str) -> Result<Zeroizing<Vec<u8>>, PlatformError> {
    let mut secret = Zeroizing::new(vec![0_u8; AUTHORITY_SECRET_BYTES]);
    getrandom::fill(secret.as_mut_slice()).map_err(|_random_error| PlatformError::Unavailable)?;
    if secret.iter().all(|byte| *byte == 0) {
        return Err(PlatformError::Unavailable);
    }
    super::registry::write(registry_id, AUTHORITY_SECRET_NAME, secret.as_ref())?;
    match super::registry::read(registry_id, AUTHORITY_SECRET_NAME)? {
        Some(confirmed) if confirmed.as_slice() == secret.as_slice() => Ok(secret),
        _ => Err(PlatformError::Tampered),
    }
}

#[cfg(windows)]
fn validate_secret(secret: Zeroizing<Vec<u8>>) -> Result<Zeroizing<Vec<u8>>, PlatformError> {
    if secret.len() == AUTHORITY_SECRET_BYTES && secret.iter().any(|byte| *byte != 0) {
        Ok(secret)
    } else {
        Err(PlatformError::Tampered)
    }
}
