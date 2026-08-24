use crate::authority::AuthorityError;
use ocentra_protected_capability_custody_protocol::request::{ExpectedGenerations, RequestKind};
#[cfg(windows)]
use zeroize::Zeroizing;

use super::super::platform;
use super::codec;

const BINDING_NAME_PREFIX: &str = "binding-";
const CURRENT_BINDING_ENTROPY_DOMAIN: &[u8] = b"ocentra.pcc.current-binding.v1";

#[cfg(windows)]
pub(super) fn load_or_create_generations(
    registry_id: &str,
    lookup_digest: &[u8; 32],
    kind: RequestKind,
) -> Result<(ExpectedGenerations, bool), AuthorityError> {
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
            codec::decode(plaintext.as_ref()).map(|generations| (generations, false))
        }
        None if kind == RequestKind::Prepare => {
            create_generations(registry_id, lookup_digest, &value_name)
                .map(|generations| (generations, true))
        }
        None => Err(AuthorityError::Rejected),
    }
}

#[cfg(not(windows))]
pub(super) fn load_or_create_generations(
    _registry_id: &str,
    _lookup_digest: &[u8; 32],
    _kind: RequestKind,
) -> Result<(ExpectedGenerations, bool), AuthorityError> {
    Err(AuthorityError::Unavailable)
}

#[cfg(windows)]
fn create_generations(
    registry_id: &str,
    lookup_digest: &[u8; 32],
    value_name: &str,
) -> Result<ExpectedGenerations, AuthorityError> {
    let generations = random_generations()?;
    let plaintext = Zeroizing::new(codec::encode(generations));
    let sealed = platform::encrypt_dpapi(
        registry_id,
        plaintext.as_ref(),
        &binding_entropy(registry_id, lookup_digest),
    )
    .map_err(map_platform_error)?;
    platform::write_registry_value(registry_id, value_name, &sealed).map_err(map_platform_error)?;
    Ok(generations)
}

#[cfg(windows)]
fn random_generations() -> Result<ExpectedGenerations, AuthorityError> {
    ExpectedGenerations::try_new(
        random_nonzero_u64()?,
        random_nonzero_u64()?,
        random_nonzero_u64()?,
        random_nonzero_u64()?,
    )
    .map_err(map_protocol_error)
}

#[cfg(windows)]
fn random_nonzero_u64() -> Result<u64, AuthorityError> {
    let mut bytes = [0_u8; 8];
    getrandom::fill(&mut bytes).map_err(map_random_error)?;
    let value = u64::from_be_bytes(bytes);
    if value == 0 {
        return Err(AuthorityError::Unavailable);
    }
    Ok(value)
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

#[cfg(windows)]
fn map_protocol_error(
    _error: ocentra_protected_capability_custody_protocol::types::ProtocolError,
) -> AuthorityError {
    AuthorityError::Rejected
}

#[cfg(windows)]
fn map_random_error(_error: getrandom::Error) -> AuthorityError {
    AuthorityError::Unavailable
}
