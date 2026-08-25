use chrono::{DateTime, SecondsFormat, Utc};
use ocentra_lan_core::lan_pairing::signed_household_mesh_ingress::{
    replay_identity::LanHouseholdMeshDurableReplayIdentity,
    LanCryptographicallyVerifiedHouseholdMeshIngress,
};
use ocentra_parent_agent_protocol::lan_pairing::LanSignedChildAgentMessageKind;
use rusqlite::{params, Connection, OptionalExtension, TransactionBehavior};
use sha2::{Digest, Sha256};

use super::{
    LanHouseholdMeshIngressCustodyError, LanHouseholdMeshIngressReceiptStore,
    LanHouseholdMeshIngressRejectionIdentity, LanHouseholdMeshIngressRejectionOutcome,
    LanHouseholdMeshIngressRejectionReason,
};

pub(super) const TABLE: &str = "lan_household_mesh_ingress_rejections_v1";
pub(super) const CREATE_TABLE: &str = "CREATE TABLE lan_household_mesh_ingress_rejections_v1 (outcome_id TEXT PRIMARY KEY NOT NULL, replay_identity_sha256 TEXT NOT NULL, reason TEXT NOT NULL CHECK(reason IN ('authority-stale','expired','identity-mismatch','invalid-input','duplicate-receipt','reconciliation-required','sequence-regression')), observed_at TEXT NOT NULL) STRICT";
const IDENTITY_DOMAIN: &[u8] = b"ocentra.lan.household-mesh.rejection-identity.v1\0";
const OUTCOME_DOMAIN: &[u8] = b"ocentra.lan.household-mesh.rejection-outcome.v1\0";

struct StoredRejection {
    outcome_id: String,
    replay_identity_sha256: String,
    reason: LanHouseholdMeshIngressRejectionReason,
    observed_at: String,
}

pub(super) fn create_schema(
    connection: &Connection,
) -> Result<(), LanHouseholdMeshIngressCustodyError> {
    connection
        .execute(CREATE_TABLE, [])
        .map(|_changed| ())
        .map_err(storage_error)
}

pub(super) fn validate_schema(
    connection: &Connection,
) -> Result<(), LanHouseholdMeshIngressCustodyError> {
    let table_sql = connection
        .query_row(
            "SELECT sql FROM sqlite_master WHERE type='table' AND name=?1",
            [TABLE],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(storage_error)?
        .ok_or(LanHouseholdMeshIngressCustodyError::SchemaRejected)?;
    if normalize_sql(&table_sql) != normalize_sql(CREATE_TABLE) {
        return Err(LanHouseholdMeshIngressCustodyError::SchemaRejected);
    }
    let mut statement = connection
        .prepare("SELECT name, type, notnull, pk FROM pragma_table_info(?1) ORDER BY cid")
        .map_err(storage_error)?;
    let actual = statement
        .query_map([TABLE], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, i64>(2)?,
                row.get::<_, i64>(3)?,
            ))
        })
        .map_err(storage_error)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(storage_error)?;
    let expected = [
        ("outcome_id", "TEXT", 1),
        ("replay_identity_sha256", "TEXT", 0),
        ("reason", "TEXT", 0),
        ("observed_at", "TEXT", 0),
    ];
    if actual.len() != expected.len()
        || actual.iter().zip(expected).any(
            |((name, kind, notnull, pk), (expected_name, expected_kind, expected_pk))| {
                name != expected_name
                    || kind != expected_kind
                    || *notnull != 1
                    || *pk != expected_pk
            },
        )
    {
        return Err(LanHouseholdMeshIngressCustodyError::SchemaRejected);
    }
    Ok(())
}

pub(super) fn validate_integrity(
    connection: &Connection,
) -> Result<(), LanHouseholdMeshIngressCustodyError> {
    let mut statement = connection
        .prepare(
            "SELECT outcome_id,replay_identity_sha256,reason,observed_at FROM lan_household_mesh_ingress_rejections_v1",
        )
        .map_err(storage_error)?;
    let rows = statement
        .query_map([], stored_rejection_from_row)
        .map_err(storage_error)?;
    for row in rows {
        let stored =
            row.map_err(|_error| LanHouseholdMeshIngressCustodyError::IntegrityRejected)?;
        if !is_lower_hex(&stored.replay_identity_sha256, 64)
            || DateTime::parse_from_rfc3339(&stored.observed_at).is_err()
            || stored.outcome_id != outcome_id_for(&stored.replay_identity_sha256, stored.reason)
        {
            return Err(LanHouseholdMeshIngressCustodyError::IntegrityRejected);
        }
    }
    Ok(())
}

