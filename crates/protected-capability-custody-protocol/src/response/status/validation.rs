use crate::constants::OPAQUE_TOKEN_BYTES;
use crate::types::ProtocolError;

use super::super::ResponseStatus;

pub(crate) fn validate_result(
    status: ResponseStatus,
    client_process_epoch: u64,
    broker_epoch: u64,
    broker_key_epoch: u64,
    writer_lease_epoch: u64,
    authority_generation: u64,
    target_generation: u64,
    key_generation: u64,
    writer_generation: u64,
    opaque_token: &[u8],
) -> Result<(), ProtocolError> {
    if client_process_epoch == 0
        || broker_epoch == 0
        || broker_key_epoch == 0
        || writer_lease_epoch == 0
        || authority_generation == 0
        || target_generation == 0
        || key_generation == 0
        || writer_generation == 0
    {
        return Err(ProtocolError::InvalidEpoch);
    }
    if matches!(status, ResponseStatus::Prepared) && opaque_token.len() != OPAQUE_TOKEN_BYTES {
        return Err(ProtocolError::InvalidOpaqueToken);
    }
    if !matches!(status, ResponseStatus::Prepared) && !opaque_token.is_empty() {
        return Err(ProtocolError::UnexpectedOpaqueToken);
    }
    Ok(())
}
