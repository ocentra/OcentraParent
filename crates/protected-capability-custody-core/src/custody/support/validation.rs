use super::super::CustodyError;
use super::to_u64;
use crate::binding::Binding;
use crate::platform::PlatformAttestation;
use crate::storage::Record;

pub(super) fn validate_binding(
    record: &Record,
    binding: &Binding,
    expected_sequence: u64,
) -> Result<(), CustodyError> {
    if record.canonical_binding != binding.canonical_bytes()
        || record.binding_digest.as_slice() != binding.digest().as_slice()
    {
        return Err(CustodyError::WrongBinding);
    }
    if to_u64(record.sequence)? != expected_sequence {
        return Err(CustodyError::Conflict);
    }
    Ok(())
}

pub(super) fn ensure_current_epoch(
    record: &Record,
    attestation: PlatformAttestation,
) -> Result<(), CustodyError> {
    let record_key = to_u64(record.key_epoch)?;
    let record_writer = to_u64(record.writer_epoch)?;
    let record_watermark = to_u64(record.anti_rollback_watermark)?;
    if attestation.key_epoch != record_key {
        return Err(CustodyError::Rotated);
    }
    if attestation.writer_epoch < record_writer
        || attestation.anti_rollback_watermark < record_watermark
    {
        return Err(CustodyError::Tampered);
    }
    Ok(())
}
