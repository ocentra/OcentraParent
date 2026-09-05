#![forbid(unsafe_code)]

//! Authenticated-handshake operations for parent-local bridge sessions.

use ocentra_schema::account_identity_parent_local_bridge::AccountIdentityParentLocalBridgeHandshake;
use rusqlite::{params, TransactionBehavior};

use crate::session_lifecycle_custody::authenticated_parent_local_bridge::{
    AuthenticatedParentLocalBridgeSession, AuthenticatedParentLocalBridgeSessionInput,
};
use crate::session_lifecycle_custody::parent_local_bridge::{
    connection_nonce_digest, ParentLocalBridgeSessionCapability,
};

use super::super::{authority, clock, SessionLifecycleRepositoryError};
use super::{audit, ParentLocalBridgeState, CONSUMED_STATE};

impl super::super::SqliteAccountIdentityAuthorityRepository {
    /// Verify and consume one typed parent-local handshake. A consumed nonce
    /// cannot authenticate a second connection, while the resulting binding
    /// can still be revalidated by the owning runtime on later commands.
    pub fn authenticate_parent_local_bridge_handshake(
        &mut self,
        handshake: &AccountIdentityParentLocalBridgeHandshake,
    ) -> Result<AuthenticatedParentLocalBridgeSession, SessionLifecycleRepositoryError> {
        handshake
            .validate_shape()
            .map_err(|_error| SessionLifecycleRepositoryError::InvalidParentLocalBridgeHandshake)?;
        let capability_digest =
            ParentLocalBridgeSessionCapability::digest_presented(&handshake.capability)
                .ok_or(SessionLifecycleRepositoryError::InvalidParentLocalBridgeHandshake)?;
        let presented_nonce_digest = connection_nonce_digest(&handshake.connection_nonce)
            .ok_or(SessionLifecycleRepositoryError::InvalidParentLocalBridgeHandshake)?;
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_error| SessionLifecycleRepositoryError::Unavailable)?;
        let now_epoch_millis = clock::trusted_now_in_transaction(&transaction)?;
        let record = super::storage::read_record(
            &transaction,
            &capability_digest,
            now_epoch_millis,
            self.session_policy.clock_skew_millis,
            self.session_policy.freshness_ttl_millis,
        )?
        .ok_or(SessionLifecycleRepositoryError::Missing)?;
        reject_non_active(record.state)?;
        if record.audience != handshake.audience
            || record.connection_nonce_digest != presented_nonce_digest
        {
            return Err(SessionLifecycleRepositoryError::InvalidParentLocalBridgeHandshake);
        }
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
        let transitioned_at = clock::monotonic_transition_epoch_millis(
            now_epoch_millis,
            record.last_transition_at_epoch_millis,
        )?;
        consume_record(&transaction, &record, transitioned_at)?;
        audit::insert_session_event(&transaction, &record, "authenticated", transitioned_at)?;
        audit::cleanup(&transaction, &record.binding.account_id, now_epoch_millis)?;
        transaction
            .commit()
            .map_err(|_error| SessionLifecycleRepositoryError::Unavailable)?;
        Ok(AuthenticatedParentLocalBridgeSession::new(
            AuthenticatedParentLocalBridgeSessionInput {
                capability_digest: record.capability_digest,
                account_id: record.binding.account_id,
                actor_id: record.binding.member_id,
                household_id: record.binding.household_id,
                controller_device_id: record.binding.device_id,
                role,
                session_id: record.binding.authority_session_id,
                session_generation: record.binding.authority_session_generation,
                authority_generation: record.binding.authority_generation,
                audience: record.audience,
                connection_nonce: handshake.connection_nonce.clone(),
                expires_at_epoch_millis: record.expires_at_epoch_millis,
            },
        ))
    }
}

fn consume_record(
    transaction: &rusqlite::Transaction<'_>,
    record: &super::StoredParentLocalBridgeSession,
    transitioned_at: i64,
) -> Result<(), SessionLifecycleRepositoryError> {
    let changed = transaction
        .execute(
            "UPDATE account_identity_parent_local_bridge_session
             SET state = ?2, last_transition_at_epoch_millis = ?3
             WHERE capability_digest = ?1 AND state = 'active'
               AND bridge_revoke_epoch = ?4
               AND last_transition_at_epoch_millis = ?5",
            params![
                record.capability_digest,
                CONSUMED_STATE,
                transitioned_at,
                super::super::codec::to_sql_generation(record.bridge_revoke_epoch)?,
                record.last_transition_at_epoch_millis,
            ],
        )
        .map_err(|_error| SessionLifecycleRepositoryError::CurrentnessConflict)?;
    (changed == 1)
        .then_some(())
        .ok_or(SessionLifecycleRepositoryError::ReplayRejected)
}

fn reject_non_active(state: ParentLocalBridgeState) -> Result<(), SessionLifecycleRepositoryError> {
    match state {
        ParentLocalBridgeState::Active => Ok(()),
        ParentLocalBridgeState::Consumed => Err(SessionLifecycleRepositoryError::ReplayRejected),
        ParentLocalBridgeState::Revoked => {
            Err(SessionLifecycleRepositoryError::ParentLocalBridgeRevoked)
        }
    }
}
