#![forbid(unsafe_code)]

//! Bounded row transfer for the one-way bridge audit schema migration.

use ocentra_schema::account_identity_authority::AccountIdentityProviderSubject;
use rusqlite::{params, Transaction};

use super::super::parent_local_bridge_repository::audit::provider_subject_digest;

const MIGRATION_BATCH_ROWS: i64 = 256;

struct V1AuditRow {
    sequence: i64,
    event_id: String,
    account_id: String,
    provider: String,
    provider_subject: String,
    household_id: String,
    member_id: String,
    device_id: String,
    authority_session_id: String,
    audience: String,
    bridge_revoke_epoch: i64,
    action: String,
    occurred_at: i64,
    retain_until: i64,
    delivery_state: String,
    attempt_id: Option<String>,
    claimed_at: Option<i64>,
    delivered_at: Option<i64>,
}

pub(super) fn migrate_audit_rows(
    transaction: &Transaction<'_>,
    delivery_lease_millis: i64,
) -> Result<(), ()> {
    let mut after_sequence = i64::MIN;
    loop {
        let rows = read_batch(transaction, after_sequence)?;
        let Some(last_sequence) = rows.last().map(|row| row.sequence) else {
            return Ok(());
        };
        for row in rows {
            insert_row(transaction, row, delivery_lease_millis)?;
        }
        after_sequence = last_sequence;
    }
}

fn read_batch(transaction: &Transaction<'_>, after_sequence: i64) -> Result<Vec<V1AuditRow>, ()> {
    let mut statement = transaction
        .prepare(
            "SELECT sequence, event_id, account_id, provider, provider_subject,
                    household_id, member_id, device_id, authority_session_id,
                    audience, bridge_revoke_epoch, action,
                    occurred_at_epoch_millis, retain_until_epoch_millis,
                    delivery_state, delivery_attempt_id,
                    delivery_claimed_at_epoch_millis, delivered_at_epoch_millis
               FROM account_identity_parent_local_bridge_audit_copy
              WHERE sequence > ?1
              ORDER BY sequence LIMIT ?2",
        )
        .map_err(|_| ())?;
    let rows = statement
        .query_map(params![after_sequence, MIGRATION_BATCH_ROWS], |row| {
            Ok(V1AuditRow {
                sequence: row.get(0)?,
                event_id: row.get(1)?,
                account_id: row.get(2)?,
                provider: row.get(3)?,
                provider_subject: row.get(4)?,
                household_id: row.get(5)?,
                member_id: row.get(6)?,
                device_id: row.get(7)?,
                authority_session_id: row.get(8)?,
                audience: row.get(9)?,
                bridge_revoke_epoch: row.get(10)?,
                action: row.get(11)?,
                occurred_at: row.get(12)?,
                retain_until: row.get(13)?,
                delivery_state: row.get(14)?,
                attempt_id: row.get(15)?,
                claimed_at: row.get(16)?,
                delivered_at: row.get(17)?,
            })
        })
        .map_err(|_| ())?;
    rows.collect::<Result<Vec<_>, _>>().map_err(|_| ())
}

fn insert_row(
    transaction: &Transaction<'_>,
    row: V1AuditRow,
    delivery_lease_millis: i64,
) -> Result<(), ()> {
    let provider_subject = AccountIdentityProviderSubject::parse(row.provider_subject).ok_or(())?;
    let attempt_count = i64::from(row.delivery_state != "pending");
    let lease_expires = match (row.delivery_state.as_str(), row.claimed_at) {
        ("in-flight", Some(claimed)) => Some(claimed.checked_add(delivery_lease_millis).ok_or(())?),
        ("pending" | "delivered", None) => None,
        _ => return Err(()),
    };
    let next_delivery = row
        .delivered_at
        .or(row.claimed_at)
        .unwrap_or(row.occurred_at);
    transaction
        .execute(
            "INSERT INTO account_identity_parent_local_bridge_audit_outbox (
                 sequence, event_id, account_id, provider,
                 provider_subject_digest, household_id, member_id, device_id,
                 authority_session_id, audience, bridge_revoke_epoch, action,
                 occurred_at_epoch_millis, retain_until_epoch_millis,
                 delivery_state, delivery_attempt_id, delivery_attempt_count,
                 delivery_claimed_at_epoch_millis,
                 delivery_lease_expires_at_epoch_millis,
                 next_delivery_at_epoch_millis, delivered_at_epoch_millis
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11,
                       ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21)",
            params![
                row.sequence,
                row.event_id,
                row.account_id,
                row.provider,
                provider_subject_digest(&provider_subject),
                row.household_id,
                row.member_id,
                row.device_id,
                row.authority_session_id,
                row.audience,
                row.bridge_revoke_epoch,
                row.action,
                row.occurred_at,
                row.retain_until,
                row.delivery_state,
                row.attempt_id,
                attempt_count,
                row.claimed_at,
                lease_expires,
                next_delivery,
                row.delivered_at,
            ],
        )
        .map(|_| ())
        .map_err(|_| ())
}
