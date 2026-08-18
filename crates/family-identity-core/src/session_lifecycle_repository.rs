#![forbid(unsafe_code)]

//! Account-owned durable browser-session custody.

use ocentra_eventing::error::EventingError;
use ocentra_schema::account_identity_authority::{
    AccountIdentityDeviceId, AccountIdentityMemberId, AccountIdentityProviderSubject,
};
use ocentra_schema::report_query_custody::ParentAccountId;
use rusqlite::TransactionBehavior;

use crate::account_identity_authority::VerifiedAccountIdentityAuthority;
use crate::account_identity_authority_repository::SqliteAccountIdentityAuthorityRepository;
use crate::family_identity::SessionFreshnessState;
use crate::session_lifecycle::{
    authorize_session_token_action, SessionActivityState, SessionCredentialKind,
    SessionLifecycleAction, SessionTokenDecision, SessionTokenInput, TokenReplayState,
    TokenValidityWindowState,
};
use crate::session_lifecycle_custody::audit_delivery::{
    SessionAuditDeliveryAttemptId, SessionAuditEventId,
};
use crate::session_lifecycle_custody::browser_credentials::{
    IssuedBrowserSession, PresentedBrowserAccessCredential, PresentedBrowserRefreshCredential,
};
use crate::session_lifecycle_custody::record::SessionCredentialRecord;
use crate::session_lifecycle_custody::storage_values::{
    SessionAccessDigest, SessionCredentialMaterial, SessionRefreshDigest, SessionRefreshFamilyId,
};
use crate::session_lifecycle_record::SessionId;

#[path = "session_lifecycle_repository_audit.rs"]
mod audit;
#[path = "session_lifecycle_repository_authority.rs"]
mod authority;
#[path = "session_lifecycle_repository_clock.rs"]
mod clock;
#[path = "session_lifecycle_repository_codec.rs"]
mod codec;
#[path = "session_lifecycle_repository_invariants.rs"]
mod invariants;
#[path = "session_lifecycle_repository_labels.rs"]
mod labels;
#[path = "session_lifecycle_repository_schema.rs"]
mod schema;

