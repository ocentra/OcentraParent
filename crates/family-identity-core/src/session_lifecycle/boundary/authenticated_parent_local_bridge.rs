#![forbid(unsafe_code)]

//! Account-owned binding returned after a parent-local bridge handshake.

use std::fmt;

use ocentra_schema::account_identity_authority::{
    AccountIdentityDeviceId, AccountIdentityMemberId, AccountIdentityRole, AccountIdentitySessionId,
};
use ocentra_schema::account_identity_parent_local_bridge::AccountIdentityParentLocalBridgeAudience;
use ocentra_schema::report_query_custody::{FamilyId, ParentAccountId};

/// Authenticated Account binding returned only after the repository consumes
/// a matching handshake. It carries no bearer secret and is not reconstructible
/// from a serialized DTO or caller-supplied identity fields.
pub struct AuthenticatedParentLocalBridgeSession {
    capability_digest: String,
    account_id: ParentAccountId,
    actor_id: AccountIdentityMemberId,
    household_id: FamilyId,
    controller_device_id: AccountIdentityDeviceId,
    role: AccountIdentityRole,
    session_id: AccountIdentitySessionId,
    session_generation: u64,
    authority_generation: u64,
    audience: AccountIdentityParentLocalBridgeAudience,
    connection_nonce: String,
    expires_at_epoch_millis: i64,
}

pub(crate) struct AuthenticatedParentLocalBridgeSessionInput {
    pub(crate) capability_digest: String,
    pub(crate) account_id: ParentAccountId,
    pub(crate) actor_id: AccountIdentityMemberId,
    pub(crate) household_id: FamilyId,
    pub(crate) controller_device_id: AccountIdentityDeviceId,
    pub(crate) role: AccountIdentityRole,
    pub(crate) session_id: AccountIdentitySessionId,
    pub(crate) session_generation: u64,
    pub(crate) authority_generation: u64,
    pub(crate) audience: AccountIdentityParentLocalBridgeAudience,
    pub(crate) connection_nonce: String,
    pub(crate) expires_at_epoch_millis: i64,
}

impl fmt::Debug for AuthenticatedParentLocalBridgeSession {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AuthenticatedParentLocalBridgeSession")
            .field("capability_digest", &"<redacted>")
            .field("account_id", &self.account_id)
            .field("actor_id", &self.actor_id)
            .field("household_id", &self.household_id)
            .field("controller_device_id", &self.controller_device_id)
            .field("role", &self.role)
            .field("session_id", &self.session_id)
            .field("session_generation", &self.session_generation)
            .field("authority_generation", &self.authority_generation)
            .field("audience", &self.audience)
            .field("connection_nonce", &"<redacted>")
            .field("expires_at_epoch_millis", &self.expires_at_epoch_millis)
            .finish()
    }
}

impl AuthenticatedParentLocalBridgeSession {
    pub fn account_id(&self) -> &ParentAccountId {
        &self.account_id
    }

    pub fn actor_id(&self) -> &AccountIdentityMemberId {
        &self.actor_id
    }

    pub fn household_id(&self) -> &FamilyId {
        &self.household_id
    }

    pub fn controller_device_id(&self) -> &AccountIdentityDeviceId {
        &self.controller_device_id
    }

    pub fn role(&self) -> AccountIdentityRole {
        self.role
    }

    pub fn session_id(&self) -> &AccountIdentitySessionId {
        &self.session_id
    }

    pub fn session_generation(&self) -> u64 {
        self.session_generation
    }

    pub fn authority_generation(&self) -> u64 {
        self.authority_generation
    }

    pub fn audience(&self) -> AccountIdentityParentLocalBridgeAudience {
        self.audience
    }

    pub fn connection_nonce(&self) -> &str {
        &self.connection_nonce
    }

    pub fn expires_at_epoch_millis(&self) -> i64 {
        self.expires_at_epoch_millis
    }

    pub(crate) fn capability_digest(&self) -> &str {
        &self.capability_digest
    }

    pub(crate) fn new(input: AuthenticatedParentLocalBridgeSessionInput) -> Self {
        let AuthenticatedParentLocalBridgeSessionInput {
            capability_digest,
            account_id,
            actor_id,
            household_id,
            controller_device_id,
            role,
            session_id,
            session_generation,
            authority_generation,
            audience,
            connection_nonce,
            expires_at_epoch_millis,
        } = input;
        Self {
            capability_digest,
            account_id,
            actor_id,
            household_id,
            controller_device_id,
            role,
            session_id,
            session_generation,
            authority_generation,
            audience,
            connection_nonce,
            expires_at_epoch_millis,
        }
    }
}
