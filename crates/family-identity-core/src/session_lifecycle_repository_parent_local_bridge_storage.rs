#![forbid(unsafe_code)]

//! Durable row encoding and fail-closed decoding for parent-local bridges.

use ocentra_schema::account_identity_authority::{
    AccountIdentityDeviceId, AccountIdentityMemberId, AccountIdentityProviderSubject,
    AccountIdentitySessionId,
};
use ocentra_schema::account_identity_parent_local_bridge::AccountIdentityParentLocalBridgeAudience;
use ocentra_schema::report_query_custody::{FamilyId, ParentAccountId};
use rusqlite::{OptionalExtension, Transaction};

use crate::session_lifecycle_custody::record::SessionAuthorityBinding;

use super::super::{labels, SessionLifecycleRepositoryError};
use super::{
    ParentLocalBridgeState, StoredParentLocalBridgeSession, ACTIVE_STATE, CONSUMED_STATE,
    DIGEST_ALGORITHM, REVOKED_STATE,
};
use crate::session_lifecycle_custody::parent_local_bridge::CAPABILITY_DIGEST_DOMAIN;

struct StoredParentLocalBridgeRow {
    capability_digest: String,
    digest_algorithm: String,
    capability_digest_domain: String,
    audience: String,
    connection_nonce_digest: String,
    account_id: String,
    provider: String,
    provider_subject: String,
    household_id: String,
    member_id: String,
    device_id: String,
    authority_session_id: String,
    authority_session_generation: i64,
    authority_generation: i64,
    authority_expires_at_epoch_millis: i64,
    issued_at_epoch_millis: i64,
    expires_at_epoch_millis: i64,
    bridge_revoke_epoch: i64,
    state: String,
    last_transition_at_epoch_millis: i64,
}

pub(super) fn read_record(
    transaction: &Transaction<'_>,
    capability_digest: &str,
    now_epoch_millis: i64,
    clock_skew_millis: i64,
    freshness_ttl_millis: i64,
) -> Result<Option<StoredParentLocalBridgeSession>, SessionLifecycleRepositoryError> {
    transaction
        .query_row(
            "SELECT capability_digest, digest_algorithm, capability_digest_domain,
                    audience, connection_nonce_digest, account_id, provider,
                    provider_subject, household_id, member_id, device_id,
                    authority_session_id, authority_session_generation,
                    authority_generation, authority_expires_at_epoch_millis,
                    issued_at_epoch_millis, expires_at_epoch_millis,
                    bridge_revoke_epoch, state, last_transition_at_epoch_millis
             FROM account_identity_parent_local_bridge_session
            WHERE capability_digest = ?1 LIMIT 1",
            [capability_digest],
            |row| {
                Ok(StoredParentLocalBridgeRow {
                    capability_digest: row.get(0)?,
                    digest_algorithm: row.get(1)?,
                    capability_digest_domain: row.get(2)?,
                    audience: row.get(3)?,
                    connection_nonce_digest: row.get(4)?,
                    account_id: row.get(5)?,
                    provider: row.get(6)?,
                    provider_subject: row.get(7)?,
                    household_id: row.get(8)?,
                    member_id: row.get(9)?,
                    device_id: row.get(10)?,
                    authority_session_id: row.get(11)?,
                    authority_session_generation: row.get(12)?,
                    authority_generation: row.get(13)?,
                    authority_expires_at_epoch_millis: row.get(14)?,
                    issued_at_epoch_millis: row.get(15)?,
                    expires_at_epoch_millis: row.get(16)?,
                    bridge_revoke_epoch: row.get(17)?,
                    state: row.get(18)?,
                    last_transition_at_epoch_millis: row.get(19)?,
                })
            },
        )
        .optional()
        .map_err(|_error| SessionLifecycleRepositoryError::Unavailable)?
        .map(|row| {
            decode_record(
                row,
                now_epoch_millis,
                clock_skew_millis,
                freshness_ttl_millis,
            )
        })
        .transpose()
}

fn decode_record(
    row: StoredParentLocalBridgeRow,
    now_epoch_millis: i64,
    clock_skew_millis: i64,
    freshness_ttl_millis: i64,
) -> Result<StoredParentLocalBridgeSession, SessionLifecycleRepositoryError> {
    validate_record_shape(
        &row,
        now_epoch_millis,
        clock_skew_millis,
        freshness_ttl_millis,
    )?;
    let audience = decode_audience(&row.audience)?;
    let state = decode_state(&row.state)?;
    let binding = decode_binding(&row)?;
    Ok(StoredParentLocalBridgeSession {
        capability_digest: row.capability_digest,
        audience,
        connection_nonce_digest: row.connection_nonce_digest,
        binding,
        issued_at_epoch_millis: row.issued_at_epoch_millis,
        expires_at_epoch_millis: row.expires_at_epoch_millis,
        bridge_revoke_epoch: positive_generation(row.bridge_revoke_epoch)?,
        state,
        last_transition_at_epoch_millis: row.last_transition_at_epoch_millis,
    })
}

