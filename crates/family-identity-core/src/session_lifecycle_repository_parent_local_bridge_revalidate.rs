#![forbid(unsafe_code)]

//! Currentness revalidation for authenticated parent-local bridges.

use rusqlite::TransactionBehavior;

use crate::session_lifecycle_custody::authenticated_parent_local_bridge::AuthenticatedParentLocalBridgeSession;
use crate::session_lifecycle_custody::parent_local_bridge::connection_nonce_digest;

use super::super::{authority, clock, SessionLifecycleRepositoryError};
use super::{audit, ParentLocalBridgeState};

impl super::super::SqliteAccountIdentityAuthorityRepository {
    /// Revalidate an authenticated connection against Account currentness and
    /// the bridge-specific revoke epoch. Revocation and authority rotation therefore
    /// fail closed after the initial handshake as well.
    pub fn revalidate_parent_local_bridge_session(
        &mut self,
        authenticated: &AuthenticatedParentLocalBridgeSession,
    ) -> Result<(), SessionLifecycleRepositoryError> {
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
        let now_epoch_millis = clock::trusted_now_in_transaction(&transaction)?;
        let record = super::storage::read_record(
            &transaction,
            authenticated.capability_digest(),
            now_epoch_millis,
            self.session_policy.clock_skew_millis,
            self.session_policy.freshness_ttl_millis,
        )?
        .ok_or(SessionLifecycleRepositoryError::Missing)?;
        reject_non_consumed(record.state)?;
        ensure_binding_matches(&record, authenticated)?;
        if now_epoch_millis >= record.expires_at_epoch_millis {
            return Err(SessionLifecycleRepositoryError::ParentLocalBridgeExpired);
        }
        let current_epoch =
            super::storage::current_bridge_revoke_epoch(&transaction, &record.binding.account_id)?;
        if current_epoch != record.bridge_revoke_epoch {
            return Err(SessionLifecycleRepositoryError::CurrentnessConflict);
        }
        let role = authority::parent_local_bridge_current_role(
            &transaction,
            &record.binding,
            now_epoch_millis,
        )?;
        if role != authenticated.role() {
            return Err(SessionLifecycleRepositoryError::CurrentnessConflict);
        }
        audit::cleanup(&transaction, &record.binding.account_id, now_epoch_millis)?;
        transaction
            .commit()
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)
    }
}

fn reject_non_consumed(
    state: ParentLocalBridgeState,
) -> Result<(), SessionLifecycleRepositoryError> {
    match state {
        ParentLocalBridgeState::Consumed => Ok(()),
        ParentLocalBridgeState::Revoked => {
            Err(SessionLifecycleRepositoryError::ParentLocalBridgeRevoked)
        }
        ParentLocalBridgeState::Active => Err(SessionLifecycleRepositoryError::CurrentnessConflict),
    }
}

fn ensure_binding_matches(
    record: &super::StoredParentLocalBridgeSession,
    authenticated: &AuthenticatedParentLocalBridgeSession,
) -> Result<(), SessionLifecycleRepositoryError> {
    let authenticated_nonce_digest = connection_nonce_digest(authenticated.connection_nonce())
        .ok_or(SessionLifecycleRepositoryError::InvalidParentLocalBridgeHandshake)?;
    let binding_matches = record.audience == authenticated.audience()
        && record.connection_nonce_digest == authenticated_nonce_digest
        && record.expires_at_epoch_millis == authenticated.expires_at_epoch_millis()
        && record.binding.account_id == *authenticated.account_id()
        && record.binding.household_id == *authenticated.household_id()
        && record.binding.member_id == *authenticated.actor_id()
        && record.binding.device_id == *authenticated.controller_device_id()
        && record.binding.authority_session_id == *authenticated.session_id()
        && record.binding.authority_session_generation == authenticated.session_generation()
        && record.binding.authority_generation == authenticated.authority_generation();
    binding_matches
        .then_some(())
        .ok_or(SessionLifecycleRepositoryError::CurrentnessConflict)
}
