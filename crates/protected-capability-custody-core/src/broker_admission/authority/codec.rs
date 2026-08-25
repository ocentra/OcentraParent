use crate::authority::AuthorityError;
use ocentra_protected_capability_custody_protocol::request::ExpectedGenerations;

const AUTHORITY_MAGIC: [u8; 4] = *b"OCPA";
const AUTHORITY_VERSION: u16 = 1;
const AUTHORITY_PLAINTEXT_BYTES: usize = 38;

pub(super) fn decode(plaintext: &[u8]) -> Result<ExpectedGenerations, AuthorityError> {
    if plaintext.len() != AUTHORITY_PLAINTEXT_BYTES
        || plaintext.get(..4) != Some(AUTHORITY_MAGIC.as_slice())
        || plaintext.get(4..6) != Some(AUTHORITY_VERSION.to_be_bytes().as_slice())
    {
        return Err(AuthorityError::Rejected);
    }
    ExpectedGenerations::try_new(
        read_u64(plaintext, 6)?,
        read_u64(plaintext, 14)?,
        read_u64(plaintext, 22)?,
        read_u64(plaintext, 30)?,
    )
    .map_err(map_protocol_error)
}

fn read_u64(bytes: &[u8], offset: usize) -> Result<u64, AuthorityError> {
    bytes
        .get(offset..offset + 8)
        .ok_or(AuthorityError::Rejected)?
        .try_into()
        .map(u64::from_be_bytes)
        .map_err(map_slice_error)
}

fn map_protocol_error(
    _error: ocentra_protected_capability_custody_protocol::types::ProtocolError,
) -> AuthorityError {
    AuthorityError::Rejected
}

fn map_slice_error(_error: std::array::TryFromSliceError) -> AuthorityError {
    AuthorityError::Rejected
}
