#![forbid(unsafe_code)]

//! Explicit revocation for parent-local bridge sessions.

use rusqlite::{params, TransactionBehavior};

use crate::session_lifecycle_custody::authenticated_parent_local_bridge::AuthenticatedParentLocalBridgeSession;
use crate::session_lifecycle_custody::parent_local_bridge::ParentLocalBridgeSessionCapability;

use super::super::{clock, SessionLifecycleRepositoryError};
use super::{ParentLocalBridgeState, REVOKED_STATE};

impl super::super::SqliteAccountIdentityAuthorityRepository {
    /// Revoke an unconsumed capability before it is presented.
    pub fn revoke_parent_local_bridge_session(
        &mut self,
        capability: &ParentLocalBridgeSessionCapability,
    ) -> Result<(), SessionLifecycleRepositoryError> {
        self.revoke_parent_local_bridge_digest(capability.digest())
    }

    /// Revoke an already-authenticated connection at the Account owner. The
    /// digest is retained only inside the non-serializable authenticated result.
    pub fn revoke_authenticated_parent_local_bridge_session(
        &mut self,
        authenticated: &AuthenticatedParentLocalBridgeSession,
    ) -> Result<(), SessionLifecycleRepositoryError> {
        self.revoke_parent_local_bridge_digest(authenticated.capability_digest().to_owned())
    }

    fn revoke_parent_local_bridge_digest(
        &mut self,
        capability_digest: String,
    ) -> Result<(), SessionLifecycleRepositoryError> {
        let now_epoch_millis = clock::trusted_now_epoch_millis()?;
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
        let record = super::storage::read_record(&transaction, &capability_digest)?
            .ok_or(SessionLifecycleRepositoryError::Missing)?;
        if record.state == ParentLocalBridgeState::Revoked {
            return Err(SessionLifecycleRepositoryError::ParentLocalBridgeRevoked);
        }
        let transitioned_at = clock::monotonic_transition_epoch_millis(
            now_epoch_millis,
            record.last_transition_at_epoch_millis,
        )?;
        let changed = transaction
            .execute(
                "UPDATE account_identity_parent_local_bridge_session
                 SET state = ?2, last_transition_at_epoch_millis = ?3
                 WHERE capability_digest = ?1 AND state IN ('active','consumed')
                   AND last_transition_at_epoch_millis = ?4",
                params![
                    record.capability_digest,
                    REVOKED_STATE,
                    transitioned_at,
                    record.last_transition_at_epoch_millis,
                ],
            )
            .map_err(|_| SessionLifecycleRepositoryError::CurrentnessConflict)?;
        if changed != 1 {
            return Err(SessionLifecycleRepositoryError::CurrentnessConflict);
        }
        transaction
            .commit()
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)
    }
}
