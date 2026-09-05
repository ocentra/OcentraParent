#![forbid(unsafe_code)]
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::SchemaVersion;
use ocentra_parent_agent_protocol::activity::policy_preview::{
    PolicyAssistantConfirmationState, PolicyRequestOrigin, PolicyRequestStatus,
};
use serde::{Deserialize, Serialize};

mod approval;
mod decision;
mod lifecycle;
mod origin;
mod primitives;
mod resolution;
mod status;
mod validation;

use crate::policy_source::{
    ParentPolicyActorRole, ParentPolicyDocumentId, PolicyActorId, PolicyAuditReferenceId,
    PolicyChildProfileId, PolicyDeviceId, PolicyHouseholdId, PolicyRuleAction, PolicyRuleId,
    PolicySourceActorState, PolicyTargetKind, PolicyTargetReferenceId, PolicyVersion,
};

const POLICY_REQUEST_SCHEMA_VERSION_VALUE: u16 = 1;

macro_rules! policy_request_text_id {
    ($name:ident) => {
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);
    };
}

policy_request_text_id!(PolicyRequestId);
policy_request_text_id!(PolicyRequestSubmissionKey);
policy_request_text_id!(PolicyApprovalId);
policy_request_text_id!(PolicyOverrideId);
policy_request_text_id!(PolicyAssistantPreviewId);
policy_request_text_id!(PolicyRequestTimestamp);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "u16", into = "u16")]
pub struct PolicyDurationMinutes(u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyRequestKind {
    #[serde(rename = "ask-parent")]
    AskParent,
    #[serde(rename = "bonus-time")]
    BonusTime,
    #[serde(rename = "temporary-override")]
    TemporaryOverride,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyApprovalDecision {
    #[serde(rename = "grant")]
    Grant,
    #[serde(rename = "deny")]
    Deny,
    #[serde(rename = "modify")]
    Modify,
    #[serde(rename = "expire")]
    Expire,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyOverrideState {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "expired")]
    Expired,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyRequestTarget {
    pub kind: PolicyTargetKind,
    pub reference_id: PolicyTargetReferenceId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyRequestScope {
    pub request_kind: PolicyRequestKind,
    pub target: PolicyRequestTarget,
    pub requested_action: PolicyRuleAction,
    pub rule_id: Option<PolicyRuleId>,
    pub requested_bonus_minutes: Option<PolicyDurationMinutes>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChildPolicyRequest {
    pub schema_version: SchemaVersion,
    pub request_id: PolicyRequestId,
    pub submission_key: PolicyRequestSubmissionKey,
    pub household_id: PolicyHouseholdId,
    pub child_profile_id: PolicyChildProfileId,
    pub device_id: Option<PolicyDeviceId>,
    pub source_document_id: ParentPolicyDocumentId,
    pub policy_version: PolicyVersion,
    pub origin: PolicyRequestOrigin,
    pub assistant_preview_id: Option<PolicyAssistantPreviewId>,
    pub assistant_confirmation_state: PolicyAssistantConfirmationState,
    pub status: PolicyRequestStatus,
    pub scope: PolicyRequestScope,
    pub requested_at: PolicyRequestTimestamp,
    pub expires_at: PolicyRequestTimestamp,
    pub audit_reference_ids: Vec<PolicyAuditReferenceId>,
    pub resolved_approval_id: Option<PolicyApprovalId>,
    pub resolved_at: Option<PolicyRequestTimestamp>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistantPolicyRequestConfirmation {
    pub actor_id: PolicyActorId,
    pub actor_role: ParentPolicyActorRole,
    pub actor_state: PolicySourceActorState,
    pub confirmed_at: PolicyRequestTimestamp,
    pub audit_reference_id: PolicyAuditReferenceId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParentPolicyApproval {
    pub approval_id: PolicyApprovalId,
    pub request_id: PolicyRequestId,
    pub household_id: PolicyHouseholdId,
    pub policy_version: PolicyVersion,
    pub actor_id: PolicyActorId,
    pub actor_role: ParentPolicyActorRole,
    pub actor_state: PolicySourceActorState,
    pub decision: PolicyApprovalDecision,
    pub approved_action: Option<PolicyRuleAction>,
    pub approved_bonus_minutes: Option<PolicyDurationMinutes>,
    pub override_expires_at: Option<PolicyRequestTimestamp>,
    pub decided_at: PolicyRequestTimestamp,
    pub audit_reference_id: PolicyAuditReferenceId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyTemporaryOverride {
    pub schema_version: SchemaVersion,
    pub override_id: PolicyOverrideId,
    pub source_request_id: PolicyRequestId,
    pub source_approval_id: PolicyApprovalId,
    pub household_id: PolicyHouseholdId,
    pub child_profile_id: PolicyChildProfileId,
    pub device_id: Option<PolicyDeviceId>,
    pub source_document_id: ParentPolicyDocumentId,
    pub policy_version: PolicyVersion,
    pub request_kind: PolicyRequestKind,
    pub target: PolicyRequestTarget,
    pub approved_action: PolicyRuleAction,
    pub approved_bonus_minutes: Option<PolicyDurationMinutes>,
    pub effective_at: PolicyRequestTimestamp,
    pub expires_at: PolicyRequestTimestamp,
    pub state: PolicyOverrideState,
    pub audit_reference_ids: Vec<PolicyAuditReferenceId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyRequestResolution {
    pub request: ChildPolicyRequest,
    pub temporary_override: Option<PolicyTemporaryOverride>,
}

pub fn policy_request_schema_version() -> Result<SchemaVersion, EventingError> {
    SchemaVersion::new(POLICY_REQUEST_SCHEMA_VERSION_VALUE)
}

pub fn validate_child_policy_request(request: &ChildPolicyRequest) -> Result<(), EventingError> {
    validation::validate_child_policy_request(request)
}

pub fn validate_policy_temporary_override(
    override_record: &PolicyTemporaryOverride,
) -> Result<(), EventingError> {
    validation::validate_policy_temporary_override(override_record)
}

pub fn register_child_policy_request(
    existing: Option<&ChildPolicyRequest>,
    candidate: ChildPolicyRequest,
) -> Result<ChildPolicyRequest, EventingError> {
    lifecycle::register_child_policy_request(existing, candidate)
}

pub fn confirm_assistant_policy_request_preview(
    request: &ChildPolicyRequest,
    confirmation: AssistantPolicyRequestConfirmation,
) -> Result<ChildPolicyRequest, EventingError> {
    lifecycle::confirm_assistant_policy_request_preview(request, confirmation)
}

pub fn expire_child_policy_request(
    request: &ChildPolicyRequest,
    expired_at: PolicyRequestTimestamp,
    audit_reference_id: PolicyAuditReferenceId,
) -> Result<ChildPolicyRequest, EventingError> {
    lifecycle::expire_child_policy_request(request, expired_at, audit_reference_id)
}

pub fn expire_policy_temporary_override(
    override_record: &PolicyTemporaryOverride,
    expired_at: &PolicyRequestTimestamp,
    audit_reference_id: PolicyAuditReferenceId,
) -> Result<PolicyTemporaryOverride, EventingError> {
    lifecycle::expire_policy_temporary_override(override_record, expired_at, audit_reference_id)
}

pub fn resolve_parent_policy_approval(
    request: &ChildPolicyRequest,
    approval: ParentPolicyApproval,
    existing_override: Option<&PolicyTemporaryOverride>,
) -> Result<PolicyRequestResolution, EventingError> {
    resolution::resolve_parent_policy_approval(request, approval, existing_override)
}
