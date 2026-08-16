use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::activity::policy_preview::{
    PolicyAssistantConfirmationState, PolicyRequestOrigin, PolicyRequestStatus,
};
use ocentra_policy_control_core::policy_delivery::{
    apply_policy_delivery_transition, queue_policy_delivery, PolicyDeliveryAttemptId,
    PolicyDeliveryId, PolicyDeliveryRecord, PolicyDeliverySequence, PolicyDeliveryState,
    PolicyDeliveryTarget, PolicyDeliveryTransition,
};
use ocentra_policy_control_core::policy_request::{
    policy_request_schema_version, ChildPolicyRequest, PolicyApprovalId, PolicyDurationMinutes,
    PolicyOverrideId, PolicyOverrideState, PolicyRequestId, PolicyRequestKind, PolicyRequestScope,
    PolicyRequestSubmissionKey, PolicyRequestTarget, PolicyRequestTimestamp,
    PolicyTemporaryOverride,
};
use ocentra_policy_control_core::policy_source::{
    CompiledDomainPolicyArtifact, ParentPolicyDocumentId, PolicyAuditReferenceId,
    PolicyChildProfileId, PolicyConsumerDomain, PolicyDeviceId, PolicyHouseholdId,
    PolicyReasonCode, PolicyRuleAction, PolicyRuleId, PolicyTargetKind, PolicyTargetReferenceId,
    PolicyVersion,
};

#[derive(Clone, Debug)]
struct NotificationText(String);

macro_rules! notification_text {
    ($value:expr) => {
        NotificationText($value.to_string())
    };
}

fn timestamp(value: &NotificationText) -> PolicyRequestTimestamp {
    PolicyRequestTimestamp::parse(&value.0).expect_value("policy request timestamp")
}

fn audit_ref(value: &NotificationText) -> PolicyAuditReferenceId {
    PolicyAuditReferenceId::parse(&value.0).expect_value("policy audit ref")
}

fn request_scope(kind: PolicyRequestKind, minutes: Option<u16>) -> PolicyRequestScope {
    PolicyRequestScope {
        request_kind: kind,
        target: PolicyRequestTarget {
            kind: PolicyTargetKind::Category,
            reference_id: PolicyTargetReferenceId::parse("category-gaming")
                .expect_value("policy target ref"),
        },
        requested_action: PolicyRuleAction::TimeLimit,
        rule_id: Some(PolicyRuleId::parse("rule-school-night").expect_value("policy rule id")),
        requested_bonus_minutes: minutes
            .map(PolicyDurationMinutes::new)
            .transpose()
            .expect_value("policy duration"),
    }
}

fn base_request() -> ChildPolicyRequest {
    ChildPolicyRequest {
        schema_version: policy_request_schema_version()
            .expect_value("policy request schema version"),
        request_id: PolicyRequestId::parse("request-bonus-time").expect_value("policy request id"),
        submission_key: PolicyRequestSubmissionKey::parse("request-bonus-time-submit")
            .expect_value("policy submission key"),
        household_id: PolicyHouseholdId::parse("household-default")
            .expect_value("policy household id"),
        child_profile_id: PolicyChildProfileId::parse("child-primary")
            .expect_value("child profile id"),
        device_id: Some(PolicyDeviceId::parse("device-laptop").expect_value("policy device id")),
        source_document_id: ParentPolicyDocumentId::parse("policy-source-default")
            .expect_value("policy source id"),
        policy_version: PolicyVersion::new(7).expect_value("policy version"),
        origin: PolicyRequestOrigin::Child,
        assistant_preview_id: None,
        assistant_confirmation_state: PolicyAssistantConfirmationState::NotRequired,
        status: PolicyRequestStatus::PendingParentReview,
        scope: request_scope(PolicyRequestKind::BonusTime, Some(30)),
        requested_at: timestamp(&notification_text!("2026-06-13T20:00:00Z")),
        expires_at: timestamp(&notification_text!("2026-06-13T22:00:00Z")),
        audit_reference_ids: vec![audit_ref(&notification_text!("audit-request-created"))],
        resolved_approval_id: None,
        resolved_at: None,
    }
}

