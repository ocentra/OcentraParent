#[cfg(windows)]
const AUTHORITY_SECRET_NAME: &str = "authority-secret";
#[cfg(windows)]
const AUTHORITY_SECRET_BYTES: usize = 32;

#[cfg(windows)]
use crate::platform::PlatformError;

#[cfg(windows)]
pub(super) fn load_or_create(registry_id: &str) -> Result<Vec<u8>, PlatformError> {
    match super::registry::read(registry_id, AUTHORITY_SECRET_NAME)? {
        Some(secret)
            if secret.len() == AUTHORITY_SECRET_BYTES && secret.iter().any(|byte| *byte != 0) =>
        {
            Ok(secret)
        }
        Some(_) => Err(PlatformError::Tampered),
        None => create_and_confirm(registry_id),
    }
}

#[cfg(windows)]
fn create_and_confirm(registry_id: &str) -> Result<Vec<u8>, PlatformError> {
    let mut secret = vec![0_u8; AUTHORITY_SECRET_BYTES];
    getrandom::fill(&mut secret).map_err(|_| PlatformError::Unavailable)?;
    if secret.iter().all(|byte| *byte == 0) {
        return Err(PlatformError::Unavailable);
    }
    super::registry::write(registry_id, AUTHORITY_SECRET_NAME, &secret)?;
    match super::registry::read(registry_id, AUTHORITY_SECRET_NAME)? {
        Some(confirmed) if confirmed == secret => Ok(secret),
        _ => Err(PlatformError::Tampered),
    }
}