fn validate_record_shape(
    row: &StoredParentLocalBridgeRow,
    now_epoch_millis: i64,
    clock_skew_millis: i64,
    freshness_ttl_millis: i64,
) -> Result<(), SessionLifecycleRepositoryError> {
    super::storage_time::validate_record_time_shape(
        super::storage_time::ParentLocalBridgeRecordTimeShape {
            issued_at_epoch_millis: row.issued_at_epoch_millis,
            expires_at_epoch_millis: row.expires_at_epoch_millis,
            authority_expires_at_epoch_millis: row.authority_expires_at_epoch_millis,
            last_transition_at_epoch_millis: row.last_transition_at_epoch_millis,
            now_epoch_millis,
            clock_skew_millis,
            freshness_ttl_millis,
            active: row.state == ACTIVE_STATE,
        },
    )?;
    if row.digest_algorithm != DIGEST_ALGORITHM
        || row.capability_digest_domain != CAPABILITY_DIGEST_DOMAIN
        || !is_hex_digest(&row.capability_digest)
        || !is_hex_digest(&row.connection_nonce_digest)
        || row.bridge_revoke_epoch <= 0
    {
        return Err(SessionLifecycleRepositoryError::InvalidStoredSession);
    }
    Ok(())
}

fn decode_audience(
    audience: &str,
) -> Result<AccountIdentityParentLocalBridgeAudience, SessionLifecycleRepositoryError> {
    (audience == AccountIdentityParentLocalBridgeAudience::fixed().as_str())
        .then_some(AccountIdentityParentLocalBridgeAudience::fixed())
        .ok_or(SessionLifecycleRepositoryError::InvalidStoredSession)
}

pub(super) fn current_bridge_revoke_epoch(
    transaction: &Transaction<'_>,
    account_id: &ParentAccountId,
) -> Result<u64, SessionLifecycleRepositoryError> {
    let account_id = account_id.to_string();
    let epoch = transaction
        .query_row(
            "SELECT epoch
               FROM account_identity_parent_local_bridge_revoke_epoch
              WHERE account_id = ?1",
            [account_id.as_str()],
            |row| row.get::<_, i64>(0),
        )
        .optional()
        .map_err(|_error| SessionLifecycleRepositoryError::Unavailable)?;
    match epoch {
        Some(epoch) => positive_generation(epoch),
        None => {
            let inserted = transaction
                .execute(
                    "INSERT INTO account_identity_parent_local_bridge_revoke_epoch
                         (account_id, epoch) VALUES (?1, 1)",
                    [account_id.as_str()],
                )
                .map_err(|_error| SessionLifecycleRepositoryError::CurrentnessConflict)?;
            (inserted == 1)
                .then_some(1)
                .ok_or(SessionLifecycleRepositoryError::CurrentnessConflict)
        }
    }
}

pub(super) fn advance_bridge_revoke_epoch(
    transaction: &Transaction<'_>,
    account_id: &ParentAccountId,
    expected_epoch: u64,
) -> Result<u64, SessionLifecycleRepositoryError> {
    let next_epoch = expected_epoch
        .checked_add(1)
        .ok_or(SessionLifecycleRepositoryError::InvalidTransition)?;
    let changed = transaction
        .execute(
            "UPDATE account_identity_parent_local_bridge_revoke_epoch
                SET epoch = ?2
              WHERE account_id = ?1 AND epoch = ?3",
            rusqlite::params![
                account_id.to_string(),
                super::super::codec::to_sql_generation(next_epoch)?,
                super::super::codec::to_sql_generation(expected_epoch)?,
            ],
        )
        .map_err(|_error| SessionLifecycleRepositoryError::Unavailable)?;
    if changed != 1 {
        let reloaded = current_bridge_revoke_epoch(transaction, account_id)?;
        return if reloaded != expected_epoch {
            Err(SessionLifecycleRepositoryError::CurrentnessConflict)
        } else {
            Err(SessionLifecycleRepositoryError::Unavailable)
        };
    }
    Ok(next_epoch)
}

fn decode_state(state: &str) -> Result<ParentLocalBridgeState, SessionLifecycleRepositoryError> {
    match state {
        ACTIVE_STATE => Ok(ParentLocalBridgeState::Active),
        CONSUMED_STATE => Ok(ParentLocalBridgeState::Consumed),
        REVOKED_STATE => Ok(ParentLocalBridgeState::Revoked),
        _ => Err(SessionLifecycleRepositoryError::InvalidStoredSession),
    }
}

fn decode_binding(
    row: &StoredParentLocalBridgeRow,
) -> Result<SessionAuthorityBinding, SessionLifecycleRepositoryError> {
    Ok(SessionAuthorityBinding {
        account_id: ParentAccountId::parse(row.account_id.clone())
            .ok_or(SessionLifecycleRepositoryError::InvalidStoredSession)?,
        provider: labels::parse_provider(row.provider.as_bytes())?,
        provider_subject: AccountIdentityProviderSubject::parse(row.provider_subject.clone())
            .ok_or(SessionLifecycleRepositoryError::InvalidStoredSession)?,
        household_id: FamilyId::parse(row.household_id.clone())
            .ok_or(SessionLifecycleRepositoryError::InvalidStoredSession)?,
        member_id: AccountIdentityMemberId::parse(row.member_id.clone())
            .ok_or(SessionLifecycleRepositoryError::InvalidStoredSession)?,
        device_id: AccountIdentityDeviceId::parse(row.device_id.clone())
            .ok_or(SessionLifecycleRepositoryError::InvalidStoredSession)?,
        authority_session_id: AccountIdentitySessionId::parse(row.authority_session_id.clone())
            .ok_or(SessionLifecycleRepositoryError::InvalidStoredSession)?,
        authority_session_generation: positive_generation(row.authority_session_generation)?,
        authority_generation: positive_generation(row.authority_generation)?,
        authority_expires_at_epoch_millis: row.authority_expires_at_epoch_millis,
    })
}

fn is_hex_digest(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn positive_generation(value: i64) -> Result<u64, SessionLifecycleRepositoryError> {
    u64::try_from(value)
        .ok()
        .filter(|value| *value > 0)
        .ok_or(SessionLifecycleRepositoryError::InvalidStoredSession)
}
