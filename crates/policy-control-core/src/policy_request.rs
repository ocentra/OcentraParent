#![forbid(unsafe_code)]
#![allow(clippy::needless_pass_by_value)]

use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::SchemaVersion;
use ocentra_parent_agent_protocol::constants::policy_control;
use serde::{Deserialize, Serialize};

use crate::policy_source::{
    ParentPolicyActorRole, ParentPolicyDocumentId, PolicyActorId, PolicyAuditReferenceId,
    PolicyChildProfileId, PolicyDeviceId, PolicyHouseholdId, PolicyRuleAction, PolicyRuleId,
    PolicySourceActorState, PolicyTargetKind, PolicyTargetReferenceId, PolicyVersion,
};

const POLICY_REQUEST_SCHEMA_VERSION_VALUE: u16 = 1;

macro_rules! policy_request_text_id {
    ($name:ident, $field:expr) => {
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);

        impl $name {
            pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
                let value = value.into();
                if value.trim().is_empty() {
                    return Err(EventingError::EmptyValue { field: $field });
                }
                Ok(Self(value))
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl TryFrom<String> for $name {
            type Error = EventingError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::parse(value)
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }
    };
}

policy_request_text_id!(PolicyRequestId, policy_control::request::FIELD_REQUEST_ID);
policy_request_text_id!(
    PolicyRequestSubmissionKey,
    policy_control::request::FIELD_SUBMISSION_KEY
);
policy_request_text_id!(PolicyApprovalId, policy_control::request::FIELD_APPROVAL_ID);
policy_request_text_id!(PolicyOverrideId, policy_control::request::FIELD_OVERRIDE_ID);
policy_request_text_id!(
    PolicyAssistantPreviewId,
    policy_control::request::FIELD_ASSISTANT_PREVIEW_ID
);
policy_request_text_id!(
    PolicyRequestTimestamp,
    policy_control::request::FIELD_TIMESTAMP
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "u16", into = "u16")]
pub struct PolicyDurationMinutes(u16);

impl PolicyDurationMinutes {
    pub fn new(value: u16) -> Result<Self, EventingError> {
        if value == 0 {
            return Err(EventingError::InvalidValue {
                field: policy_control::request::FIELD_DURATION_MINUTES,
                value: value.to_string(),
            });
        }
        Ok(Self(value))
    }

    pub fn value(self) -> u16 {
        self.0
    }
}

