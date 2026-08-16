//! Versioned, identifier-only account-authority handoff for downstream persistence owners.
//!
//! The handoff derives family scope from canonical family records before it delegates to the
//! household authority evaluator. It intentionally carries neither display names nor secrets.

use super::{
    ActorAccountState, ChildProfileBindingState, ChildProfileId, DeviceId, DeviceOwnershipScope,
    DeviceRegistration, DeviceTrustState, HouseholdAuthorityEvaluationId, HouseholdId,
    HouseholdRole, ParentControllerLease, ParentMember, ParentMemberId, SessionFreshnessState,
};
use crate::household_authority::{
    authorize_household_action, HouseholdAuthorityAction, HouseholdAuthorityDecision,
    HouseholdAuthorityInput, ParentControllerLeaseState,
};
use crate::parent_presence::ParentPresenceObservedAt;
use serde::{Deserialize, Serialize};

pub const HOUSEHOLD_AUTHORITY_HANDOFF_SCHEMA_VERSION: u16 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HouseholdAuthorityHandoffRedactionState {
    #[serde(rename = "identifiers-only")]
    IdentifiersOnly,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HouseholdAuthorityChildTarget {
    pub child_id: ChildProfileId,
    pub household_id: HouseholdId,
    pub device_ids: Vec<DeviceId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParentControllerDeviceTrustProof {
    pub parent_member_id: ParentMemberId,
    pub household_id: HouseholdId,
    pub device_id: DeviceId,
    pub role: HouseholdRole,
    pub trust_state: DeviceTrustState,
    pub stale_since: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HouseholdAuthorityHandoffRequest {
    pub evaluation_id: HouseholdAuthorityEvaluationId,
    pub parent_member: ParentMember,
    pub child_target: HouseholdAuthorityChildTarget,
    pub device_registration: DeviceRegistration,
    pub parent_controller_device: ParentControllerDeviceTrustProof,
    pub actor_account_state: ActorAccountState,
    pub session_freshness_state: SessionFreshnessState,
    pub capability_granted: bool,
    pub controller_lease: Option<ParentControllerLease>,
    pub observed_at: ParentPresenceObservedAt,
    pub action: HouseholdAuthorityAction,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HouseholdAuthorityHandoffDecision {
    pub schema_version: u16,
    pub evaluation_id: HouseholdAuthorityEvaluationId,
    pub household_id: HouseholdId,
    pub parent_member_id: super::ParentMemberId,
    pub child_profile_id: super::ChildProfileId,
    pub device_id: super::DeviceId,
    pub action: HouseholdAuthorityAction,
    pub decision: HouseholdAuthorityDecision,
    pub redaction_state: HouseholdAuthorityHandoffRedactionState,
}

/// Derives one persistence-safe authority decision from canonical account/family records.
///
/// Downstream storage/runtime owners receive only stable identifiers and the decision. They do
/// not receive profile display names, timestamps, invite/recovery secrets, or session material.
pub fn evaluate_household_authority_handoff(
    request: HouseholdAuthorityHandoffRequest,
) -> HouseholdAuthorityHandoffDecision {
    let household_id = request.parent_member.household_id.clone();
    let decision = authorize_household_action(HouseholdAuthorityInput {
        actor_role: request.parent_member.role,
        same_family: same_family(&request),
        actor_account_state: request.actor_account_state,
        membership_state: request.parent_member.invite_state,
        child_profile_binding_state: child_profile_binding_state(&request),
        device_ownership_scope: device_ownership_scope(&request),
        device_trust_state: device_trust_state(&request),
        session_freshness_state: request.session_freshness_state,
        capability_granted: request.capability_granted,
        controller_lease_state: controller_lease_state(&request),
        action: request.action,
    });

    HouseholdAuthorityHandoffDecision {
        schema_version: HOUSEHOLD_AUTHORITY_HANDOFF_SCHEMA_VERSION,
        evaluation_id: request.evaluation_id,
        household_id,
        parent_member_id: request.parent_member.member_id,
        child_profile_id: request.child_target.child_id,
        device_id: request.device_registration.device_id,
        action: request.action,
        decision,
        redaction_state: HouseholdAuthorityHandoffRedactionState::IdentifiersOnly,
    }
}

fn same_family(request: &HouseholdAuthorityHandoffRequest) -> bool {
    request.parent_member.household_id == request.child_target.household_id
        && request.parent_member.household_id == request.device_registration.household_id
        && request.parent_member.household_id == request.parent_controller_device.household_id
}

fn child_profile_binding_state(
    request: &HouseholdAuthorityHandoffRequest,
) -> ChildProfileBindingState {
    if request.device_registration.child_id == request.child_target.child_id
        && request.device_registration.household_id == request.child_target.household_id
        && request
            .child_target
            .device_ids
            .contains(&request.device_registration.device_id)
    {
        ChildProfileBindingState::Bound
    } else {
        ChildProfileBindingState::Missing
    }
}

fn device_ownership_scope(request: &HouseholdAuthorityHandoffRequest) -> DeviceOwnershipScope {
    if request.device_registration.role_label == super::HouseholdRole::ChildDeviceAgent {
        DeviceOwnershipScope::ChildProfileDevice
    } else {
        DeviceOwnershipScope::OtherDevice
    }
}

fn device_trust_state(request: &HouseholdAuthorityHandoffRequest) -> DeviceTrustState {
    if request.device_registration.stale_since.is_some() {
        return DeviceTrustState::Revoked;
    }

    if request.device_registration.trust_state != DeviceTrustState::Trusted {
        return request.device_registration.trust_state;
    }

    parent_controller_trust_state(request)
}

fn parent_controller_trust_state(request: &HouseholdAuthorityHandoffRequest) -> DeviceTrustState {
    if request.parent_controller_device.parent_member_id != request.parent_member.member_id
        || request.parent_controller_device.role != request.parent_member.role
        || request.parent_controller_device.stale_since.is_some()
    {
        DeviceTrustState::Revoked
    } else {
        request.parent_controller_device.trust_state
    }
}

fn controller_lease_state(
    request: &HouseholdAuthorityHandoffRequest,
) -> Option<ParentControllerLeaseState> {
    request.controller_lease.as_ref().and_then(|lease| {
        (lease.parent_member_id == request.parent_member.member_id
            && lease.device_id == request.parent_controller_device.device_id
            && lease.granted_actions.contains(&request.action))
        .then(|| derived_controller_lease_state(lease, &request.observed_at))
    })
}

fn derived_controller_lease_state(
    lease: &ParentControllerLease,
    observed_at: &ParentPresenceObservedAt,
) -> ParentControllerLeaseState {
    if lease.revocation_state != ParentControllerLeaseState::Active {
        return lease.revocation_state;
    }

    match ParentPresenceObservedAt::from_canonical_utc(&lease.expires_at) {
        Ok(expires_at) if expires_at.is_after(observed_at) => ParentControllerLeaseState::Active,
        Ok(_) | Err(_) => ParentControllerLeaseState::Expired,
    }
}
