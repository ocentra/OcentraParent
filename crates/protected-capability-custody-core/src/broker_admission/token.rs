use crate::binding::BindingLocator;
use crate::custody::{PreparedCapability, PreparedTokenParts};
use crate::platform::PlatformError;
use ocentra_protected_capability_custody_protocol::types::OpaquePreparedToken;
#[cfg(windows)]
use zeroize::Zeroizing;

#[cfg(windows)]
use super::platform;

#[cfg(windows)]
const TOKEN_MAGIC: [u8; 4] = *b"OCPT";
#[cfg(windows)]
const TOKEN_VERSION: u16 = 2;
#[cfg(windows)]
const TOKEN_PLAINTEXT_BYTES: usize =
    ocentra_protected_capability_custody_protocol::constants::OPAQUE_TOKEN_BYTES + 78;
#[cfg(windows)]
const TOKEN_RECORD_OFFSET: usize =
    6 + ocentra_protected_capability_custody_protocol::constants::OPAQUE_TOKEN_BYTES;

pub(super) fn issue(
    registry_id: &str,
    prepared: PreparedCapability,
) -> Result<OpaquePreparedToken, PlatformError> {
    #[cfg(windows)]
    {
        let parts = prepared.into_token_parts();
        if let Some((token, stored)) = read_active(registry_id, &parts.lookup_digest)? {
            return if same_parts(&stored, &parts) {
                Ok(token)
            } else {
                Err(PlatformError::Tampered)
            };
        }
        let mut raw_token = Zeroizing::new(vec![
                0_u8;
                ocentra_protected_capability_custody_protocol::constants::OPAQUE_TOKEN_BYTES
            ]);
        getrandom::fill(raw_token.as_mut_slice()).map_err(map_random_error)?;
        if raw_token.iter().all(|byte| *byte == 0) {
            return Err(PlatformError::Unavailable);
        }
        let plaintext = Zeroizing::new(encode_entry(raw_token.as_ref(), &parts));
        let sealed = platform::encrypt_dpapi(
            registry_id,
            plaintext.as_ref(),
            &token_entropy(registry_id, &parts.lookup_digest),
        )?;
        platform::write_registry_value(registry_id, &token_name(&parts.lookup_digest), &sealed)?;
        OpaquePreparedToken::from_untrusted_wire_bytes(raw_token.to_vec())
            .map_err(map_protocol_error)
    }
    #[cfg(not(windows))]
    {
        let _registry_id = registry_id;
        let _prepared = prepared;
        Err(PlatformError::Unavailable)
    }
}

pub(super) fn redeem(
    registry_id: &str,
    digest: [u8; 32],
    locator: BindingLocator,
) -> Result<PreparedCapability, PlatformError> {
    #[cfg(windows)]
    {
        let lookup_digest = locator.lookup_digest();
        let (token, parts) =
            read_active(registry_id, &lookup_digest)?.ok_or(PlatformError::Rejected)?;
        if token.digest() != digest || parts.lookup_digest != lookup_digest {
            return Err(PlatformError::WrongBinding);
        }
        Ok(PreparedCapability::from_token_parts(&parts, locator))
    }
    #[cfg(not(windows))]
    {
        let _registry_id = registry_id;
        let _digest = digest;
        let _locator = locator;
        Err(PlatformError::Unavailable)
    }
}

pub(super) fn consume(
    registry_id: &str,
    lookup_digest: &[u8; 32],
    digest: [u8; 32],
) -> Result<(), PlatformError> {
    #[cfg(windows)]
    {
        let (token, _parts) =
            read_active(registry_id, lookup_digest)?.ok_or(PlatformError::Rejected)?;
        if token.digest() != digest {
            return Err(PlatformError::WrongBinding);
        }
        platform::delete_registry_value(registry_id, &token_name(lookup_digest))
    }
    #[cfg(not(windows))]
    {
        let _registry_id = registry_id;
        let _lookup_digest = lookup_digest;
        let _digest = digest;
        Err(PlatformError::Unavailable)
    }
}

#[cfg(windows)]
fn read_active(
    registry_id: &str,
    lookup_digest: &[u8; 32],
) -> Result<Option<(OpaquePreparedToken, PreparedTokenParts)>, PlatformError> {
    let Some(sealed) = platform::read_registry_value(registry_id, &token_name(lookup_digest))?
    else {
        return Ok(None);
    };
    let plaintext = Zeroizing::new(platform::decrypt_dpapi(
        registry_id,
        &sealed,
        &token_entropy(registry_id, lookup_digest),
    )?);
    decode_entry(plaintext.as_ref()).map(Some)
}