impl TryFrom<u16> for PolicyDurationMinutes {
    type Error = EventingError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<PolicyDurationMinutes> for u16 {
    fn from(value: PolicyDurationMinutes) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyRequestOrigin {
    #[serde(rename = "child")]
    Child,
    #[serde(rename = "assistant-draft")]
    AssistantDraft,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyAssistantConfirmationState {
    #[serde(rename = "not-required")]
    NotRequired,
    #[serde(rename = "parent-confirmation-required")]
    ParentConfirmationRequired,
    #[serde(rename = "parent-confirmed")]
    ParentConfirmed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyRequestStatus {
    #[serde(rename = "preview-only")]
    PreviewOnly,
    #[serde(rename = "pending-parent-review")]
    PendingParentReview,
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "denied")]
    Denied,
    #[serde(rename = "modified")]
    Modified,
    #[serde(rename = "expired")]
    Expired,
}

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
    assert_non_empty_audit_refs(
        &request.audit_reference_ids,
        policy_control::request::FIELD_AUDIT_REFERENCE_IDS,
    )?;
    assert_request_scope(&request.scope)?;
    assert_request_origin_shape(request)?;
    assert_request_resolution_shape(request)?;
    Ok(())
}

pub fn register_child_policy_request(
    existing: Option<&ChildPolicyRequest>,
    candidate: ChildPolicyRequest,
) -> Result<ChildPolicyRequest, EventingError> {
    validate_child_policy_request(&candidate)?;
    if !matches!(
        candidate.status,
        PolicyRequestStatus::PreviewOnly | PolicyRequestStatus::PendingParentReview
    ) || candidate.resolved_approval_id.is_some()
        || candidate.resolved_at.is_some()
    {
        return Err(EventingError::InvalidValue {
            field: policy_control::request::FIELD_STATUS,
            value: policy_request_status_name(candidate.status).to_string(),
        });
    }

    if let Some(current) = existing {
        if current.request_id == candidate.request_id
            && current.submission_key != candidate.submission_key
        {
            return Err(EventingError::InvalidValue {
                field: policy_control::request::FIELD_SUBMISSION_KEY,
                value: current.submission_key.as_str().to_string(),
            });
        }

        if current.submission_key == candidate.submission_key {
            if child_requests_match(current, &candidate) {
                return Ok(current.clone());
            }

            return Err(EventingError::InvalidValue {
                field: policy_control::request::FIELD_SUBMISSION_KEY,
                value: duplicate_submission_key_value(&current.submission_key),
            });
        }
    }

    Ok(candidate)
}

pub fn confirm_assistant_policy_request_preview(
    request: &ChildPolicyRequest,
    confirmation: AssistantPolicyRequestConfirmation,
) -> Result<ChildPolicyRequest, EventingError> {
    validate_child_policy_request(request)?;
    assert_parent_actor_authority(
        confirmation.actor_role,
        confirmation.actor_state,
        policy_control::request::FIELD_ACTOR_ROLE,
        policy_control::request::FIELD_ACTOR_STATE,
    )?;
    if request.origin != PolicyRequestOrigin::AssistantDraft
        || request.status != PolicyRequestStatus::PreviewOnly
        || request.assistant_confirmation_state
            != PolicyAssistantConfirmationState::ParentConfirmationRequired
    {
        return Err(EventingError::InvalidValue {
            field: policy_control::request::FIELD_ASSISTANT_CONFIRMATION_STATE,
            value: policy_request_status_name(request.status).to_string(),
        });
    }

    let mut confirmed = request.clone();
    confirmed.status = PolicyRequestStatus::PendingParentReview;
    confirmed.assistant_confirmation_state = PolicyAssistantConfirmationState::ParentConfirmed;
    confirmed
        .audit_reference_ids
        .push(confirmation.audit_reference_id);
    validate_child_policy_request(&confirmed)?;
    Ok(confirmed)
}

pub fn expire_child_policy_request(
    request: &ChildPolicyRequest,
    expired_at: PolicyRequestTimestamp,
    audit_reference_id: PolicyAuditReferenceId,
) -> Result<ChildPolicyRequest, EventingError> {
    validate_child_policy_request(request)?;
    if matches!(
        request.status,
        PolicyRequestStatus::Approved | PolicyRequestStatus::Denied | PolicyRequestStatus::Modified
    ) {
        return Err(EventingError::InvalidValue {
            field: policy_control::request::FIELD_STATUS,
            value: policy_request_status_name(request.status).to_string(),
        });
    }
    if request.status == PolicyRequestStatus::Expired {
        return Ok(request.clone());
    }

    let mut expired = request.clone();
    expired.status = PolicyRequestStatus::Expired;
    expired.resolved_at = Some(expired_at);
    expired.audit_reference_ids.push(audit_reference_id);
    validate_child_policy_request(&expired)?;
    Ok(expired)
}

pub fn resolve_parent_policy_approval(
    request: &ChildPolicyRequest,
    approval: ParentPolicyApproval,
    existing_override: Option<&PolicyTemporaryOverride>,
) -> Result<PolicyRequestResolution, EventingError> {
    validate_child_policy_request(request)?;
    validate_parent_policy_approval(&approval)?;
    assert_request_matches_approval(request, &approval)?;

    if request.assistant_confirmation_state
        == PolicyAssistantConfirmationState::ParentConfirmationRequired
    {
        return Err(EventingError::InvalidValue {
            field: policy_control::request::FIELD_ASSISTANT_CONFIRMATION_STATE,
            value: policy_control::request::VALUE_ASSISTANT_PREVIEW_ONLY.to_string(),
        });
    }

    if let Some(resolved_approval_id) = &request.resolved_approval_id {
        if resolved_approval_id == &approval.approval_id
            && request.status == policy_request_status_for_approval(approval.decision)
        {
            let replay_override = match request.status {
                PolicyRequestStatus::Approved | PolicyRequestStatus::Modified => {
                    let replay_override =
                        existing_override.ok_or_else(|| EventingError::InvalidValue {
                            field: policy_control::request::FIELD_OVERRIDE_ID,
                            value: policy_control::request::VALUE_MISSING_OVERRIDE_FOR_RESOLVED_APPROVAL_REPLAY
                                .to_string(),
                        })?;
                    assert_override_matches(request, &approval, replay_override)?;
                    Some(replay_override.clone())
                }
                _ => None,
            };

            return Ok(PolicyRequestResolution {
                request: request.clone(),
                temporary_override: replay_override,
            });
        }

        return Err(EventingError::InvalidValue {
            field: policy_control::request::FIELD_APPROVAL_ID,
            value: approval.approval_id.as_str().to_string(),
        });
    }

    if request.status != PolicyRequestStatus::PendingParentReview {
        return Err(EventingError::InvalidValue {
            field: policy_control::request::FIELD_STATUS,
            value: policy_request_status_name(request.status).to_string(),
        });
    }

    let mut resolved_request = request.clone();
    resolved_request.status = policy_request_status_for_approval(approval.decision);
    resolved_request.resolved_approval_id = Some(approval.approval_id.clone());
    resolved_request.resolved_at = Some(approval.decided_at.clone());
    resolved_request
        .audit_reference_ids
        .push(approval.audit_reference_id.clone());

    let temporary_override = match approval.decision {
        PolicyApprovalDecision::Grant | PolicyApprovalDecision::Modify => {
            Some(build_policy_temporary_override(request, &approval)?)
        }
        PolicyApprovalDecision::Deny | PolicyApprovalDecision::Expire => None,
    };

    validate_child_policy_request(&resolved_request)?;
    Ok(PolicyRequestResolution {
        request: resolved_request,
        temporary_override,
    })
}

fn validate_parent_policy_approval(approval: &ParentPolicyApproval) -> Result<(), EventingError> {
    assert_parent_actor_authority(
        approval.actor_role,
        approval.actor_state,
        policy_control::request::FIELD_ACTOR_ROLE,
        policy_control::request::FIELD_ACTOR_STATE,
    )?;
    match approval.decision {
        PolicyApprovalDecision::Deny | PolicyApprovalDecision::Expire => {
            if approval.approved_action.is_some()
                || approval.approved_bonus_minutes.is_some()
                || approval.override_expires_at.is_some()
            {
                return Err(EventingError::InvalidValue {
                    field: policy_control::request::FIELD_APPROVAL_DECISION,
                    value:
                        policy_control::request::VALUE_DENY_OR_EXPIRE_CANNOT_CARRY_OVERRIDE_VALUES
                            .to_string(),
                });
            }
        }
        PolicyApprovalDecision::Modify => {
            if approval.approved_action.is_none()
                && approval.approved_bonus_minutes.is_none()
                && approval.override_expires_at.is_none()
            {
                return Err(EventingError::InvalidValue {
                    field: policy_control::request::FIELD_APPROVAL_DECISION,
                    value: policy_control::request::VALUE_MODIFY_REQUIRES_CHANGED_OVERRIDE_VALUES
                        .to_string(),
                });
            }
        }
        PolicyApprovalDecision::Grant => {}
    }
    Ok(())
}

fn build_policy_temporary_override(
    request: &ChildPolicyRequest,
    approval: &ParentPolicyApproval,
) -> Result<PolicyTemporaryOverride, EventingError> {
    let approved_action = approval
        .approved_action
        .unwrap_or(request.scope.requested_action);
    let approved_bonus_minutes = approval
        .approved_bonus_minutes
        .or(request.scope.requested_bonus_minutes);

    if request.scope.request_kind == PolicyRequestKind::BonusTime
        && approved_bonus_minutes.is_none()
    {
        return Err(EventingError::InvalidValue {
            field: policy_control::request::FIELD_APPROVED_BONUS_MINUTES,
            value: policy_control::request::VALUE_BONUS_TIME_APPROVAL_REQUIRES_MINUTES.to_string(),
        });
    }

    Ok(PolicyTemporaryOverride {
        schema_version: policy_request_schema_version()?,
        override_id: PolicyOverrideId::parse(policy_override_id_value(&approval.approval_id))?,
        source_request_id: request.request_id.clone(),
        source_approval_id: approval.approval_id.clone(),
        household_id: request.household_id.clone(),
        child_profile_id: request.child_profile_id.clone(),
        device_id: request.device_id.clone(),
        source_document_id: request.source_document_id.clone(),
        policy_version: request.policy_version,
        request_kind: request.scope.request_kind,
        target: request.scope.target.clone(),
        approved_action,
        approved_bonus_minutes,
        effective_at: approval.decided_at.clone(),
        expires_at: approval
            .override_expires_at
            .clone()
            .unwrap_or_else(|| request.expires_at.clone()),
        state: PolicyOverrideState::Active,
        audit_reference_ids: vec![approval.audit_reference_id.clone()],
    })
}

fn assert_request_scope(scope: &PolicyRequestScope) -> Result<(), EventingError> {
    if scope.request_kind == PolicyRequestKind::BonusTime && scope.requested_bonus_minutes.is_none()
    {
        return Err(EventingError::InvalidValue {
            field: policy_control::request::FIELD_REQUESTED_BONUS_MINUTES,
            value: policy_control::request::VALUE_BONUS_TIME_REQUEST_REQUIRES_MINUTES.to_string(),
        });
    }
    Ok(())
}

fn assert_request_origin_shape(request: &ChildPolicyRequest) -> Result<(), EventingError> {
    match request.origin {
        PolicyRequestOrigin::Child => {
            if request.assistant_preview_id.is_some()
                || request.assistant_confirmation_state
                    != PolicyAssistantConfirmationState::NotRequired
                || request.status == PolicyRequestStatus::PreviewOnly
            {
                return Err(EventingError::InvalidValue {
                    field: policy_control::request::FIELD_ORIGIN,
                    value: policy_control::request::VALUE_CHILD_REQUEST_CANNOT_BE_ASSISTANT_PREVIEW
                        .to_string(),
                });
            }
        }
        PolicyRequestOrigin::AssistantDraft => {
            let invalid_confirmation_shape = match request.assistant_confirmation_state {
                PolicyAssistantConfirmationState::NotRequired => true,
                PolicyAssistantConfirmationState::ParentConfirmationRequired => !matches!(
                    request.status,
                    PolicyRequestStatus::PreviewOnly | PolicyRequestStatus::Expired
                ),
                PolicyAssistantConfirmationState::ParentConfirmed => {
                    request.status == PolicyRequestStatus::PreviewOnly
                }
            };

            if request.assistant_preview_id.is_none() || invalid_confirmation_shape {
                return Err(EventingError::InvalidValue {
                    field: policy_control::request::FIELD_ASSISTANT_PREVIEW_ID,
                    value: policy_control::request::VALUE_ASSISTANT_DRAFT_REQUEST_MUST_STAY_PREVIEW_ONLY_UNTIL_PARENT_CONFIRMED
                        .to_string(),
                });
            }
        }
    }
    Ok(())
}

fn assert_request_resolution_shape(request: &ChildPolicyRequest) -> Result<(), EventingError> {
    let resolved = request.resolved_approval_id.is_some() || request.resolved_at.is_some();
    if matches!(
        request.status,
        PolicyRequestStatus::Approved | PolicyRequestStatus::Denied | PolicyRequestStatus::Modified
    ) && (request.resolved_approval_id.is_none() || request.resolved_at.is_none())
    {
        return Err(EventingError::InvalidValue {
            field: policy_control::request::FIELD_RESOLVED_APPROVAL_ID,
            value: policy_request_status_name(request.status).to_string(),
        });
    }
    if request.status == PolicyRequestStatus::PreviewOnly && resolved {
        return Err(EventingError::InvalidValue {
            field: policy_control::request::FIELD_STATUS,
            value: policy_control::request::VALUE_PREVIEW_ONLY_REQUEST_CANNOT_BE_RESOLVED
                .to_string(),
        });
    }
    Ok(())
}

fn assert_request_matches_approval(
    request: &ChildPolicyRequest,
    approval: &ParentPolicyApproval,
) -> Result<(), EventingError> {
    if request.request_id != approval.request_id {
        return Err(EventingError::InvalidValue {
            field: policy_control::request::FIELD_REQUEST_ID,
            value: approval.request_id.as_str().to_string(),
        });
    }
    if request.household_id != approval.household_id {
        return Err(EventingError::InvalidValue {
            field: policy_control::request::FIELD_HOUSEHOLD_ID,
            value: approval.household_id.as_str().to_string(),
        });
    }
    if request.policy_version != approval.policy_version {
        return Err(EventingError::InvalidValue {
            field: policy_control::request::FIELD_POLICY_VERSION,
            value: approval.policy_version.value().to_string(),
        });
    }
    if request.status == PolicyRequestStatus::Expired {
        return Err(EventingError::InvalidValue {
            field: policy_control::request::FIELD_STATUS,
            value: policy_control::request::VALUE_EXPIRED_REQUEST_CANNOT_BE_APPROVED.to_string(),
        });
    }
    Ok(())
}

fn assert_override_matches(
    request: &ChildPolicyRequest,
    approval: &ParentPolicyApproval,
    existing_override: &PolicyTemporaryOverride,
) -> Result<(), EventingError> {
    let expected_action = approval
        .approved_action
        .unwrap_or(request.scope.requested_action);
    let expected_minutes = approval
        .approved_bonus_minutes
        .or(request.scope.requested_bonus_minutes);
    let expected_expires_at = approval
        .override_expires_at
        .as_ref()
        .unwrap_or(&request.expires_at);

    if existing_override.source_request_id != request.request_id
        || existing_override.source_approval_id != approval.approval_id
        || existing_override.policy_version != request.policy_version
        || existing_override.approved_action != expected_action
        || existing_override.approved_bonus_minutes != expected_minutes
        || &existing_override.expires_at != expected_expires_at
    {
        return Err(EventingError::InvalidValue {
            field: policy_control::request::FIELD_OVERRIDE_ID,
            value: existing_override.override_id.as_str().to_string(),
        });
    }

    Ok(())
}

fn assert_parent_actor_authority(
    role: ParentPolicyActorRole,
    state: PolicySourceActorState,
    role_field: &'static str,
    state_field: &'static str,
) -> Result<(), EventingError> {
    if !matches!(
        role,
        ParentPolicyActorRole::Parent | ParentPolicyActorRole::CoParent
    ) {
        return Err(EventingError::InvalidValue {
            field: role_field,
            value: policy_actor_role_name(role).to_string(),
        });
    }

    if state != PolicySourceActorState::Active {
        return Err(EventingError::InvalidValue {
            field: state_field,
            value: policy_actor_state_name(state).to_string(),
        });
    }

    Ok(())
}

fn policy_actor_role_name(role: ParentPolicyActorRole) -> &'static str {
    match role {
        ParentPolicyActorRole::Parent => policy_control::source::ROLE_PARENT,
        ParentPolicyActorRole::CoParent => policy_control::source::ROLE_CO_PARENT,
        ParentPolicyActorRole::Observer => policy_control::source::ROLE_OBSERVER,
        ParentPolicyActorRole::Child => policy_control::source::ROLE_CHILD,
        ParentPolicyActorRole::Support => policy_control::source::ROLE_SUPPORT,
    }
}