#[derive(Debug)]
pub enum SessionLifecycleRepositoryError {
    Unavailable,
    ClockUnavailable,
    EntropyUnavailable,
    AuthorityMissing,
    AuthorityExpired,
    InvalidAuthorityBinding,
    WrongCredentialClass,
    Missing,
    ReplayRejected,
    InvalidStoredSession,
    InvalidAuditRecord,
    InvalidTransition,
    CurrentnessConflict,
    AuditConflict,
    DeliveryConflict,
    InvalidValue(EventingError),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SessionAuditAction {
    Created,
    Rotated,
    LoggedOut,
    Revoked,
    GloballyRevoked,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SessionAuditEvent {
    event_id: SessionAuditEventId,
    session_id: SessionId,
    account_id: ParentAccountId,
    provider_subject: AccountIdentityProviderSubject,
    member_id: AccountIdentityMemberId,
    device_id: AccountIdentityDeviceId,
    action: SessionAuditAction,
    occurred_at_epoch_millis: i64,
}

impl SessionAuditEvent {
    pub fn event_id(&self) -> &SessionAuditEventId {
        &self.event_id
    }

    pub fn session_id(&self) -> &SessionId {
        &self.session_id
    }

    pub fn account_id(&self) -> &ParentAccountId {
        &self.account_id
    }

    pub fn provider_subject(&self) -> &AccountIdentityProviderSubject {
        &self.provider_subject
    }

    pub fn member_id(&self) -> &AccountIdentityMemberId {
        &self.member_id
    }

    pub fn device_id(&self) -> &AccountIdentityDeviceId {
        &self.device_id
    }

    pub fn action(&self) -> SessionAuditAction {
        self.action
    }

    pub fn occurred_at_epoch_millis(&self) -> i64 {
        self.occurred_at_epoch_millis
    }
}

pub struct PendingSessionAuditDelivery {
    event: SessionAuditEvent,
    delivery_attempt_id: SessionAuditDeliveryAttemptId,
}

impl PendingSessionAuditDelivery {
    pub fn event(&self) -> &SessionAuditEvent {
        &self.event
    }
}

impl SqliteAccountIdentityAuthorityRepository {
    pub fn issue_browser_session(
        &mut self,
        current_authority: &VerifiedAccountIdentityAuthority,
    ) -> Result<IssuedBrowserSession, SessionLifecycleRepositoryError> {
        let policy = self.session_policy.clone();
        let now_epoch_millis = clock::trusted_now_epoch_millis()?;
        let material = SessionCredentialMaterial::issue()
            .map_err(|_| SessionLifecycleRepositoryError::EntropyUnavailable)?;
        let refresh_family_id = SessionRefreshFamilyId::generate()
            .map_err(|_| SessionLifecycleRepositoryError::EntropyUnavailable)?;
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
        let binding =
            authority::binding_from_verified(&transaction, current_authority, now_epoch_millis)?;
        let current_epoch = codec::current_revoke_epoch(&transaction, &binding.account_id)?;
        let record = SessionCredentialRecord::issue(
            binding,
            &material,
            refresh_family_id,
            now_epoch_millis,
            current_epoch,
            &policy,
        )
        .map_err(SessionLifecycleRepositoryError::InvalidValue)?;
        invariants::validate_record(&record)?;
        codec::insert_record(&transaction, &record)?;
        audit::insert_audit(
            &transaction,
            &record,
            SessionAuditAction::Created,
            now_epoch_millis,
        )?;
        transaction
            .commit()
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
        Ok(record.into_issued(material))
    }

    pub fn authorize_browser_session(
        &mut self,
        credential: &PresentedBrowserAccessCredential,
        action: SessionLifecycleAction,
    ) -> Result<SessionTokenDecision, SessionLifecycleRepositoryError> {
        let policy = self.session_policy.clone();
        let now_epoch_millis = clock::trusted_now_epoch_millis()?;
        let digest = SessionAccessDigest::from_presented(credential);
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
        let Some(record) = codec::read_by_access_digest(&transaction, &digest)? else {
            return Ok(missing_browser_session_decision(action));
        };
        authority::binding_for_record_current(&transaction, &record, now_epoch_millis)?;
        let current_epoch = codec::current_revoke_epoch(&transaction, &record.binding.account_id)?;
        let decision = record.authorize(action, now_epoch_millis, current_epoch, &policy);
        transaction
            .commit()
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
        Ok(decision)
    }

    pub fn rotate_browser_session(
        &mut self,
        credential: &PresentedBrowserRefreshCredential,
    ) -> Result<IssuedBrowserSession, SessionLifecycleRepositoryError> {
        let policy = self.session_policy.clone();
        let trusted_now = clock::trusted_now_epoch_millis()?;
        let digest = SessionRefreshDigest::from_presented(credential);
        let material = SessionCredentialMaterial::issue()
            .map_err(|_| SessionLifecycleRepositoryError::EntropyUnavailable)?;
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
        let Some(current) = codec::read_by_refresh_digest(&transaction, &digest)? else {
            return if codec::refresh_was_consumed(&transaction, &digest)? {
                Err(SessionLifecycleRepositoryError::ReplayRejected)
            } else {
                Err(SessionLifecycleRepositoryError::Missing)
            };
        };
        let binding = authority::binding_for_record_current(&transaction, &current, trusted_now)?;
        let current_epoch = codec::current_revoke_epoch(&transaction, &current.binding.account_id)?;
        if !current.refresh_is_current(trusted_now, current_epoch, &policy) {
            return Err(SessionLifecycleRepositoryError::ReplayRejected);
        }
        let transitioned_at = clock::monotonic_transition_epoch_millis(
            trusted_now,
            current.last_transition_at_epoch_millis,
        )?;
        let next = current
            .rotated(binding, &material, transitioned_at, &policy)
            .map_err(SessionLifecycleRepositoryError::InvalidValue)?;
        invariants::validate_record(&next)?;
        codec::register_consumed_refresh(&transaction, &current, transitioned_at)?;
        codec::rotate_record(&transaction, &current, &next)?;
        audit::insert_audit(
            &transaction,
            &next,
            SessionAuditAction::Rotated,
            transitioned_at,
        )?;
        transaction
            .commit()
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
        Ok(next.into_issued(material))
    }

    pub fn logout_browser_session(
        &mut self,
        credential: &PresentedBrowserAccessCredential,
    ) -> Result<(), SessionLifecycleRepositoryError> {
        self.transition_browser_session(credential, SessionActivityState::LoggedOut)
    }

    pub fn revoke_browser_session(
        &mut self,
        credential: &PresentedBrowserAccessCredential,
    ) -> Result<(), SessionLifecycleRepositoryError> {
        self.transition_browser_session(credential, SessionActivityState::Revoked)
    }

    pub fn revoke_all_browser_sessions(
        &mut self,
        current_authority: &VerifiedAccountIdentityAuthority,
    ) -> Result<u64, SessionLifecycleRepositoryError> {
        let trusted_now = clock::trusted_now_epoch_millis()?;
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
        let binding =
            authority::binding_from_verified(&transaction, current_authority, trusted_now)?;
        let current_epoch = codec::current_revoke_epoch(&transaction, &binding.account_id)?;
        let sessions = codec::read_active_for_account(&transaction, &binding.account_id)?;
        let next_epoch =
            codec::advance_revoke_epoch(&transaction, &binding.account_id, current_epoch)?;
        for session in sessions {
            if session.global_revoke_epoch != current_epoch {
                return Err(SessionLifecycleRepositoryError::CurrentnessConflict);
            }
            let transitioned_at = clock::monotonic_transition_epoch_millis(
                trusted_now,
                session.last_transition_at_epoch_millis,
            )?;
            codec::transition_activity(
                &transaction,
                &session,
                SessionActivityState::GloballyRevoked,
                transitioned_at,
            )?;
            audit::insert_audit(
                &transaction,
                &session,
                SessionAuditAction::GloballyRevoked,
                transitioned_at,
            )?;
        }
        transaction
            .commit()
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
        Ok(next_epoch)
    }

    fn transition_browser_session(
        &mut self,
        credential: &PresentedBrowserAccessCredential,
        activity_state: SessionActivityState,
    ) -> Result<(), SessionLifecycleRepositoryError> {
        let trusted_now = clock::trusted_now_epoch_millis()?;
        let digest = SessionAccessDigest::from_presented(credential);
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
        let current = codec::read_by_access_digest(&transaction, &digest)?
            .ok_or(SessionLifecycleRepositoryError::Missing)?;
        authority::binding_for_record_current(&transaction, &current, trusted_now)?;
        let current_epoch = codec::current_revoke_epoch(&transaction, &current.binding.account_id)?;
        if current.global_revoke_epoch != current_epoch
            || current.activity_state != SessionActivityState::Active
        {
            return Err(SessionLifecycleRepositoryError::CurrentnessConflict);
        }
        let transitioned_at = clock::monotonic_transition_epoch_millis(
            trusted_now,
            current.last_transition_at_epoch_millis,
        )?;
        codec::transition_activity(&transaction, &current, activity_state, transitioned_at)?;
        audit::insert_audit(
            &transaction,
            &current,
            labels::audit_action_for_state(activity_state),
            transitioned_at,
        )?;
        transaction
            .commit()
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)
    }
}

fn missing_browser_session_decision(action: SessionLifecycleAction) -> SessionTokenDecision {
    authorize_session_token_action(SessionTokenInput {
        credential_kind: SessionCredentialKind::BrowserUserSession,
        action,
        activity_state: SessionActivityState::Revoked,
        replay_state: TokenReplayState::ReplayDetected,
        validity_window_state: TokenValidityWindowState::Expired,
        session_freshness_state: SessionFreshnessState::Stale,
    })
}

pub(crate) const SESSION_SCHEMA_SQL: &str = schema::SESSION_SCHEMA_SQL;
