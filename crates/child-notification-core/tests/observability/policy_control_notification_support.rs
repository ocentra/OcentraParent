use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::activity::policy_preview::{
    PolicyAssistantConfirmationState, PolicyRequestOrigin, PolicyRequestStatus,
};
use ocentra_policy_control_core::policy_delivery::{
    policy_delivery_schema_version, PolicyDeliveryAttemptId, PolicyDeliveryId,
    PolicyDeliveryRecord, PolicyDeliverySequence, PolicyDeliveryState, PolicyDeliveryTarget,
};
use ocentra_policy_control_core::policy_request::{
    policy_request_schema_version, ChildPolicyRequest, PolicyApprovalId, PolicyDurationMinutes,
    PolicyOverrideId, PolicyOverrideState, PolicyRequestId, PolicyRequestKind, PolicyRequestScope,
    PolicyRequestSubmissionKey, PolicyRequestTarget, PolicyRequestTimestamp,
    PolicyTemporaryOverride,
};
use ocentra_policy_control_core::policy_source::{
    ParentPolicyDocumentId, PolicyAuditReferenceId, PolicyChildProfileId, PolicyConsumerDomain,
    PolicyDeviceId, PolicyHouseholdId, PolicyReasonCode, PolicyRuleAction, PolicyRuleId,
    PolicyTargetKind, PolicyTargetReferenceId, PolicyVersion,
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
    PolicyDeliveryRecord {
        schema_version: policy_delivery_schema_version()
            .expect_value("policy delivery schema version"),
        delivery_id: PolicyDeliveryId::parse(&delivery_id.0).expect_value("policy delivery id"),
        household_id: PolicyHouseholdId::parse("household-default")
            .expect_value("policy household id"),
        policy_version: PolicyVersion::new(7).expect_value("policy version"),
        source_document_id: ParentPolicyDocumentId::parse("policy-source-default")
            .expect_value("policy source id"),
        target: PolicyDeliveryTarget {
            child_profile_id: PolicyChildProfileId::parse("child-primary")
                .expect_value("child profile id"),
            device_id: PolicyDeviceId::parse("device-laptop").expect_value("policy device id"),
            domain: PolicyConsumerDomain::Tracking,
        },
        state,
        last_sequence: PolicyDeliverySequence::new(sequence).expect_value("policy delivery seq"),
        last_attempt_id: PolicyDeliveryAttemptId::parse(&attempt_id.0)
            .expect_value("policy attempt id"),
        audit_reference_ids: vec![audit_ref(&notification_text!(match state {
            PolicyDeliveryState::Queued => "audit-policy-queued",
            PolicyDeliveryState::Applied => "audit-policy-applied",
            PolicyDeliveryState::RetryScheduled => "audit-policy-retry",
            PolicyDeliveryState::BlockedByPermission => "audit-policy-blocked",
            _ => "audit-policy-delivery",
        }))],
        source_audit_reference_ids: vec![audit_ref(&notification_text!("audit-request-created"))],
        source_superseded_by_policy_version: None,
        source_rollback_ref: None,
        reason_code: reason_code
            .map(|value| PolicyReasonCode::parse(&value.0).expect_value("policy reason code")),
        superseded_by_policy_version: None,
        rollback_reference_state: None,
    }
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

pub fn applied_delivery() -> PolicyDeliveryRecord {
    delivery_record(
        &notification_text!("delivery-policy-applied"),
        &notification_text!("attempt-applied"),
        2,
        PolicyDeliveryState::Applied,
        None,
    )
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
