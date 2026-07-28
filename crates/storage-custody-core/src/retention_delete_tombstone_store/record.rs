use std::io;

use ocentra_eventing::envelope::StoredEventEnvelope;
use serde::{Deserialize, Serialize};

use crate::storage_custody::StorageCustodyActionPlannedEvent;

use super::{
    RetentionDeleteOutboxPayload, RetentionDeleteOutboxRecord, TypedTombstoneOutboxPayload,
};

const TYPED_STORE_VERSION: u16 = 2;

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct LegacyVersionOneRecord {
    version: u16,
    deletion_ref: String,
    proof_ref: String,
    terminal_pending: bool,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct TypedVersionTwoRecord {
    version: u16,
    deletion_ref: String,
    proof_ref: String,
    action: StorageCustodyActionPlannedEvent,
    envelope: StoredEventEnvelope,
    terminal_pending: bool,
}

pub(super) fn typed(
    deletion_ref: String,
    proof_ref: String,
    action: StorageCustodyActionPlannedEvent,
    envelope: StoredEventEnvelope,
) -> RetentionDeleteOutboxRecord {
    RetentionDeleteOutboxRecord {
        version: TYPED_STORE_VERSION,
        deletion_ref,
        proof_ref,
        terminal_pending: true,
        payload: RetentionDeleteOutboxPayload::Typed(Box::new(TypedTombstoneOutboxPayload {
            action,
            envelope,
        })),
    }
}

pub(super) fn decode(
    value: serde_json::Value,
) -> Result<RetentionDeleteOutboxRecord, serde_json::Error> {
    match record_version(&value)? {
        1 => decode_legacy(value),
        TYPED_STORE_VERSION => decode_typed(value),
        _ => Err(unsupported_version_error()),
    }
}

pub(super) fn encode(
    record: &RetentionDeleteOutboxRecord,
) -> Result<serde_json::Value, serde_json::Error> {
    match &record.payload {
        RetentionDeleteOutboxPayload::LegacyVersionOne => encode_legacy(record),
        RetentionDeleteOutboxPayload::Typed(payload) => {
            encode_typed(record, &payload.action, &payload.envelope)
        }
    }
}

fn record_version(value: &serde_json::Value) -> Result<u16, serde_json::Error> {
    value
        .get("version")
        .and_then(serde_json::Value::as_u64)
        .and_then(|version| u16::try_from(version).ok())
        .ok_or_else(missing_version_error)
}

fn decode_legacy(
    value: serde_json::Value,
) -> Result<RetentionDeleteOutboxRecord, serde_json::Error> {
    let legacy: LegacyVersionOneRecord = serde_json::from_value(value)?;
    Ok(RetentionDeleteOutboxRecord {
        version: legacy.version,
        deletion_ref: legacy.deletion_ref,
        proof_ref: legacy.proof_ref,
        terminal_pending: legacy.terminal_pending,
        payload: RetentionDeleteOutboxPayload::LegacyVersionOne,
    })
}

fn decode_typed(
    value: serde_json::Value,
) -> Result<RetentionDeleteOutboxRecord, serde_json::Error> {
    let typed: TypedVersionTwoRecord = serde_json::from_value(value)?;
    Ok(RetentionDeleteOutboxRecord {
        version: typed.version,
        deletion_ref: typed.deletion_ref,
        proof_ref: typed.proof_ref,
        terminal_pending: typed.terminal_pending,
        payload: RetentionDeleteOutboxPayload::Typed(Box::new(TypedTombstoneOutboxPayload {
            action: typed.action,
            envelope: typed.envelope,
        })),
    })
}

fn encode_legacy(
    record: &RetentionDeleteOutboxRecord,
) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::to_value(LegacyVersionOneRecord {
        version: record.version,
        deletion_ref: record.deletion_ref.clone(),
        proof_ref: record.proof_ref.clone(),
        terminal_pending: record.terminal_pending,
    })
}

fn encode_typed(
    record: &RetentionDeleteOutboxRecord,
    action: &StorageCustodyActionPlannedEvent,
    envelope: &StoredEventEnvelope,
) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::to_value(TypedVersionTwoRecord {
        version: record.version,
        deletion_ref: record.deletion_ref.clone(),
        proof_ref: record.proof_ref.clone(),
        action: action.clone(),
        envelope: envelope.clone(),
        terminal_pending: record.terminal_pending,
    })
}

fn missing_version_error() -> serde_json::Error {
    serde_json::Error::io(io::Error::new(
        io::ErrorKind::InvalidData,
        "tombstone outbox record is missing a numeric version",
    ))
}

fn unsupported_version_error() -> serde_json::Error {
    serde_json::Error::io(io::Error::new(
        io::ErrorKind::InvalidData,
        "tombstone outbox record version is unsupported",
    ))
}
