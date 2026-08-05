//! Versioned, identifier-only account-authority handoff for downstream persistence owners.
//!
//! The handoff derives family scope from canonical family records before it delegates to the
//! household authority evaluator. It intentionally carries neither display names nor secrets.

use super::{
    ActorAccountState, ChildProfile, ChildProfileBindingState, DeviceOwnershipScope,
    DeviceRegistration, HouseholdAuthorityEvaluationId, HouseholdId, ParentControllerLease,
    ParentMember, SessionFreshnessState,
};
use crate::household_authority::{
    authorize_household_action, HouseholdAuthorityAction, HouseholdAuthorityDecision,
    HouseholdAuthorityInput, ParentControllerLeaseState,
};
use serde::{Deserialize, Serialize};

pub const HOUSEHOLD_AUTHORITY_HANDOFF_SCHEMA_VERSION: u16 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HouseholdAuthorityHandoffRedactionState {
    #[serde(rename = "identifiers-only")]
    IdentifiersOnly,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HouseholdAuthorityHandoffRequest {
    pub evaluation_id: HouseholdAuthorityEvaluationId,
    pub parent_member: ParentMember,
    pub child_profile: ChildProfile,
    pub device_registration: DeviceRegistration,
    pub actor_account_state: ActorAccountState,
    pub session_freshness_state: SessionFreshnessState,
    pub capability_granted: bool,
    pub controller_lease: Option<ParentControllerLease>,
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
        device_trust_state: request.device_registration.trust_state,
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
        child_profile_id: request.child_profile.child_id,
        device_id: request.device_registration.device_id,
        action: request.action,
        decision,
        redaction_state: HouseholdAuthorityHandoffRedactionState::IdentifiersOnly,
    }
}

fn same_family(request: &HouseholdAuthorityHandoffRequest) -> bool {
    request.parent_member.household_id == request.child_profile.household_id
        && request.parent_member.household_id == request.device_registration.household_id
}

fn child_profile_binding_state(
    request: &HouseholdAuthorityHandoffRequest,
) -> ChildProfileBindingState {
    if request
        .device_registration
        .validate_child_profile(&request.child_profile)
        .is_ok()
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

fn controller_lease_state(
    request: &HouseholdAuthorityHandoffRequest,
) -> Option<ParentControllerLeaseState> {
    request.controller_lease.as_ref().and_then(|lease| {
        (lease.parent_member_id == request.parent_member.member_id
            && lease.device_id == request.device_registration.device_id)
            .then_some(lease.revocation_state)
    })
}
