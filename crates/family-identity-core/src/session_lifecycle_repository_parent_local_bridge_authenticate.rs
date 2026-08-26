#![forbid(unsafe_code)]

//! Authenticated-handshake operations for parent-local bridge sessions.

use ocentra_schema::account_identity_parent_local_bridge::AccountIdentityParentLocalBridgeHandshake;
use rusqlite::{params, TransactionBehavior};

use crate::session_lifecycle_custody::authenticated_parent_local_bridge::AuthenticatedParentLocalBridgeSession;
use crate::session_lifecycle_custody::parent_local_bridge::{
    connection_nonce_digest, ParentLocalBridgeSessionCapability,
};

use super::super::{authority, clock, codec, SessionLifecycleRepositoryError};
use super::{ParentLocalBridgeState, CONSUMED_STATE};

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
            .map_err(|_| SessionLifecycleRepositoryError::InvalidParentLocalBridgeHandshake)?;
        let capability_digest =
            ParentLocalBridgeSessionCapability::digest_presented(&handshake.capability)
                .ok_or(SessionLifecycleRepositoryError::InvalidParentLocalBridgeHandshake)?;
        let presented_nonce_digest = connection_nonce_digest(&handshake.connection_nonce)
            .ok_or(SessionLifecycleRepositoryError::InvalidParentLocalBridgeHandshake)?;
        let now_epoch_millis = clock::trusted_now_epoch_millis()?;
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
        let record = super::storage::read_record(&transaction, &capability_digest)?
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
        let current_epoch = codec::current_revoke_epoch(&transaction, &record.binding.account_id)?;
        if current_epoch != record.global_revoke_epoch {
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
        let changed = transaction
            .execute(
                "UPDATE account_identity_parent_local_bridge_session
                 SET state = ?2, last_transition_at_epoch_millis = ?3
                 WHERE capability_digest = ?1 AND state = 'active'
                   AND global_revoke_epoch = ?4
                   AND last_transition_at_epoch_millis = ?5",
                params![
                    record.capability_digest,
                    CONSUMED_STATE,
                    transitioned_at,
                    codec::to_sql_generation(record.global_revoke_epoch)?,
                    record.last_transition_at_epoch_millis,
                ],
            )
            .map_err(|_| SessionLifecycleRepositoryError::CurrentnessConflict)?;
        if changed != 1 {
            return Err(SessionLifecycleRepositoryError::ReplayRejected);
        }
        transaction
            .commit()
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
        Ok(AuthenticatedParentLocalBridgeSession::new(
            record.capability_digest,
            record.binding.account_id,
            record.binding.member_id,
            record.binding.household_id,
            record.binding.device_id,
            role,
            record.binding.authority_session_id,
            record.binding.authority_session_generation,
            record.binding.authority_generation,
            record.audience,
            handshake.connection_nonce.clone(),
            record.expires_at_epoch_millis,
        ))
    }
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
