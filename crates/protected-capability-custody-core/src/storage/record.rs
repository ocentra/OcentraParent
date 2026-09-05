use rusqlite::Row;
use sha2::{Digest, Sha256};

use super::{Record, StorageError};
use crate::binding::{Binding, BINDING_VERSION};
use crate::platform::{identity::DatabaseIdentity, record::BrokerRecord, SealedState};

mod transition;

const MAX_CANONICAL_BYTES: usize = 16 * 1024;
const MAX_SEALED_BYTES: usize = 64 * 1024;

pub(super) struct RawRecord {
    record_id: Vec<u8>,
    lookup_digest: Vec<u8>,
    binding_digest: Vec<u8>,
    canonical_binding: Vec<u8>,
    state: i64,
    sequence: i64,
    key_epoch: i64,
    writer_epoch: i64,
    anti_rollback_watermark: i64,
    sealed: Vec<u8>,
    schema_version: i64,
    binding_version: i64,
    database_identity: Vec<u8>,
    cas_digest: Vec<u8>,
}

pub(super) fn read_raw(row: &Row<'_>) -> rusqlite::Result<RawRecord> {
    Ok(RawRecord {
        record_id: row.get(0)?,
        lookup_digest: row.get(1)?,
        binding_digest: row.get(2)?,
        canonical_binding: row.get(3)?,
        state: row.get(4)?,
        sequence: row.get(5)?,
        key_epoch: row.get(6)?,
        writer_epoch: row.get(7)?,
        anti_rollback_watermark: row.get(8)?,
        sealed: row.get(9)?,
        schema_version: row.get(10)?,
        binding_version: row.get(11)?,
        database_identity: row.get(12)?,
        cas_digest: row.get(13)?,
    })
}

pub(super) fn from_raw(raw: RawRecord) -> Result<Record, StorageError> {
    if raw.canonical_binding.len() > MAX_CANONICAL_BYTES
        || raw.sealed.len() > MAX_SEALED_BYTES
        || raw.database_identity.len() > 128
    {
        return Err(StorageError::Tampered);
    }
    let value = Record {
        record_id: array(raw.record_id)?,
        lookup_digest: array(raw.lookup_digest)?,
        binding_digest: array(raw.binding_digest)?,
        canonical_binding: raw.canonical_binding,
        state: decode_state(raw.state)?,
        sequence: positive(raw.sequence)?,
        key_epoch: positive(raw.key_epoch)?,
        writer_epoch: positive(raw.writer_epoch)?,
        anti_rollback_watermark: positive(raw.anti_rollback_watermark)?,
        sealed: raw.sealed,
        schema_version: u32::try_from(raw.schema_version)
            .map_err(|_schema_version_error| StorageError::Tampered)?,
        binding_version: u16::try_from(raw.binding_version)
            .map_err(|_binding_version_error| StorageError::Tampered)?,
        database_identity: DatabaseIdentity::from_bytes(&raw.database_identity)
            .map_err(|_database_identity_error| StorageError::Tampered)?,
        cas_digest: array(raw.cas_digest)?,
    };
    validate(&value)?;
    Ok(value)
}

pub(super) fn from_broker(broker: &BrokerRecord) -> Result<Record, StorageError> {
    if broker.record_namespace != crate::RECORD_NAMESPACE {
        return Err(StorageError::Tampered);
    }
    let mut value = Record {
        record_id: broker.record_id,
        lookup_digest: broker.lookup_digest,
        binding_digest: broker.binding_digest,
        canonical_binding: broker.canonical_binding.clone(),
        state: broker.state,
        sequence: broker.sequence,
        key_epoch: broker.key_epoch,
        writer_epoch: broker.writer_epoch,
        anti_rollback_watermark: broker.anti_rollback_watermark,
        sealed: broker.sealed.clone(),
        schema_version: broker.schema_version,
        binding_version: broker.binding_version,
        database_identity: broker.database_identity,
        cas_digest: [0_u8; 32],
    };
    value.cas_digest = cas_digest(&value);
    validate(&value)?;
    Ok(value)
}