pub(super) fn record_rejected_ingress(
    store: &mut LanHouseholdMeshIngressReceiptStore,
    ingress: &LanCryptographicallyVerifiedHouseholdMeshIngress,
    reason: LanHouseholdMeshIngressRejectionReason,
    observed_at: &str,
) -> Result<LanHouseholdMeshIngressRejectionOutcome, LanHouseholdMeshIngressCustodyError> {
    let observed_at = DateTime::parse_from_rfc3339(observed_at)
        .map_err(|_error| LanHouseholdMeshIngressCustodyError::InvalidInput)?
        .with_timezone(&Utc)
        .to_rfc3339_opts(SecondsFormat::Millis, true);
    let replay_identity_sha256 = identity_sha256(&ingress.durable_replay_identity());
    let outcome_id = outcome_id_for(&replay_identity_sha256, reason);
    let transaction = store
        .connection
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(storage_error)?;
    if let Some(existing) = load_rejection(&transaction, &outcome_id)? {
        if existing.replay_identity_sha256 != replay_identity_sha256 || existing.reason != reason {
            return Err(LanHouseholdMeshIngressCustodyError::IntegrityRejected);
        }
        transaction.commit().map_err(storage_error)?;
        return Ok(outcome_from_stored(existing));
    }
    transaction
        .execute(
            "INSERT INTO lan_household_mesh_ingress_rejections_v1 (outcome_id,replay_identity_sha256,reason,observed_at) VALUES (?1,?2,?3,?4)",
            params![
                outcome_id.as_str(),
                replay_identity_sha256.as_str(),
                reason.as_str(),
                observed_at.as_str()
            ],
        )
        .map_err(storage_error)?;
    transaction.commit().map_err(storage_error)?;
    let stored = load_rejection(&store.connection, &outcome_id)?
        .ok_or(LanHouseholdMeshIngressCustodyError::IntegrityRejected)?;
    if stored.replay_identity_sha256 != replay_identity_sha256
        || stored.reason != reason
        || stored.observed_at != observed_at
    {
        return Err(LanHouseholdMeshIngressCustodyError::IntegrityRejected);
    }
    Ok(outcome_from_stored(stored))
}

fn load_rejection(
    connection: &Connection,
    outcome_id: &str,
) -> Result<Option<StoredRejection>, LanHouseholdMeshIngressCustodyError> {
    connection
        .query_row(
            "SELECT outcome_id,replay_identity_sha256,reason,observed_at FROM lan_household_mesh_ingress_rejections_v1 WHERE outcome_id=?1",
            [outcome_id],
            stored_rejection_from_row,
        )
        .optional()
        .map_err(storage_error)
}

fn stored_rejection_from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<StoredRejection> {
    let reason = row.get::<_, String>(2)?;
    let reason = LanHouseholdMeshIngressRejectionReason::from_stored(&reason).ok_or_else(|| {
        rusqlite::Error::InvalidColumnType(2, "reason".to_string(), rusqlite::types::Type::Text)
    })?;
    Ok(StoredRejection {
        outcome_id: row.get(0)?,
        replay_identity_sha256: row.get(1)?,
        reason,
        observed_at: row.get(3)?,
    })
}

fn outcome_from_stored(stored: StoredRejection) -> LanHouseholdMeshIngressRejectionOutcome {
    LanHouseholdMeshIngressRejectionOutcome {
        outcome_id: stored.outcome_id,
        identity: LanHouseholdMeshIngressRejectionIdentity {
            sha256: stored.replay_identity_sha256,
        },
        reason: stored.reason,
        observed_at: stored.observed_at,
    }
}

fn identity_sha256(identity: &LanHouseholdMeshDurableReplayIdentity<'_>) -> String {
    let mut digest = Sha256::new();
    digest.update(IDENTITY_DOMAIN);
    for value in [
        identity.family_hash(),
        identity.child_device_id(),
        identity.target_device_id(),
        identity.parent_device_id(),
        identity.signer_public_key_id(),
        identity.signer_public_key_sha256(),
        identity.local_event_ref(),
        identity.lan_message_type().as_str(),
        identity.message_id(),
        identity.idempotency_key(),
        identity.route_id(),
        identity.nonce(),
        identity.canonical_payload_sha256(),
        identity.install_id(),
        identity.pairing_id(),
        identity.registry_proof_digest(),
    ] {
        digest.update(value.as_bytes());
        digest.update([0]);
    }
    digest.update(message_kind_value(&identity.message_kind()).as_bytes());
    digest.update([0]);
    digest.update(identity.sequence().value().to_be_bytes());
    format!("{:x}", digest.finalize())
}

fn message_kind_value(kind: &LanSignedChildAgentMessageKind) -> &'static str {
    match kind {
        LanSignedChildAgentMessageKind::Hello => "hello",
        LanSignedChildAgentMessageKind::Heartbeat => "heartbeat",
    }
}

fn outcome_id_for(
    replay_identity_sha256: &str,
    reason: LanHouseholdMeshIngressRejectionReason,
) -> String {
    let mut digest = Sha256::new();
    digest.update(OUTCOME_DOMAIN);
    digest.update(replay_identity_sha256.as_bytes());
    digest.update([0]);
    digest.update(reason.as_str().as_bytes());
    format!("lan-reject-{:x}", digest.finalize())
}

fn is_lower_hex(value: &str, length: usize) -> bool {
    value.len() == length
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn normalize_sql(sql: &str) -> String {
    sql.chars()
        .filter(|character| !character.is_whitespace())
        .flat_map(char::to_lowercase)
        .collect()
}

fn storage_error(_error: rusqlite::Error) -> LanHouseholdMeshIngressCustodyError {
    LanHouseholdMeshIngressCustodyError::StorageUnavailable
}