fn policy_actor_state_name(state: PolicySourceActorState) -> &'static str {
    match state {
        PolicySourceActorState::Active => policy_control::source::ACTOR_STATE_ACTIVE,
        PolicySourceActorState::Revoked => policy_control::source::ACTOR_STATE_REVOKED,
    }
}

fn assert_non_empty_audit_refs(
    audit_reference_ids: &[PolicyAuditReferenceId],
    field: &'static str,
) -> Result<(), EventingError> {
    if audit_reference_ids.is_empty() {
        return Err(EventingError::InvalidValue {
            field,
            value: policy_control::request::VALUE_MISSING_AUDIT_REFERENCE.to_string(),
        });
    }
    Ok(())
}

fn child_requests_match(left: &ChildPolicyRequest, right: &ChildPolicyRequest) -> bool {
    left.household_id == right.household_id
        && left.child_profile_id == right.child_profile_id
        && left.device_id == right.device_id
        && left.source_document_id == right.source_document_id
        && left.policy_version == right.policy_version
        && left.origin == right.origin
        && left.assistant_preview_id == right.assistant_preview_id
        && left.assistant_confirmation_state == right.assistant_confirmation_state
        && left.status == right.status
        && left.scope == right.scope
        && left.requested_at == right.requested_at
        && left.expires_at == right.expires_at
}