pub(super) fn to_broker(value: &Record) -> BrokerRecord {
    BrokerRecord {
        record_namespace: crate::RECORD_NAMESPACE.to_vec(),
        schema_version: value.schema_version,
        binding_version: value.binding_version,
        database_identity: value.database_identity,
        record_id: value.record_id,
        lookup_digest: value.lookup_digest,
        binding_digest: value.binding_digest,
        canonical_binding: value.canonical_binding.clone(),
        state: value.state,
        sequence: value.sequence,
        key_epoch: value.key_epoch,
        writer_epoch: value.writer_epoch,
        anti_rollback_watermark: value.anti_rollback_watermark,
        sealed: value.sealed.clone(),
    }
}

pub(super) fn validate(value: &Record) -> Result<(), StorageError> {
    if value.canonical_binding.is_empty()
        || value.canonical_binding.len() > MAX_CANONICAL_BYTES
        || value.sealed.is_empty()
        || value.sealed.len() > MAX_SEALED_BYTES
        || value.record_id == [0_u8; 32]
        || value.lookup_digest == [0_u8; 32]
        || value.binding_digest == [0_u8; 32]
        || value.schema_version != crate::STORAGE_SCHEMA_VERSION
        || value.binding_version != BINDING_VERSION
        || value.sequence > i64::MAX as u64
        || value.key_epoch > i64::MAX as u64
        || value.writer_epoch > i64::MAX as u64
        || value.anti_rollback_watermark > i64::MAX as u64
    {
        return Err(StorageError::Tampered);
    }
    transition::validate_state_sequence(value.state, value.sequence)?;
    let binding = Binding::decode(&value.canonical_binding)
        .map_err(|_binding_decode_error| StorageError::Tampered)?;
    if binding.digest() != value.binding_digest
        || binding.locator().lookup_digest() != value.lookup_digest
        || cas_digest(value) != value.cas_digest
    {
        return Err(StorageError::Tampered);
    }
    Ok(())
}

pub(super) fn validate_transition(prior: &Record, next: &Record) -> Result<(), StorageError> {
    transition::validate(prior, next)
}

fn cas_digest(value: &Record) -> [u8; 32] {
    let mut hasher = Sha256::new();
    frame(&mut hasher, b"ocentra.custody-row-cas.v2");
    frame(&mut hasher, crate::RECORD_NAMESPACE);
    frame(&mut hasher, &value.schema_version.to_be_bytes());
    frame(&mut hasher, &value.binding_version.to_be_bytes());
    frame(&mut hasher, value.database_identity.as_bytes());
    frame(&mut hasher, &value.record_id);
    frame(&mut hasher, &value.lookup_digest);
    frame(&mut hasher, &value.binding_digest);
    frame(&mut hasher, &value.canonical_binding);
    frame(&mut hasher, &[value.state as u8]);
    frame(&mut hasher, &value.sequence.to_be_bytes());
    frame(&mut hasher, &value.key_epoch.to_be_bytes());
    frame(&mut hasher, &value.writer_epoch.to_be_bytes());
    frame(&mut hasher, &value.anti_rollback_watermark.to_be_bytes());
    frame(&mut hasher, &value.sealed);
    hasher.finalize().into()
}

fn frame(hasher: &mut Sha256, value: &[u8]) {
    hasher.update((value.len() as u64).to_be_bytes());
    hasher.update(value);
}

fn array(value: Vec<u8>) -> Result<[u8; 32], StorageError> {
    value
        .try_into()
        .map_err(|_array_length_error| StorageError::Tampered)
}

fn positive(value: i64) -> Result<u64, StorageError> {
    let converted =
        u64::try_from(value).map_err(|_integer_conversion_error| StorageError::Tampered)?;
    if converted == 0 {
        return Err(StorageError::Tampered);
    }
    Ok(converted)
}

fn decode_state(value: i64) -> Result<SealedState, StorageError> {
    match value {
        1 => Ok(SealedState::Prepared),
        2 => Ok(SealedState::CommitAmbiguous),
        3 => Ok(SealedState::AbortAmbiguous),
        4 => Ok(SealedState::Committed),
        5 => Ok(SealedState::Aborted),
        _ => Err(StorageError::Tampered),
    }
}
