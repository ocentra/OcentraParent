use crate::authority::AuthorityError;
use ocentra_protected_capability_custody_protocol::request::ExpectedGenerations;
#[cfg(windows)]
use zeroize::Zeroizing;

use super::super::platform;
use super::codec;

const BINDING_NAME_PREFIX: &str = "binding-";
const CURRENT_BINDING_ENTROPY_DOMAIN: &[u8] = b"ocentra.pcc.current-binding.v1";

#[cfg(windows)]
pub(super) fn load_generations(
    registry_id: &str,
    lookup_digest: &[u8; 32],
) -> Result<Option<ExpectedGenerations>, AuthorityError> {
    let value_name = binding_name(lookup_digest);
    match platform::read_registry_value(registry_id, &value_name).map_err(map_platform_error)? {
        Some(sealed) => {
            let plaintext = Zeroizing::new(
                platform::decrypt_dpapi(
                    registry_id,
                    &sealed,
                    &binding_entropy(registry_id, lookup_digest),
                )
                .map_err(map_platform_error)?,
            );
            codec::decode(plaintext.as_ref()).map(Some)
        }
        None => Ok(None),
    }
}

#[cfg(not(windows))]
pub(super) fn load_generations(
    _registry_id: &str,
    _lookup_digest: &[u8; 32],
) -> Result<Option<ExpectedGenerations>, AuthorityError> {
    Err(AuthorityError::Unavailable)
}

#[cfg(windows)]
fn binding_name(lookup_digest: &[u8; 32]) -> String {
    let mut name = String::from(BINDING_NAME_PREFIX);
    name.push_str(&platform::hex(lookup_digest));
    name
}

#[cfg(windows)]
fn binding_entropy(registry_id: &str, lookup_digest: &[u8; 32]) -> Vec<u8> {
    let mut entropy = CURRENT_BINDING_ENTROPY_DOMAIN.to_vec();
    entropy.extend_from_slice(registry_id.as_bytes());
    entropy.extend_from_slice(lookup_digest);
    entropy
}

#[cfg(windows)]
fn map_platform_error(_error: crate::platform::PlatformError) -> AuthorityError {
    AuthorityError::Unavailable
}