#[cfg(windows)]
fn encode_entry(raw_token: &[u8], parts: &PreparedTokenParts) -> [u8; TOKEN_PLAINTEXT_BYTES] {
    let mut bytes = [0_u8; TOKEN_PLAINTEXT_BYTES];
    bytes[..4].copy_from_slice(&TOKEN_MAGIC);
    bytes[4..6].copy_from_slice(&TOKEN_VERSION.to_be_bytes());
    bytes[6..TOKEN_RECORD_OFFSET].copy_from_slice(raw_token);
    bytes[TOKEN_RECORD_OFFSET..TOKEN_RECORD_OFFSET + 32].copy_from_slice(&parts.record_id);
    bytes[TOKEN_RECORD_OFFSET + 32..TOKEN_RECORD_OFFSET + 64].copy_from_slice(&parts.lookup_digest);
    bytes[TOKEN_RECORD_OFFSET + 64..].copy_from_slice(&parts.sequence.to_be_bytes());
    bytes
}

#[cfg(windows)]
fn decode_entry(
    plaintext: &[u8],
) -> Result<(OpaquePreparedToken, PreparedTokenParts), PlatformError> {
    if plaintext.len() != TOKEN_PLAINTEXT_BYTES
        || plaintext.get(..4) != Some(TOKEN_MAGIC.as_slice())
        || plaintext.get(4..6) != Some(TOKEN_VERSION.to_be_bytes().as_slice())
    {
        return Err(PlatformError::Tampered);
    }
    let raw_token = plaintext
        .get(6..TOKEN_RECORD_OFFSET)
        .ok_or(PlatformError::Tampered)?
        .to_vec();
    let token =
        OpaquePreparedToken::from_untrusted_wire_bytes(raw_token).map_err(map_protocol_error)?;
    let record_id = take_array(plaintext, TOKEN_RECORD_OFFSET)?;
    let lookup_digest = take_array(plaintext, TOKEN_RECORD_OFFSET + 32)?;
    let sequence = plaintext
        .get(TOKEN_RECORD_OFFSET + 64..TOKEN_PLAINTEXT_BYTES)
        .ok_or(PlatformError::Tampered)?
        .try_into()
        .map(u64::from_be_bytes)
        .map_err(map_slice_error)?;
    if record_id == [0_u8; 32] || lookup_digest == [0_u8; 32] || sequence == 0 {
        return Err(PlatformError::Tampered);
    }
    Ok((
        token,
        PreparedTokenParts {
            record_id,
            lookup_digest,
            sequence,
        },
    ))
}

#[cfg(windows)]
fn take_array<const LENGTH: usize>(
    plaintext: &[u8],
    offset: usize,
) -> Result<[u8; LENGTH], PlatformError> {
    plaintext
        .get(offset..offset + LENGTH)
        .ok_or(PlatformError::Tampered)?
        .try_into()
        .map_err(map_slice_error)
}

#[cfg(windows)]
fn token_name(lookup_digest: &[u8; 32]) -> String {
    let mut name = String::from("active-token-");
    name.push_str(&platform::hex(lookup_digest));
    name
}

#[cfg(windows)]
fn token_entropy(registry_id: &str, lookup_digest: &[u8; 32]) -> Vec<u8> {
    let mut entropy = b"ocentra.protected-custody.prepared-token.v2".to_vec();
    entropy.extend_from_slice(registry_id.as_bytes());
    entropy.extend_from_slice(lookup_digest);
    entropy
}

#[cfg(windows)]
fn same_parts(left: &PreparedTokenParts, right: &PreparedTokenParts) -> bool {
    left.record_id == right.record_id
        && left.lookup_digest == right.lookup_digest
        && left.sequence == right.sequence
}

#[cfg(windows)]
fn map_random_error(_error: getrandom::Error) -> PlatformError {
    PlatformError::Unavailable
}

#[cfg(windows)]
fn map_protocol_error(
    _error: ocentra_protected_capability_custody_protocol::types::ProtocolError,
) -> PlatformError {
    PlatformError::Unavailable
}

#[cfg(windows)]
fn map_slice_error(_error: std::array::TryFromSliceError) -> PlatformError {
    PlatformError::Tampered
}