pub fn preview_request() -> ChildPolicyRequest {
    ChildPolicyRequest {
        request_id: PolicyRequestId::parse("request-assistant-preview")
            .expect_value("policy request id"),
        submission_key: PolicyRequestSubmissionKey::parse("request-assistant-preview-submit")
            .expect_value("policy submission key"),
        origin: PolicyRequestOrigin::AssistantDraft,
        assistant_preview_id: Some(
            ocentra_policy_control_core::policy_request::PolicyAssistantPreviewId::parse(
                "assistant-preview-default",
            )
            .expect_value("assistant preview id"),
        ),
        assistant_confirmation_state: PolicyAssistantConfirmationState::ParentConfirmationRequired,
        status: PolicyRequestStatus::PreviewOnly,
        scope: request_scope(PolicyRequestKind::AskParent, None),
        ..base_request()
    }
}

pub fn approved_request() -> ChildPolicyRequest {
    let mut request = base_request();
    request.status = PolicyRequestStatus::Approved;
    request.resolved_approval_id = Some(
        PolicyApprovalId::parse("request-bonus-time-grant").expect_value("policy approval id"),
    );
    request.resolved_at = Some(timestamp(&notification_text!("2026-06-13T20:05:00Z")));
    request
        .audit_reference_ids
        .push(audit_ref(&notification_text!("audit-request-approved")));
    request
}

pub fn replay_rejected_request() -> ChildPolicyRequest {
    let mut request = base_request();
    request.status = PolicyRequestStatus::ReplayRejected;
    request
}

pub fn approved_override() -> PolicyTemporaryOverride {
    PolicyTemporaryOverride {
        schema_version: policy_request_schema_version()
            .expect_value("policy request schema version"),
        override_id: PolicyOverrideId::parse("policy-override:request-bonus-time-grant")
            .expect_value("policy override id"),
        source_request_id: PolicyRequestId::parse("request-bonus-time")
            .expect_value("policy request id"),
        source_approval_id: PolicyApprovalId::parse("request-bonus-time-grant")
            .expect_value("policy approval id"),
        household_id: PolicyHouseholdId::parse("household-default")
            .expect_value("policy household id"),
        child_profile_id: PolicyChildProfileId::parse("child-primary")
            .expect_value("child profile id"),
        device_id: Some(PolicyDeviceId::parse("device-laptop").expect_value("policy device id")),
        source_document_id: ParentPolicyDocumentId::parse("policy-source-default")
            .expect_value("policy source id"),
        policy_version: PolicyVersion::new(7).expect_value("policy version"),
        request_kind: PolicyRequestKind::BonusTime,
        target: PolicyRequestTarget {
            kind: PolicyTargetKind::Category,
            reference_id: PolicyTargetReferenceId::parse("category-gaming")
                .expect_value("policy target ref"),
        },
        approved_action: PolicyRuleAction::TimeLimit,
        approved_bonus_minutes: Some(PolicyDurationMinutes::new(30).expect_value("minutes")),
        effective_at: timestamp(&notification_text!("2026-06-13T20:05:00Z")),
        expires_at: timestamp(&notification_text!("2026-06-13T22:00:00Z")),
        state: PolicyOverrideState::Active,
        audit_reference_ids: vec![audit_ref(&notification_text!("audit-policy-override"))],
    }
}