fn policy_request_status_for_approval(decision: PolicyApprovalDecision) -> PolicyRequestStatus {
    match decision {
        PolicyApprovalDecision::Grant => PolicyRequestStatus::Approved,
        PolicyApprovalDecision::Deny => PolicyRequestStatus::Denied,
        PolicyApprovalDecision::Modify => PolicyRequestStatus::Modified,
        PolicyApprovalDecision::Expire => PolicyRequestStatus::Expired,
    }
}

fn policy_request_status_name(status: PolicyRequestStatus) -> &'static str {
    match status {
        PolicyRequestStatus::PreviewOnly => policy_control::request::STATUS_PREVIEW_ONLY,
        PolicyRequestStatus::PendingParentReview => {
            policy_control::request::STATUS_PENDING_PARENT_REVIEW
        }
        PolicyRequestStatus::Approved => policy_control::request::STATUS_APPROVED,
        PolicyRequestStatus::Denied => policy_control::request::STATUS_DENIED,
        PolicyRequestStatus::Modified => policy_control::request::STATUS_MODIFIED,
        PolicyRequestStatus::Expired => policy_control::request::STATUS_EXPIRED,
    }
}

fn duplicate_submission_key_value(submission_key: &PolicyRequestSubmissionKey) -> String {
    let mut value = String::from(policy_control::request::VALUE_DUPLICATE_SUBMISSION_KEY_PREFIX);
    value.push_str(submission_key.as_str());
    value.push_str(policy_control::request::VALUE_DUPLICATE_SUBMISSION_KEY_SUFFIX);
    value
}

fn policy_override_id_value(approval_id: &PolicyApprovalId) -> String {
    let mut value = String::from(policy_control::request::OVERRIDE_ID_PREFIX);
    value.push_str(approval_id.as_str());
    value
}
