#![forbid(unsafe_code)]

use rusqlite::{params, OptionalExtension, Transaction};

use super::{SessionAuditAction, SessionLifecycleRepositoryError};
use crate::family_identity::SessionFreshnessState;
use crate::family_identity_account::AccountUserId;
use crate::session_lifecycle::SessionActivityState;
use crate::session_lifecycle_custody::{
    SessionCredentialRecord, SessionRefreshFamilyId, SessionTimestamp, SessionTokenDigest,
};
use crate::session_lifecycle_record::SessionId;

#[path = "session_lifecycle_repository_labels.rs"]
pub(crate) mod labels;

pub(crate) fn read_by_digest(
    transaction: &Transaction<'_>,
    token_digest: &SessionTokenDigest,
) -> Result<Option<SessionCredentialRecord>, SessionLifecycleRepositoryError> {
    let row = transaction
        .query_row(
            "SELECT session_id, account_user_id, refresh_family_id, refresh_generation,
                    issued_at, expires_at, activity_state, freshness_state,
                    global_revoke_epoch, last_transition_at
             FROM account_identity_session WHERE token_digest = ?1 LIMIT 1",
            [token_digest.as_str()],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, i64>(3)?,
                    row.get::<_, String>(4)?,
                    row.get::<_, String>(5)?,
                    row.get::<_, String>(6)?,
                    row.get::<_, String>(7)?,
                    row.get::<_, i64>(8)?,
                    row.get::<_, String>(9)?,
                ))
            },
        )
        .optional()
        .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
    let Some((
        session_id,
        account_user_id,
        refresh_family_id,
        refresh_generation,
        issued_at,
        expires_at,
        activity_state,
        freshness_state,
        global_revoke_epoch,
        last_transition_at,
    )) = row
    else {
        return Ok(None);
    };
    let digest_is_valid = token_digest.as_str().len() == 64
        && token_digest
            .as_str()
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit());
    if !digest_is_valid {
        return Err(SessionLifecycleRepositoryError::InvalidStoredSession);
    }
    Ok(Some(SessionCredentialRecord {
        session_id: SessionId::parse(session_id)
            .map_err(SessionLifecycleRepositoryError::InvalidValue)?,
        account_user_id: AccountUserId::parse(account_user_id)
            .map_err(SessionLifecycleRepositoryError::InvalidValue)?,
        token_digest: token_digest.clone(),
        refresh_family_id: SessionRefreshFamilyId::parse(refresh_family_id)
            .map_err(SessionLifecycleRepositoryError::InvalidValue)?,
        refresh_generation: from_sql_generation(refresh_generation)?,
        issued_at: SessionTimestamp::parse(issued_at)
            .map_err(SessionLifecycleRepositoryError::InvalidValue)?,
        expires_at: SessionTimestamp::parse(expires_at)
            .map_err(SessionLifecycleRepositoryError::InvalidValue)?,
        activity_state: labels::parse_activity_state(activity_state.as_bytes())?,
        freshness_state: labels::parse_freshness_state(freshness_state.as_bytes())?,
        global_revoke_epoch: from_sql_generation(global_revoke_epoch)?,
        last_transition_at: SessionTimestamp::parse(last_transition_at)
            .map_err(SessionLifecycleRepositoryError::InvalidValue)?,
    }))
}

pub(crate) fn insert_record(
    transaction: &Transaction<'_>,
    record: &SessionCredentialRecord,
) -> Result<(), SessionLifecycleRepositoryError> {
    let changed = transaction
        .execute(
            "INSERT INTO account_identity_session (
                 token_digest, session_id, account_user_id, refresh_family_id,
                 refresh_generation, issued_at, expires_at, activity_state,
                 freshness_state, global_revoke_epoch, last_transition_at
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                record.token_digest.as_str(),
                record.session_id.as_str(),
                record.account_user_id.as_str(),
                record.refresh_family_id.as_str(),
                to_sql_generation(record.refresh_generation)?,
                record.issued_at.as_str(),
                record.expires_at.as_str(),
                labels::activity_label(record.activity_state).0,
                labels::freshness_label(record.freshness_state).0,
                to_sql_generation(record.global_revoke_epoch)?,
                record.last_transition_at.as_str(),
            ],
        )
        .map_err(|_| SessionLifecycleRepositoryError::CurrentnessConflict)?;
    (changed == 1)
        .then_some(())
        .ok_or(SessionLifecycleRepositoryError::CurrentnessConflict)
}

pub(crate) fn insert_audit(
    transaction: &Transaction<'_>,
    record: &SessionCredentialRecord,
    action: SessionAuditAction,
    occurred_at: &SessionTimestamp,
) -> Result<(), SessionLifecycleRepositoryError> {
    let event_id = format!(
        "account-session:{}:{}:{}",
        record.session_id,
        record.refresh_generation,
        labels::audit_label(&action).0
    );
    transaction
        .execute(
            "INSERT INTO account_identity_session_audit_outbox
             (event_id, session_id, account_user_id, action, occurred_at, delivery_state)
             VALUES (?1, ?2, ?3, ?4, ?5, 'pending')
             ON CONFLICT(event_id) DO NOTHING",
            params![
                event_id,
                record.session_id.as_str(),
                record.account_user_id.as_str(),
                labels::audit_label(&action).0,
                occurred_at.as_str(),
            ],
        )
        .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
    Ok(())
}

pub(crate) fn current_revoke_epoch(
    transaction: &Transaction<'_>,
    account_user_id: &AccountUserId,
) -> Result<u64, SessionLifecycleRepositoryError> {
    let epoch = transaction
        .query_row(
            "SELECT epoch FROM account_identity_session_revoke_epoch WHERE account_user_id = ?1",
            [account_user_id.as_str()],
            |row| row.get::<_, i64>(0),
        )
        .optional()
        .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
    match epoch {
        Some(epoch) => from_sql_generation(epoch),
        None => {
            transaction
                .execute(
                    "INSERT INTO account_identity_session_revoke_epoch (account_user_id, epoch)
                     VALUES (?1, 1) ON CONFLICT(account_user_id) DO NOTHING",
                    [account_user_id.as_str()],
                )
                .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
            Ok(1)
        }
    }
}

pub(crate) fn to_sql_generation(value: u64) -> Result<i64, SessionLifecycleRepositoryError> {
    i64::try_from(value).map_err(|_| SessionLifecycleRepositoryError::InvalidTransition)
}

fn from_sql_generation(value: i64) -> Result<u64, SessionLifecycleRepositoryError> {
    u64::try_from(value)
        .ok()
        .filter(|value| *value > 0)
        .ok_or(SessionLifecycleRepositoryError::InvalidStoredSession)
}
