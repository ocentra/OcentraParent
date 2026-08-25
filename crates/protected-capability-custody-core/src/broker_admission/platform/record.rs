#[cfg(windows)]
use crate::platform::record::BrokerRecord;
#[cfg(windows)]
use crate::platform::request::BrokerLookup;
#[cfg(windows)]
use crate::platform::{PlatformError, SealContext};
#[cfg(windows)]
use zeroize::Zeroizing;

#[cfg(windows)]
use super::guard::{map_poison, BrokerPlatformGuard};
#[cfg(windows)]
use super::{crypto, registry, state};
#[cfg(windows)]
use crate::broker_admission::record_codec;

#[cfg(windows)]
const MAX_SEALED_BYTES: usize = 64 * 1024;
#[cfg(windows)]
const RECORD_NAME_PREFIX: &str = "record-";

#[cfg(windows)]
pub(super) fn current(
    guard: &BrokerPlatformGuard,
    lookup: BrokerLookup<'_>,
) -> Result<Option<BrokerRecord>, PlatformError> {
    guard.revalidate_live()?;
    validate_lookup(guard, lookup)?;
    let Some(sealed) = read_ciphertext(&guard.registry_id, lookup.lookup_digest)? else {
        return Ok(None);
    };
    let plaintext = Zeroizing::new(crypto::unprotect_record(
        &guard.registry_id,
        lookup.lookup_digest,
        lookup.database_identity,
        &sealed,
    )?);
    let record = record_codec::decode_record(plaintext.as_ref(), sealed)?;
    validate_record(&record, lookup)?;
    let mut ledger = guard.state.lock().map_err(map_poison)?;
    if record.anti_rollback_watermark > ledger.watermark {
        let repaired = state::LedgerState {
            watermark: record.anti_rollback_watermark,
            ..*ledger
        };
        state::write(&guard.registry_id, repaired)?;
        *ledger = repaired;
    }
    Ok(Some(record))
}

#[cfg(windows)]
pub(super) fn open_and_verify(
    guard: &BrokerPlatformGuard,
    context: SealContext<'_>,
    sealed: &[u8],
) -> Result<(), PlatformError> {
    guard.revalidate_live()?;
    if sealed.is_empty() || sealed.len() > MAX_SEALED_BYTES {
        return Err(PlatformError::Tampered);
    }
    let plaintext = Zeroizing::new(crypto::unprotect_record(
        &guard.registry_id,
        context.lookup_digest,
        context.database_identity,
        sealed,
    )?);
    let record = record_codec::decode_record(plaintext.as_ref(), sealed.to_vec())?;
    if record_codec::context_matches(&record, context) {
        Ok(())
    } else {
        Err(PlatformError::Tampered)
    }
}

#[cfg(windows)]
pub(super) fn read_ciphertext(
    registry_id: &str,
    lookup_digest: &[u8; 32],
) -> Result<Option<Vec<u8>>, PlatformError> {
    registry::read(registry_id, &record_name(lookup_digest))
}

#[cfg(windows)]
pub(super) fn validate_ciphertext(sealed: &[u8]) -> Result<(), PlatformError> {
    if sealed.is_empty() || sealed.len() > MAX_SEALED_BYTES {
        return Err(PlatformError::InvalidAttestation);
    }
    Ok(())
}

#[cfg(windows)]
pub(super) fn same(left: &BrokerRecord, right: &BrokerRecord) -> bool {
    left.record_namespace == right.record_namespace
        && left.schema_version == right.schema_version
        && left.binding_version == right.binding_version
        && left.database_identity == right.database_identity
        && left.record_id == right.record_id
        && left.lookup_digest == right.lookup_digest
        && left.binding_digest == right.binding_digest
        && left.canonical_binding == right.canonical_binding
        && left.state == right.state
        && left.sequence == right.sequence
        && left.key_epoch == right.key_epoch
        && left.writer_epoch == right.writer_epoch
        && left.anti_rollback_watermark == right.anti_rollback_watermark
        && left.sealed == right.sealed
}

#[cfg(windows)]
fn validate_lookup(
    guard: &BrokerPlatformGuard,
    lookup: BrokerLookup<'_>,
) -> Result<(), PlatformError> {
    let identity = guard
        .database_identity
        .lock()
        .map_err(map_poison)?
        .ok_or(PlatformError::InvalidAttestation)?;
    if lookup.database_identity != identity
        || lookup.record_namespace != crate::RECORD_NAMESPACE
        || lookup.schema_version != crate::STORAGE_SCHEMA_VERSION
        || lookup.binding_version != crate::binding::BINDING_VERSION
    {
        return Err(PlatformError::InvalidAttestation);
    }
    Ok(())
}

#[cfg(windows)]
fn validate_record(record: &BrokerRecord, lookup: BrokerLookup<'_>) -> Result<(), PlatformError> {
    if record.record_namespace != lookup.record_namespace
        || record.schema_version != lookup.schema_version
        || record.binding_version != lookup.binding_version
        || record.database_identity != lookup.database_identity
        || record.lookup_digest != *lookup.lookup_digest
    {
        return Err(PlatformError::Tampered);
    }
    Ok(())
}

#[cfg(windows)]
pub(super) fn record_name(lookup_digest: &[u8; 32]) -> String {
    let mut name = String::from(RECORD_NAME_PREFIX);
    name.push_str(&registry::hex(lookup_digest));
    name
}
