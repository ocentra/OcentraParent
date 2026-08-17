#![forbid(unsafe_code)]

//! Account-owned durable session custody.

use ocentra_eventing::error::EventingError;
use rusqlite::params;

use crate::account_identity_authority_repository::SqliteAccountIdentityAuthorityRepository;
use crate::family_identity::SessionFreshnessState;
use crate::family_identity_account::AccountUserId;
use crate::session_lifecycle::{
    authorize_session_token_action, SessionActivityState, SessionCredentialKind,
    SessionLifecycleAction, SessionTokenDecision, SessionTokenInput, TokenReplayState,
    TokenValidityWindowState,
};
use crate::session_lifecycle_custody::{
    SessionAuditEventId, SessionCredentialRecord, SessionTimestamp, SessionTokenDigest,
};
use crate::session_lifecycle_record::SessionId;

#[path = "session_lifecycle_repository_codec.rs"]
mod codec;
#[path = "session_lifecycle_repository_schema.rs"]
mod schema;

#[derive(Debug)]
pub enum SessionLifecycleRepositoryError {
    Unavailable,
    Missing,
    ReplayRejected,
    InvalidStoredSession,
    InvalidTransition,
    CurrentnessConflict,
    InvalidValue(EventingError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionAuditAction {
    Created,
    Rotated,
    LoggedOut,
    Revoked,
    GloballyRevoked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionAuditEvent {
    pub event_id: String,
    pub session_id: SessionId,
    pub account_user_id: AccountUserId,
    pub action: SessionAuditAction,
    pub occurred_at: SessionTimestamp,
}

impl SqliteAccountIdentityAuthorityRepository {
    pub fn insert_session(
        &mut self,
        record: &SessionCredentialRecord,
    ) -> Result<(), SessionLifecycleRepositoryError> {
        if record.refresh_generation != 1 {
            return Err(SessionLifecycleRepositoryError::InvalidTransition);
        }
        let transaction = self
            .connection
            .unchecked_transaction()
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
        let current_epoch = codec::current_revoke_epoch(&transaction, &record.account_user_id)?;
        if record.global_revoke_epoch != current_epoch {
            return Err(SessionLifecycleRepositoryError::CurrentnessConflict);
        }
        codec::insert_record(&transaction, record)?;
        codec::insert_audit(
            &transaction,
            record,
            SessionAuditAction::Created,
            &record.issued_at,
        )?;
        transaction
            .commit()
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)
    }

    pub fn authorize_browser_session(
        &mut self,
        token_digest: &SessionTokenDigest,
        action: SessionLifecycleAction,
        observed_at: &SessionTimestamp,
    ) -> Result<SessionTokenDecision, SessionLifecycleRepositoryError> {
        let transaction = self
            .connection
            .unchecked_transaction()
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
        let Some(record) = codec::read_by_digest(&transaction, token_digest)? else {
            return Ok(authorize_session_token_action(SessionTokenInput {
                credential_kind: SessionCredentialKind::BrowserUserSession,
                action,
                activity_state: SessionActivityState::Revoked,
                replay_state: TokenReplayState::ReplayDetected,
                validity_window_state: TokenValidityWindowState::Expired,
                session_freshness_state: SessionFreshnessState::Stale,
            }));
        };
        let current_epoch = codec::current_revoke_epoch(&transaction, &record.account_user_id)?;
        let decision = record.authorize(
            SessionCredentialKind::BrowserUserSession,
            action,
            TokenReplayState::Fresh,
            observed_at,
            current_epoch,
        );
        transaction
            .commit()
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
        Ok(decision)
    }

    pub fn rotate_session(
        &mut self,
        current_digest: &SessionTokenDigest,
        next: &SessionCredentialRecord,
        transitioned_at: &SessionTimestamp,
    ) -> Result<(), SessionLifecycleRepositoryError> {
        let transaction = self
            .connection
            .unchecked_transaction()
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
        let current = codec::read_by_digest(&transaction, current_digest)?
            .ok_or(SessionLifecycleRepositoryError::ReplayRejected)?;
        let epoch = codec::current_revoke_epoch(&transaction, &current.account_user_id)?;
        if current.activity_state != SessionActivityState::Active
            || current.global_revoke_epoch != epoch
            || next.account_user_id != current.account_user_id
            || next.refresh_family_id != current.refresh_family_id
            || next.refresh_generation != current.refresh_generation.saturating_add(1)
            || next.global_revoke_epoch != epoch
            || current.validity_window_state_at(transitioned_at) != TokenValidityWindowState::Valid
        {
            return Err(SessionLifecycleRepositoryError::InvalidTransition);
        }
        let changed = transaction
            .execute(
                "UPDATE account_identity_session
                 SET activity_state = 'revoked', freshness_state = 'stale', last_transition_at = ?2
                 WHERE token_digest = ?1 AND activity_state = 'active'",
                params![current_digest.as_str(), transitioned_at.as_str()],
            )
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
        if changed != 1 {
            return Err(SessionLifecycleRepositoryError::ReplayRejected);
        }
        codec::insert_record(&transaction, next)?;
        codec::insert_audit(
            &transaction,
            &current,
            SessionAuditAction::Rotated,
            transitioned_at,
        )?;
        transaction
            .commit()
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)
    }

    pub fn logout_session(
        &mut self,
        token_digest: &SessionTokenDigest,
        transitioned_at: &SessionTimestamp,
    ) -> Result<(), SessionLifecycleRepositoryError> {
        self.transition_session(
            token_digest,
            SessionActivityState::LoggedOut,
            transitioned_at,
        )
    }

    pub fn revoke_session(
        &mut self,
        token_digest: &SessionTokenDigest,
        transitioned_at: &SessionTimestamp,
    ) -> Result<(), SessionLifecycleRepositoryError> {
        self.transition_session(token_digest, SessionActivityState::Revoked, transitioned_at)
    }

    pub fn revoke_all_sessions(
        &mut self,
        account_user_id: &AccountUserId,
        transitioned_at: &SessionTimestamp,
    ) -> Result<u64, SessionLifecycleRepositoryError> {
        let transaction = self
            .connection
            .unchecked_transaction()
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
        let next_epoch = codec::current_revoke_epoch(&transaction, account_user_id)?
            .checked_add(1)
            .ok_or(SessionLifecycleRepositoryError::InvalidTransition)?;
        transaction
            .execute(
                "INSERT INTO account_identity_session_revoke_epoch (account_user_id, epoch)
                 VALUES (?1, ?2) ON CONFLICT(account_user_id) DO UPDATE SET epoch = excluded.epoch",
                params![
                    account_user_id.as_str(),
                    codec::to_sql_generation(next_epoch)?
                ],
            )
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
        transaction
            .execute(
                "INSERT INTO account_identity_session_audit_outbox
                 (event_id, session_id, account_user_id, action, occurred_at, delivery_state)
                 SELECT 'account-session:' || session_id || ':global-' || ?2,
                        session_id, account_user_id, 'globally-revoked', ?3, 'pending'
                 FROM account_identity_session
                 WHERE account_user_id = ?1 AND activity_state = 'active'
                 ON CONFLICT(event_id) DO NOTHING",
                params![
                    account_user_id.as_str(),
                    codec::to_sql_generation(next_epoch)?,
                    transitioned_at.as_str()
                ],
            )
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
        transaction
            .execute(
                "UPDATE account_identity_session
                 SET activity_state = 'globally-revoked', freshness_state = 'stale',
                     last_transition_at = ?2
                 WHERE account_user_id = ?1 AND activity_state = 'active'",
                params![account_user_id.as_str(), transitioned_at.as_str()],
            )
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
        transaction
            .commit()
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
        Ok(next_epoch)
    }

    pub fn pending_session_audit_events(
        &self,
    ) -> Result<Vec<SessionAuditEvent>, SessionLifecycleRepositoryError> {
        let mut statement = self
            .connection
            .prepare(
                "SELECT event_id, session_id, account_user_id, action, occurred_at
                 FROM account_identity_session_audit_outbox
                 WHERE delivery_state = 'pending' ORDER BY sequence",
            )
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
        let rows = statement
            .query_map([], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, String>(4)?,
                ))
            })
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
        rows.map(|row| {
            let (event_id, session_id, account_user_id, action, occurred_at) =
                row.map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
            Ok(SessionAuditEvent {
                event_id,
                session_id: SessionId::parse(session_id)
                    .map_err(SessionLifecycleRepositoryError::InvalidValue)?,
                account_user_id: AccountUserId::parse(account_user_id)
                    .map_err(SessionLifecycleRepositoryError::InvalidValue)?,
                action: codec::labels::parse_audit_action(action.as_bytes())?,
                occurred_at: SessionTimestamp::parse(occurred_at)
                    .map_err(SessionLifecycleRepositoryError::InvalidValue)?,
            })
        })
        .collect::<Result<Vec<_>, _>>()
    }

    pub fn mark_session_audit_delivered(
        &self,
        event_id: &SessionAuditEventId,
    ) -> Result<(), SessionLifecycleRepositoryError> {
        let changed = self
            .connection
            .execute(
                "UPDATE account_identity_session_audit_outbox
                 SET delivery_state = 'delivered'
                 WHERE event_id = ?1 AND delivery_state = 'pending'",
                [event_id.as_str()],
            )
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
        (changed == 1)
            .then_some(())
            .ok_or(SessionLifecycleRepositoryError::Missing)
    }

    fn transition_session(
        &mut self,
        token_digest: &SessionTokenDigest,
        activity_state: SessionActivityState,
        transitioned_at: &SessionTimestamp,
    ) -> Result<(), SessionLifecycleRepositoryError> {
        let transaction = self
            .connection
            .unchecked_transaction()
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
        let record = codec::read_by_digest(&transaction, token_digest)?
            .ok_or(SessionLifecycleRepositoryError::Missing)?;
        if record.activity_state != SessionActivityState::Active {
            return Err(SessionLifecycleRepositoryError::InvalidTransition);
        }
        let changed = transaction
            .execute(
                "UPDATE account_identity_session
                 SET activity_state = ?2, freshness_state = 'stale', last_transition_at = ?3
                 WHERE token_digest = ?1 AND activity_state = 'active'",
                params![
                    token_digest.as_str(),
                    codec::labels::activity_label(activity_state).0,
                    transitioned_at.as_str()
                ],
            )
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
        if changed != 1 {
            return Err(SessionLifecycleRepositoryError::CurrentnessConflict);
        }
        codec::insert_audit(
            &transaction,
            &record,
            codec::labels::audit_action_for_state(activity_state),
            transitioned_at,
        )?;
        transaction
            .commit()
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)
    }
}

pub(crate) const SESSION_SCHEMA_SQL: &str = schema::SESSION_SCHEMA_SQL;
