#![forbid(unsafe_code)]

//! Redacted Account-owned bridge audit delivery boundary.
//!
//! Delivery values retain the exact Account, household, member, device,
//! authority-session, session-generation, and authority-generation binding
//! needed by the owner adapter. Provider subjects, capability material,
//! connection nonces, and their digests never cross this boundary.

use ocentra_schema::account_identity_authority::{
    AccountIdentityDeviceId, AccountIdentityMemberId, AccountIdentitySessionId,
};
use ocentra_schema::account_identity_parent_local_bridge::AccountIdentityParentLocalBridgeAudience;
use ocentra_schema::report_query_custody::{FamilyId, ParentAccountId};

use super::audit_delivery::{SessionAuditDeliveryAttemptId, SessionAuditEventId};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParentLocalBridgeAuditAction {
    Issued,
    Authenticated,
    Revoked,
    GloballyRevoked,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParentLocalBridgeAuditEvent {
    pub(crate) event_id: SessionAuditEventId,
    pub(crate) account_id: ParentAccountId,
    pub(crate) household_id: FamilyId,
    pub(crate) member_id: AccountIdentityMemberId,
    pub(crate) device_id: AccountIdentityDeviceId,
    pub(crate) authority_session_id: AccountIdentitySessionId,
    pub(crate) authority_session_generation: u64,
    pub(crate) authority_generation: u64,
    pub(crate) audience: AccountIdentityParentLocalBridgeAudience,
    pub(crate) bridge_revoke_epoch: u64,
    pub(crate) action: ParentLocalBridgeAuditAction,
    pub(crate) occurred_at_epoch_millis: i64,
}

impl ParentLocalBridgeAuditEvent {
    pub fn event_id(&self) -> &SessionAuditEventId {
        &self.event_id
    }

    pub fn account_id(&self) -> &ParentAccountId {
        &self.account_id
    }

    pub fn household_id(&self) -> &FamilyId {
        &self.household_id
    }

    pub fn member_id(&self) -> &AccountIdentityMemberId {
        &self.member_id
    }

    pub fn device_id(&self) -> &AccountIdentityDeviceId {
        &self.device_id
    }

    pub fn authority_session_id(&self) -> &AccountIdentitySessionId {
        &self.authority_session_id
    }

    pub fn authority_session_generation(&self) -> u64 {
        self.authority_session_generation
    }

    pub fn authority_generation(&self) -> u64 {
        self.authority_generation
    }

    pub fn bridge_revoke_epoch(&self) -> u64 {
        self.bridge_revoke_epoch
    }

    pub fn action(&self) -> ParentLocalBridgeAuditAction {
        self.action
    }

    pub fn occurred_at_epoch_millis(&self) -> i64 {
        self.occurred_at_epoch_millis
    }
}

pub struct ParentLocalBridgeAuditDeliveryClaim {
    pub(crate) event: ParentLocalBridgeAuditEvent,
    pub(crate) attempt_id: SessionAuditDeliveryAttemptId,
    pub(crate) attempt_count: u64,
    pub(crate) claimed_at_epoch_millis: i64,
    pub(crate) lease_expires_at_epoch_millis: i64,
}

impl ParentLocalBridgeAuditDeliveryClaim {
    pub fn event(&self) -> &ParentLocalBridgeAuditEvent {
        &self.event
    }

    pub fn attempt_count(&self) -> u64 {
        self.attempt_count
    }

    pub fn claimed_at_epoch_millis(&self) -> i64 {
        self.claimed_at_epoch_millis
    }

    pub fn lease_expires_at_epoch_millis(&self) -> i64 {
        self.lease_expires_at_epoch_millis
    }

    pub(crate) fn attempt_id(&self) -> &SessionAuditDeliveryAttemptId {
        &self.attempt_id
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ParentLocalBridgeStartupRecovery {
    pub(crate) expired_claims_requeued: u64,
    pub(crate) terminal_sessions_removed: u64,
    pub(crate) delivered_audits_removed: u64,
    pub(crate) more_recovery_work: bool,
}
