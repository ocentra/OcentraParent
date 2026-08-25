use crate::platform::identity::DatabaseIdentity;
use crate::platform::record::BrokerRecord;
use crate::platform::request::TransitionRequest;
use crate::platform::{PlatformError, SealContext};

mod cursor;

use cursor::Cursor;

const RECORD_MAGIC: [u8; 4] = *b"OCPB";
const RECORD_FORMAT_VERSION: u16 = 1;
const MAX_NAMESPACE_BYTES: usize = 128;
const MAX_BINDING_BYTES: usize = 16 * 1024;
const MAX_PLAINTEXT_BYTES: usize = 32 * 1024;

pub(super) fn encode_transition(
    request: TransitionRequest<'_>,
    anti_rollback_watermark: u64,
) -> Result<Vec<u8>, PlatformError> {
    if request.record_namespace.is_empty()
        || request.record_namespace.len() > MAX_NAMESPACE_BYTES
        || request.canonical_binding.is_empty()
        || request.canonical_binding.len() > MAX_BINDING_BYTES
    {
        return Err(PlatformError::InvalidAttestation);
    }
    let mut bytes = Vec::with_capacity(512 + request.canonical_binding.len());
    bytes.extend_from_slice(&RECORD_MAGIC);
    bytes.extend_from_slice(&RECORD_FORMAT_VERSION.to_be_bytes());
    append_field(&mut bytes, request.record_namespace)?;
    bytes.extend_from_slice(&request.schema_version.to_be_bytes());
    bytes.extend_from_slice(&request.binding_version.to_be_bytes());
    bytes.extend_from_slice(request.database_identity.as_bytes());
    bytes.extend_from_slice(request.record_id);
    bytes.extend_from_slice(request.lookup_digest);
    bytes.extend_from_slice(request.binding_digest);
    append_field(&mut bytes, request.canonical_binding)?;
    bytes.push(request.state as u8);
    bytes.extend_from_slice(&request.sequence.to_be_bytes());
    bytes.extend_from_slice(&request.key_epoch.to_be_bytes());
    bytes.extend_from_slice(&request.writer_epoch.to_be_bytes());
    bytes.extend_from_slice(&anti_rollback_watermark.to_be_bytes());
    if bytes.len() > MAX_PLAINTEXT_BYTES {
        return Err(PlatformError::InvalidAttestation);
    }
    Ok(bytes)
}

pub(super) fn decode_record(
    plaintext: &[u8],
    sealed: Vec<u8>,
) -> Result<BrokerRecord, PlatformError> {
    if plaintext.is_empty() || plaintext.len() > MAX_PLAINTEXT_BYTES {
        return Err(PlatformError::Tampered);
    }
    let mut cursor = Cursor::new(plaintext);
    if cursor.take_exact(RECORD_MAGIC.len())? != RECORD_MAGIC {
        return Err(PlatformError::Tampered);
    }
    if cursor.take_u16()? != RECORD_FORMAT_VERSION {
        return Err(PlatformError::Tampered);
    }
    let record_namespace = cursor.take_field(MAX_NAMESPACE_BYTES)?;
    let schema_version = cursor.take_u32()?;
    let binding_version = cursor.take_u16()?;
    let database_identity = DatabaseIdentity::from_bytes(cursor.take_exact(128)?)?;
    let record_id = cursor.take_array()?;
    let lookup_digest = cursor.take_array()?;
    let binding_digest = cursor.take_array()?;
    let canonical_binding = cursor.take_field(MAX_BINDING_BYTES)?;
    let state = cursor::decode_state(cursor.take_u8()?)?;
    let sequence = cursor.take_u64()?;
    let key_epoch = cursor.take_u64()?;
    let writer_epoch = cursor.take_u64()?;
    let anti_rollback_watermark = cursor.take_u64()?;
    cursor.finish()?;
    if record_namespace.is_empty()
        || canonical_binding.is_empty()
        || sequence == 0
        || key_epoch == 0
        || writer_epoch == 0
        || anti_rollback_watermark == 0
        || sealed.is_empty()
    {
        return Err(PlatformError::Tampered);
    }
    Ok(BrokerRecord {
        record_namespace,
        schema_version,
        binding_version,
        database_identity,
        record_id,
        lookup_digest,
        binding_digest,
        canonical_binding,
        state,
        sequence,
        key_epoch,
        writer_epoch,
        anti_rollback_watermark,
        sealed,
    })
}

pub(super) fn context_matches(record: &BrokerRecord, context: SealContext<'_>) -> bool {
    record.record_namespace == context.record_namespace
        && record.schema_version == context.schema_version
        && record.binding_version == context.binding_version
        && record.database_identity == context.database_identity
        && record.record_id == *context.record_id
        && record.lookup_digest == *context.lookup_digest
        && record.binding_digest == *context.binding_digest
        && record.canonical_binding == context.canonical_binding
        && record.state == context.state
        && record.sequence == context.sequence
        && record.key_epoch == context.key_epoch
        && record.writer_epoch == context.writer_epoch
        && record.anti_rollback_watermark == context.anti_rollback_watermark
}

fn append_field(bytes: &mut Vec<u8>, value: &[u8]) -> Result<(), PlatformError> {
    let length = u32::try_from(value.len()).map_err(map_length_error)?;
    bytes.extend_from_slice(&length.to_be_bytes());
    bytes.extend_from_slice(value);
    Ok(())
}

fn map_length_error(_error: std::num::TryFromIntError) -> PlatformError {
    PlatformError::InvalidAttestation
}
