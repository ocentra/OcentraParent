#![forbid(unsafe_code)]

//! Family identity and device-role ownership boundary.
//!
//! This crate owns household membership, child/profile/device role contracts,
//! local authorization decisions, invite/recovery state, and device-ownership
//! checks shared by parent and child runtimes.

use crate::household_authority::{HouseholdAuthorityAction, ParentControllerLeaseState};
use crate::setup_lifecycle::{RecoveryKind, SetupInviteTargetRole};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::expect_value::ExpectValue;
use serde::{Deserialize, Serialize};

pub mod household_authority_handoff;

pub const CRATE_NAME: &str = "ocentra-family-identity-core";
const ERROR_DEVICE_SCOPE_DECISION_ID: &str = "family identity device scope decision id";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HouseholdRole {
    #[serde(rename = "parent-owner")]
    ParentOwner,
    #[serde(rename = "co-parent-guardian")]
    CoParentGuardian,
    #[serde(rename = "observer")]
    Observer,
    #[serde(rename = "child-profile")]
    ChildProfile,
    #[serde(rename = "child-device-agent")]
    ChildDeviceAgent,
    #[serde(rename = "support-admin")]
    SupportAdmin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HouseholdMembershipState {
    #[serde(rename = "invited")]
    Invited,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "revoked")]
    Revoked,
    #[serde(rename = "disabled")]
    Disabled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActorAccountState {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "suspended")]
    Suspended,
    #[serde(rename = "disabled")]
    Disabled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChildProfileBindingState {
    #[serde(rename = "bound")]
    Bound,
    #[serde(rename = "missing")]
    Missing,
    #[serde(rename = "unassigned")]
    Unassigned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeviceOwnershipScope {
    #[serde(rename = "child-profile-device")]
    ChildProfileDevice,
    #[serde(rename = "parent-controller-device")]
    ParentControllerDevice,
    #[serde(rename = "other-device")]
    OtherDevice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChildDisclosureState {
    #[serde(rename = "disclosed")]
    Disclosed,
    #[serde(rename = "not-disclosed")]
    NotDisclosed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeviceScopeAuthorizationState {
    #[serde(rename = "authorized")]
    Authorized,
    #[serde(rename = "rejected")]
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentAuthorityRequirementState {
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "not-required")]
    NotRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeviceTrustState {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "trusted")]
    Trusted,
    #[serde(rename = "revoked")]
    Revoked,
    #[serde(rename = "reset-required")]
    ResetRequired,
    #[serde(rename = "disabled")]
    Disabled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionFreshnessState {
    #[serde(rename = "fresh")]
    Fresh,
    #[serde(rename = "stale")]
    Stale,
    #[serde(rename = "expired")]
    Expired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceScopeInput {
    pub actor_role: HouseholdRole,
    pub same_family: bool,
    pub actor_account_state: ActorAccountState,
    pub membership_state: HouseholdMembershipState,
    pub child_profile_binding_state: ChildProfileBindingState,
    pub device_ownership_scope: DeviceOwnershipScope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceScopeDecision {
    pub authorization_state: DeviceScopeAuthorizationState,
    pub parent_authority_requirement_state: ParentAuthorityRequirementState,
}

family_identity_text_id!(DeviceScopeEvaluationId, "family_identity.evaluation_id");
family_identity_text_id!(DeviceScopeDecisionId, "family_identity.decision_id");
family_identity_text_id!(
    HouseholdAuthorityEvaluationId,
    "family_identity.household_authority_evaluation_id"
);
family_identity_text_id!(FamilyIdentityAggregateId, "family_identity.aggregate_id");
family_identity_text_id!(HouseholdId, "family_identity.household_id");
family_identity_text_id!(ParentMemberId, "family_identity.parent_member_id");
family_identity_text_id!(ChildProfileId, "family_identity.child_profile_id");
family_identity_text_id!(DeviceId, "family_identity.device_id");
family_identity_text_id!(DeviceRouteStateLabel, "family_identity.device_route_state");
family_identity_text_id!(ChildCustodyLabel, "family_identity.child_custody_label");
family_identity_text_id!(
    ParentControllerLeaseId,
    "family_identity.parent_controller_lease_id"
);
family_identity_text_id!(
    ObserverPermissionId,
    "family_identity.observer_permission_id"
);
family_identity_text_id!(SetupInviteId, "family_identity.setup_invite_id");
family_identity_text_id!(RecoveryId, "family_identity.recovery_id");
family_identity_text_id!(SetupAuditEventId, "family_identity.setup_audit_event_id");
family_identity_text_id!(SetupAuditActionId, "family_identity.setup_audit_action_id");
family_identity_text_id!(SetupAuditTargetId, "family_identity.setup_audit_target_id");
family_identity_text_id!(
    SetupAuditEvidenceRef,
    "family_identity.setup_audit_evidence_ref"
);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HouseholdProfile {
    pub household_id: HouseholdId,
    pub display_name: String,
    pub created_at: String,
    pub parent_member_ids: Vec<ParentMemberId>,
    pub child_profile_ids: Vec<ChildProfileId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParentMember {
    pub member_id: ParentMemberId,
    pub household_id: HouseholdId,
    pub role: HouseholdRole,
    pub invite_state: HouseholdMembershipState,
    pub joined_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChildProfile {
    pub child_id: ChildProfileId,
    pub household_id: HouseholdId,
    pub display_name: String,
    pub device_ids: Vec<DeviceId>,
    pub custody_label: ChildCustodyLabel,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceRegistration {
    pub device_id: DeviceId,
    pub child_id: ChildProfileId,
    pub household_id: HouseholdId,
    pub trust_state: DeviceTrustState,
    pub role_label: HouseholdRole,
    pub route_state: DeviceRouteStateLabel,
    pub stale_since: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParentControllerLease {
    pub lease_id: ParentControllerLeaseId,
    pub parent_member_id: ParentMemberId,
    pub device_id: DeviceId,
    pub issued_at: String,
    pub expires_at: String,
    pub revocation_state: ParentControllerLeaseState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObserverPermission {
    pub perm_id: ObserverPermissionId,
    pub parent_member_id: ParentMemberId,
    pub household_id: HouseholdId,
    pub granted_scopes: Vec<HouseholdAuthorityAction>,
    pub is_write_blocked: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SetupInvite {
    pub invite_id: SetupInviteId,
    pub household_id: HouseholdId,
    pub invitee_email: String,
    pub role: SetupInviteTargetRole,
    pub expires_at: String,
}

/// WP21's record-like recovery contract.
///
/// This stays distinct from `setup_lifecycle::RecoveryState`, which is the
/// workflow-status enum used by the recovery evaluator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryState {
    pub recovery_id: RecoveryId,
    pub device_id: DeviceId,
    pub reason: RecoveryKind,
    pub parent_action_required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SetupAuditEvent {
    pub event_id: SetupAuditEventId,
    pub household_id: HouseholdId,
    pub actor_member_id: ParentMemberId,
    pub target_id: SetupAuditTargetId,
    pub action: SetupAuditActionId,
    pub timestamp: String,
    pub evidence_ref: Option<SetupAuditEvidenceRef>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceScopeEvaluationRequestedEvent {
    pub aggregate_id: FamilyIdentityAggregateId,
    pub evaluation_id: DeviceScopeEvaluationId,
    pub input: DeviceScopeInput,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceScopeDecisionRecordedEvent {
    pub aggregate_id: FamilyIdentityAggregateId,
    pub decision_id: DeviceScopeDecisionId,
    pub source_evaluation_id: DeviceScopeEvaluationId,
    pub decision: DeviceScopeDecision,
}

pub fn authorize_child_device_scope(input: DeviceScopeInput) -> DeviceScopeDecision {
    let allowed = crate::family_identity_helpers::device_scope_is_allowed(&input);

    DeviceScopeDecision {
        authorization_state: crate::family_identity_helpers::device_scope_authorization_state(
            allowed,
        ),
        parent_authority_requirement_state:
            crate::family_identity_helpers::parent_authority_requirement_state(allowed),
    }
}

pub fn record_device_scope_decision(
    event: &DeviceScopeEvaluationRequestedEvent,
) -> DeviceScopeDecisionRecordedEvent {
    DeviceScopeDecisionRecordedEvent {
        aggregate_id: event.aggregate_id.clone(),
        decision_id: DeviceScopeDecisionId::parse(
            crate::family_identity_helpers::device_scope_decision_ref(&event.evaluation_id),
        )
        .expect_value(ERROR_DEVICE_SCOPE_DECISION_ID),
        source_evaluation_id: event.evaluation_id.clone(),
        decision: authorize_child_device_scope(event.input),
    }
}