fn delivery_record(
    delivery_id: &NotificationText,
    attempt_id: &NotificationText,
    sequence: u64,
    state: PolicyDeliveryState,
    reason_code: Option<NotificationText>,
) -> PolicyDeliveryRecord {
    let initial_attempt = match state {
        PolicyDeliveryState::Queued => attempt_id.0.as_str(),
        _ => "attempt-queued",
    };
    let queued = queue_policy_delivery(
        &delivery_artifact(),
        delivery_target(),
        PolicyDeliveryId::parse(&delivery_id.0).expect_value("policy delivery id"),
        PolicyDeliveryAttemptId::parse(initial_attempt).expect_value("policy attempt id"),
        vec![audit_ref(&notification_text!("audit-policy-queued"))],
    )
    .expect_value("queued policy delivery");
    if state == PolicyDeliveryState::Queued {
        return queued;
    }
    apply_policy_delivery_transition(
        &queued,
        PolicyDeliveryTransition {
            attempt_id: PolicyDeliveryAttemptId::parse(&attempt_id.0)
                .expect_value("policy attempt id"),
            sequence: PolicyDeliverySequence::new(sequence).expect_value("policy delivery seq"),
            state,
            audit_reference_ids: vec![delivery_audit_ref(state)],
            reason_code: reason_code
                .map(|value| PolicyReasonCode::parse(value.0).expect_value("policy reason code")),
            superseded_by_policy_version: None,
            rollback_reference_state: None,
        },
    )
    .expect_value("validated policy delivery transition")
    .into_record()
}

fn delivery_artifact() -> CompiledDomainPolicyArtifact {
    CompiledDomainPolicyArtifact {
        household_id: PolicyHouseholdId::parse("household-default")
            .expect_value("policy household id"),
        policy_version: PolicyVersion::new(7).expect_value("policy version"),
        source_document_id: ParentPolicyDocumentId::parse("policy-source-default")
            .expect_value("policy source id"),
        domain: PolicyConsumerDomain::Tracking,
        rule_count: 1,
        schedules: Vec::new(),
        audit_reference_ids: vec![audit_ref(&notification_text!("audit-request-created"))],
        superseded_by_policy_version: None,
        rollback_ref: None,
    }
}

fn delivery_target() -> PolicyDeliveryTarget {
    PolicyDeliveryTarget {
        child_profile_id: PolicyChildProfileId::parse("child-primary")
            .expect_value("child profile id"),
        device_id: PolicyDeviceId::parse("device-laptop").expect_value("policy device id"),
        domain: PolicyConsumerDomain::Tracking,
    }
}

fn delivery_audit_ref(state: PolicyDeliveryState) -> PolicyAuditReferenceId {
    let value = match state {
        PolicyDeliveryState::RetryScheduled => "audit-policy-retry",
        PolicyDeliveryState::BlockedByPermission => "audit-policy-blocked",
        _ => "audit-policy-delivery",
    };
    audit_ref(&notification_text!(value))
}

pub fn queued_delivery() -> PolicyDeliveryRecord {
    delivery_record(
        &notification_text!("delivery-policy-queued"),
        &notification_text!("attempt-queued"),
        1,
        PolicyDeliveryState::Queued,
        None,
    )
}

pub fn legacy_unverified_acknowledged_delivery() -> PolicyDeliveryRecord {
    let queued = delivery_record(
        &notification_text!("delivery-policy-legacy-acknowledged"),
        &notification_text!("attempt-queued"),
        1,
        PolicyDeliveryState::Queued,
        None,
    );
    let mut serialized =
        serde_json::to_value(queued).expect_value("serialize legacy acknowledged delivery");
    serialized["schema_version"] = serde_json::json!(1);
    serialized["state"] = serde_json::json!("acknowledged");
    serialized["last_sequence"] = serde_json::json!(2);
    serialized["last_attempt_id"] = serde_json::json!("attempt-legacy-acknowledged");
    serialized["audit_reference_ids"] = serde_json::json!(["audit-policy-legacy-acknowledged"]);
    serialized["execution_receipt"] = serde_json::Value::Null;

    serde_json::from_value(serialized)
        .expect_value("hydrate schema-v1 receiptless acknowledged delivery")
}

pub fn retry_delivery() -> PolicyDeliveryRecord {
    delivery_record(
        &notification_text!("delivery-policy-retry"),
        &notification_text!("attempt-retry"),
        2,
        PolicyDeliveryState::RetryScheduled,
        Some(notification_text!("retry-scheduled")),
    )
}

pub fn blocked_delivery() -> PolicyDeliveryRecord {
    delivery_record(
        &notification_text!("delivery-policy-blocked"),
        &notification_text!("attempt-blocked"),
        2,
        PolicyDeliveryState::BlockedByPermission,
        Some(notification_text!("blocked-by-permission")),
    )
}
