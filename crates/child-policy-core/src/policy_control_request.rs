use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::activity::policy_preview::{
    PolicyAssistantConfirmationState, PolicyRequestOrigin, PolicyRequestStatus,
};
use ocentra_parent_agent_protocol::child_domain_runtime::{
    ChildDomainEventType, ChildDomainPolicyViolationDetectedEvent,
};
use ocentra_policy_control_core::policy_request::{
    policy_request_schema_version, validate_child_policy_request, ChildPolicyRequest,
    PolicyAssistantPreviewId, PolicyDurationMinutes, PolicyRequestId, PolicyRequestScope,
    PolicyRequestSubmissionKey, PolicyRequestTarget, PolicyRequestTimestamp,
};
use ocentra_policy_control_core::policy_source::{
    ParentPolicyDocumentId, PolicyAuditReferenceId, PolicyChildProfileId, PolicyDeviceId,
    PolicyHouseholdId, PolicyRuleAction, PolicyRuleId, PolicyVersion,
};

const POLICY_CONTROL_REQUEST_ID_PREFIX: &str = "policy-control-request:";
const POLICY_CONTROL_SUBMISSION_KEY_PREFIX: &str = "policy-control-submission:";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChildPolicyControlRequestInput {
    pub household_id: PolicyHouseholdId,
    pub source_document_id: ParentPolicyDocumentId,
    pub policy_version: PolicyVersion,
    pub request_kind: ocentra_policy_control_core::policy_request::PolicyRequestKind,
    pub target: PolicyRequestTarget,
    pub requested_action: PolicyRuleAction,
    pub rule_id: Option<PolicyRuleId>,
    pub requested_bonus_minutes: Option<PolicyDurationMinutes>,
    pub origin: PolicyRequestOrigin,
    pub assistant_preview_id: Option<PolicyAssistantPreviewId>,
    pub expires_at: PolicyRequestTimestamp,
    pub audit_reference_ids: Vec<PolicyAuditReferenceId>,
}

pub fn build_policy_control_request_from_child_violation(
    violation: &ChildDomainPolicyViolationDetectedEvent,
    input: ChildPolicyControlRequestInput,
) -> Result<ChildPolicyRequest, EventingError> {
    validate_child_policy_violation(violation)?;
    let (assistant_confirmation_state, status) = request_origin_metadata(input.origin);

    let request = ChildPolicyRequest {
        schema_version: policy_request_schema_version()?,
        request_id: policy_request_id(violation)?,
        submission_key: submission_key(
            violation,
            input.origin.as_protocol_str(),
            input.assistant_preview_id.as_ref(),
        )?,
        household_id: input.household_id,
        child_profile_id: PolicyChildProfileId::parse(violation.child_profile_id.as_str())?,
        device_id: Some(PolicyDeviceId::parse(violation.child_device_id.as_str())?),
        source_document_id: input.source_document_id,
        policy_version: input.policy_version,
        origin: input.origin,
        assistant_preview_id: input.assistant_preview_id,
        assistant_confirmation_state,
        status,
        scope: PolicyRequestScope {
            request_kind: input.request_kind,
            target: input.target,
            requested_action: input.requested_action,
            rule_id: input.rule_id,
            requested_bonus_minutes: input.requested_bonus_minutes,
        },
        requested_at: PolicyRequestTimestamp::parse(violation.detected_at.as_str())?,
        expires_at: input.expires_at,
        audit_reference_ids: canonical_audit_reference_ids(input.audit_reference_ids),
        resolved_approval_id: None,
        resolved_at: None,
    };

    validate_child_policy_request(&request)?;
    Ok(request)
}

fn validate_child_policy_violation(
    violation: &ChildDomainPolicyViolationDetectedEvent,
) -> Result<(), EventingError> {
    if violation.event_type != ChildDomainEventType::policy_violation_detected() {
        return Err(EventingError::InvalidValue {
            field: "policy_control_request.event_type",
            value: violation.event_type.as_str().to_string(),
        });
    }

    if violation.evidence_refs.is_empty() {
        return Err(EventingError::InvalidValue {
            field: "policy_control_request.evidence_refs",
            value: String::from("empty"),
        });
    }

    Ok(())
}

fn policy_request_id(
    violation: &ChildDomainPolicyViolationDetectedEvent,
) -> Result<PolicyRequestId, EventingError> {
    PolicyRequestId::parse(format!(
        "{}{}",
        POLICY_CONTROL_REQUEST_ID_PREFIX,
        violation.violation_id.as_str()
    ))
}

fn submission_key(
    violation: &ChildDomainPolicyViolationDetectedEvent,
    origin_name: &'static str,
    assistant_preview_id: Option<&PolicyAssistantPreviewId>,
) -> Result<PolicyRequestSubmissionKey, EventingError> {
    let mut value = format!(
        "{}{}:{}",
        POLICY_CONTROL_SUBMISSION_KEY_PREFIX,
        origin_name,
        violation.violation_id.as_str()
    );

    if let Some(assistant_preview_id) = assistant_preview_id {
        value.push(':');
        value.push_str(assistant_preview_id.as_str());
    }

    PolicyRequestSubmissionKey::parse(value)
}

fn request_origin_metadata(
    origin: PolicyRequestOrigin,
) -> (PolicyAssistantConfirmationState, PolicyRequestStatus) {
    match origin {
        PolicyRequestOrigin::Child => (
            PolicyAssistantConfirmationState::NotRequired,
            PolicyRequestStatus::PendingParentReview,
        ),
        PolicyRequestOrigin::AssistantDraft => (
            PolicyAssistantConfirmationState::ParentConfirmationRequired,
            PolicyRequestStatus::PreviewOnly,
        ),
    }
}

fn canonical_audit_reference_ids(
    audit_reference_ids: Vec<PolicyAuditReferenceId>,
) -> Vec<PolicyAuditReferenceId> {
    let mut canonical = Vec::with_capacity(audit_reference_ids.len());
    for audit_reference_id in audit_reference_ids {
        if !canonical.contains(&audit_reference_id) {
            canonical.push(audit_reference_id);
        }
    }
    canonical
}
