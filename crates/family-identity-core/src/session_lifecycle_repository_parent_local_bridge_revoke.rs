#![forbid(unsafe_code)]

//! Explicit Account-owned revocation for parent-local bridge sessions.

use rusqlite::{params, Transaction, TransactionBehavior};

use crate::account_identity_authority::VerifiedAccountIdentityAuthority;
use crate::session_lifecycle_custody::authenticated_parent_local_bridge::AuthenticatedParentLocalBridgeSession;
use crate::session_lifecycle_custody::parent_local_bridge::{
    connection_nonce_digest, ParentLocalBridgeSessionCapability,
};

use super::super::{authority, clock, SessionLifecycleRepositoryError};
use super::{audit, ParentLocalBridgeState, StoredParentLocalBridgeSession, REVOKED_STATE};

impl super::super::SqliteAccountIdentityAuthorityRepository {
    /// Revoke one unconsumed capability after re-resolving the exact current
    /// Account authority in the same transaction. A bearer alone is never a
    /// sufficient revocation authority.
    pub fn revoke_parent_local_bridge_session(
        &mut self,
        current_authority: &VerifiedAccountIdentityAuthority,
        capability: &ParentLocalBridgeSessionCapability,
    ) -> Result<(), SessionLifecycleRepositoryError> {
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
        let now_epoch_millis = clock::trusted_now_in_transaction(&transaction)?;
        let current_binding = authority::parent_local_bridge_binding_from_verified(
            &transaction,
            current_authority,
            now_epoch_millis,
        )?;
        let record = super::storage::read_record(
            &transaction,
            &capability.digest(),
            now_epoch_millis,
            self.session_policy.clock_skew_millis,
            self.session_policy.freshness_ttl_millis,
        )?;
        let record = record.ok_or(SessionLifecycleRepositoryError::Missing)?;
        if record.binding != current_binding {
            return Err(SessionLifecycleRepositoryError::CurrentnessConflict);
        }
        ensure_current_bridge_epoch(&transaction, &record)?;
        let transitioned_at = transition_to_revoked(&transaction, &record, now_epoch_millis)?;
        audit::insert_session_event(&transaction, &record, "revoked", transitioned_at)?;
        audit::cleanup(&transaction, now_epoch_millis)?;
        transaction
            .commit()
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)
    }

    /// Revoke the connection represented by an authenticated, opaque
    /// holder. The stored binding, current Account authority, and bridge epoch
    /// are all checked before the transition.
    pub fn revoke_authenticated_parent_local_bridge_session(
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
        ensure_consumed(record.state)?;
        ensure_authenticated_binding(&record, authenticated)?;
        ensure_current_bridge_epoch(&transaction, &record)?;
        let role = authority::parent_local_bridge_current_role(
            &transaction,
            &record.binding,
            now_epoch_millis,
        )?;
        if role != authenticated.role() {
            return Err(SessionLifecycleRepositoryError::CurrentnessConflict);
        }
        let transitioned_at = transition_to_revoked(&transaction, &record, now_epoch_millis)?;
        audit::insert_session_event(&transaction, &record, "revoked", transitioned_at)?;
        audit::cleanup(&transaction, now_epoch_millis)?;
        transaction
            .commit()
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)
    }

    /// Advance the bridge-specific Account revoke epoch. This is a constant
    /// time global fence: every bridge row issued under the prior epoch fails
    /// closed without an unbounded scan or reuse of the browser-session fence.
    pub fn revoke_all_parent_local_bridge_sessions(
        &mut self,
        current_authority: &VerifiedAccountIdentityAuthority,
    ) -> Result<u64, SessionLifecycleRepositoryError> {
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
        let now_epoch_millis = clock::trusted_now_in_transaction(&transaction)?;
        let binding = authority::parent_local_bridge_binding_from_verified(
            &transaction,
            current_authority,
            now_epoch_millis,
        )?;
        if !super::is_parent_owner(current_authority.role()) {
            return Err(SessionLifecycleRepositoryError::WrongCredentialClass);
        }
        let current_epoch =
            super::storage::current_bridge_revoke_epoch(&transaction, &binding.account_id)?;
        let next_epoch = super::storage::advance_bridge_revoke_epoch(
            &transaction,
            &binding.account_id,
            current_epoch,
        )?;
        audit::insert_global_event(&transaction, &binding, next_epoch, now_epoch_millis)?;
        audit::cleanup(&transaction, now_epoch_millis)?;
        transaction
            .commit()
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
        Ok(next_epoch)
    }
}

fn ensure_consumed(state: ParentLocalBridgeState) -> Result<(), SessionLifecycleRepositoryError> {
    if state == ParentLocalBridgeState::Consumed {
        return Ok(());
    }
    if state == ParentLocalBridgeState::Revoked {
        return Err(SessionLifecycleRepositoryError::ParentLocalBridgeRevoked);
    }
    Err(SessionLifecycleRepositoryError::CurrentnessConflict)
}

fn ensure_current_bridge_epoch(
    transaction: &Transaction<'_>,
    record: &StoredParentLocalBridgeSession,
) -> Result<(), SessionLifecycleRepositoryError> {
    let current_epoch =
        super::storage::current_bridge_revoke_epoch(transaction, &record.binding.account_id)?;
    (current_epoch == record.bridge_revoke_epoch)
        .then_some(())
        .ok_or(SessionLifecycleRepositoryError::CurrentnessConflict)
}

fn transition_to_revoked(
    transaction: &Transaction<'_>,
    record: &StoredParentLocalBridgeSession,
    now_epoch_millis: i64,
) -> Result<i64, SessionLifecycleRepositoryError> {
    if record.state == ParentLocalBridgeState::Revoked {
        return Err(SessionLifecycleRepositoryError::ParentLocalBridgeRevoked);
    }
    let transitioned_at = super::super::clock::monotonic_transition_epoch_millis(
        now_epoch_millis,
        record.last_transition_at_epoch_millis,
    )?;
    let changed = transaction
        .execute(
            "UPDATE account_identity_parent_local_bridge_session
                SET state = ?2, last_transition_at_epoch_millis = ?3
              WHERE capability_digest = ?1
                AND state IN ('active','consumed')
                AND bridge_revoke_epoch = ?4
                AND last_transition_at_epoch_millis = ?5",
            params![
                record.capability_digest,
                REVOKED_STATE,
                transitioned_at,
                super::super::codec::to_sql_generation(record.bridge_revoke_epoch)?,
                record.last_transition_at_epoch_millis,
            ],
        )
        .map_err(|_| SessionLifecycleRepositoryError::CurrentnessConflict)?;
    (changed == 1)
        .then_some(transitioned_at)
        .ok_or(SessionLifecycleRepositoryError::CurrentnessConflict)
}

fn ensure_authenticated_binding(
    record: &StoredParentLocalBridgeSession,
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
