#![forbid(unsafe_code)]

//! Durable Account-owned custody for parent-local bridge sessions.
//!
//! This module deliberately stays provider-independent. Provider adapters only
//! establish the lookup key used to obtain `VerifiedAccountIdentityAuthority`;
//! the bridge bearer, nonce, lifecycle, and currentness checks are owned by the
//! Account SQLite repository.

use ocentra_schema::account_identity_authority::AccountIdentityRole;
use ocentra_schema::account_identity_parent_local_bridge::{
    AccountIdentityParentLocalBridgeAudience, AccountIdentityParentLocalBridgeHandshake,
};

use crate::account_identity_authority::VerifiedAccountIdentityAuthority;
use crate::account_identity_authority_repository::AccountIdentityAuthorityService;
use crate::session_lifecycle_custody::authenticated_parent_local_bridge::AuthenticatedParentLocalBridgeSession;
use crate::session_lifecycle_custody::parent_local_bridge::{
    IssuedParentLocalBridgeSession, ParentLocalBridgeSessionCapability,
};
use crate::session_lifecycle_custody::record::SessionAuthorityBinding;

use super::SessionLifecycleRepositoryError;

#[path = "session_lifecycle_repository_parent_local_bridge_audit.rs"]
mod audit;
#[path = "session_lifecycle_repository_parent_local_bridge_authenticate.rs"]
mod authenticate;
#[path = "session_lifecycle_repository_parent_local_bridge_issue.rs"]
mod issue;
#[path = "session_lifecycle_repository_parent_local_bridge_revalidate.rs"]
mod revalidate;
#[path = "session_lifecycle_repository_parent_local_bridge_revoke.rs"]
mod revoke;
#[path = "session_lifecycle_repository_parent_local_bridge_storage.rs"]
mod storage;

const DIGEST_ALGORITHM: &str = "sha256";
const ACTIVE_STATE: &str = "active";
const CONSUMED_STATE: &str = "consumed";
const REVOKED_STATE: &str = "revoked";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ParentLocalBridgeState {
    Active,
    Consumed,
    Revoked,
}

struct StoredParentLocalBridgeSession {
    capability_digest: String,
    audience: AccountIdentityParentLocalBridgeAudience,
    connection_nonce_digest: String,
    binding: SessionAuthorityBinding,
    issued_at_epoch_millis: i64,
    expires_at_epoch_millis: i64,
    bridge_revoke_epoch: u64,
    state: ParentLocalBridgeState,
    last_transition_at_epoch_millis: i64,
}

impl AccountIdentityAuthorityService {
    pub fn issue_parent_local_bridge_session(
        &mut self,
        current_authority: &VerifiedAccountIdentityAuthority,
    ) -> Result<IssuedParentLocalBridgeSession, SessionLifecycleRepositoryError> {
        self.repository
            .issue_parent_local_bridge_session(current_authority)
    }

    pub fn authenticate_parent_local_bridge_handshake(
        &mut self,
        handshake: &AccountIdentityParentLocalBridgeHandshake,
    ) -> Result<AuthenticatedParentLocalBridgeSession, SessionLifecycleRepositoryError> {
        self.repository
            .authenticate_parent_local_bridge_handshake(handshake)
    }

    pub fn revalidate_parent_local_bridge_session(
        &mut self,
        authenticated: &AuthenticatedParentLocalBridgeSession,
    ) -> Result<(), SessionLifecycleRepositoryError> {
        self.repository
            .revalidate_parent_local_bridge_session(authenticated)
    }

    pub fn revoke_parent_local_bridge_session(
        &mut self,
        current_authority: &VerifiedAccountIdentityAuthority,
        capability: &ParentLocalBridgeSessionCapability,
    ) -> Result<(), SessionLifecycleRepositoryError> {
        self.repository
            .revoke_parent_local_bridge_session(current_authority, capability)
    }

    pub fn revoke_authenticated_parent_local_bridge_session(
        &mut self,
        authenticated: &AuthenticatedParentLocalBridgeSession,
    ) -> Result<(), SessionLifecycleRepositoryError> {
        self.repository
            .revoke_authenticated_parent_local_bridge_session(authenticated)
    }

    pub fn revoke_all_parent_local_bridge_sessions(
        &mut self,
        current_authority: &VerifiedAccountIdentityAuthority,
    ) -> Result<u64, SessionLifecycleRepositoryError> {
        self.repository
            .revoke_all_parent_local_bridge_sessions(current_authority)
    }
}

pub(super) fn is_parent_owner(role: AccountIdentityRole) -> bool {
    role == AccountIdentityRole::ParentOwner
}
