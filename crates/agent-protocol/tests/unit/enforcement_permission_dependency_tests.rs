use super::{
    constants, constants::enforcement, policy_constants as policy, EnforcementAction,
    EnforcementAdapterKind, EnforcementAdapterResultCode, EnforcementAuditEvent,
    EnforcementAuditEventKind, EnforcementCapabilityState, EnforcementCapabilityStatus,
    EnforcementDependencyState, EnforcementIntent, EnforcementIntentSource, EnforcementMode,
    EnforcementPermissionState, EnforcementResult, EnforcementResultStatus,
    EnforcementRollbackState, EnforcementTimerEvent, EnforcementTimerEventKind,
    EnforcementUnavailableReason, EnforcementUnavailableStatus, ParentDeviceReference,
    ParentEvidenceReference, ParentEvidenceReferenceKind, ParentPlatform, PolicyAction,
    PolicyTarget, PolicyTargetType,
};
use ocentra_eventing::expect_value::ExpectValue;

#[test]
fn permission_and_dependency_unavailable_events_serialize_typed_recovery_data() {
    for (capability, reason, expected_reason) in [
        (
            unavailable_capability_missing_permission(),
            EnforcementUnavailableReason::MissingPermission,
            enforcement::UNAVAILABLE_MISSING_PERMISSION,
        ),
        (
            unavailable_capability_missing_dependency(),
            EnforcementUnavailableReason::MissingDependency,
            enforcement::UNAVAILABLE_MISSING_DEPENDENCY,
        ),
    ] {
        let (serialized_audit, serialized_timer) =
            serialized_unavailable_events(capability, reason);

        assert_eq!(
            serialized_audit["result"]["status"],
            enforcement::RESULT_UNAVAILABLE
        );
        assert_eq!(
            serialized_audit["result"]["adapterResultCode"],
            enforcement::ADAPTER_UNAVAILABLE
        );
        assert_eq!(
            serialized_audit["unavailableStatus"]["unavailableReason"],
            expected_reason
        );
        assert_eq!(serialized_audit["unavailableStatus"]["retryable"], false);
        assert_eq!(serialized_timer["unavailableReason"], expected_reason);
    }
}

fn serialized_unavailable_events(
    capability: EnforcementCapabilityStatus,
    reason: EnforcementUnavailableReason,
) -> (serde_json::Value, serde_json::Value) {
    let intent = enforcement_intent();
    let action = enforcement_action(&intent, capability.clone());
    let unavailable = unavailable_status(capability.clone(), reason);
    let result = enforcement_result(&action, capability.clone(), unavailable.clone(), reason);
    let audit = enforcement_audit(action.clone(), result, capability, unavailable);
    let timer = enforcement_timer(action, reason);

    (
        serde_json::to_value(audit).expect_value(constants::error::AGENT_EVENT_SERIALIZES),
        serde_json::to_value(timer).expect_value(constants::error::AGENT_EVENT_SERIALIZES),
    )
}

fn unavailable_capability_missing_permission() -> EnforcementCapabilityStatus {
    EnforcementCapabilityStatus {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        platform: ParentPlatform::Windows,
        adapter_kind: EnforcementAdapterKind::ProcessControl,
        capability_state: EnforcementCapabilityState::Unavailable,
        permission_state: EnforcementPermissionState::MissingPermission,
        dependency_state: EnforcementDependencyState::Installed,
        supported_actions: Vec::new(),
        degraded_reason: Some(enforcement::UNAVAILABLE_MISSING_PERMISSION.to_string()),
        last_checked_at: policy::TEST_EVALUATED_AT.to_string(),
    }
}

fn unavailable_capability_missing_dependency() -> EnforcementCapabilityStatus {
    EnforcementCapabilityStatus {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        platform: ParentPlatform::Windows,
        adapter_kind: EnforcementAdapterKind::ProcessControl,
        capability_state: EnforcementCapabilityState::Unavailable,
        permission_state: EnforcementPermissionState::Allowed,
        dependency_state: EnforcementDependencyState::Missing,
        supported_actions: Vec::new(),
        degraded_reason: Some(enforcement::UNAVAILABLE_MISSING_DEPENDENCY.to_string()),
        last_checked_at: policy::TEST_EVALUATED_AT.to_string(),
    }
}

fn enforcement_intent() -> EnforcementIntent {
    EnforcementIntent {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        intent_id: enforcement::TEST_INTENT_ID.to_string(),
        source: EnforcementIntentSource::LocalPolicyEvaluator,
        actor: None,
        device: device(),
        policy_decision_id: policy::TEST_DECISION_ID.to_string(),
        target: target(),
        requested_action: PolicyAction::Block,
        evidence_references: vec![evidence()],
        parent_approval: None,
        idempotency_key: enforcement::TEST_IDEMPOTENCY_KEY.to_string(),
    }
}

fn enforcement_action(
    intent: &EnforcementIntent,
    capability: EnforcementCapabilityStatus,
) -> EnforcementAction {
    EnforcementAction {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        action_id: enforcement::TEST_ACTION_ID.to_string(),
        intent_id: intent.intent_id.clone(),
        policy_decision_id: intent.policy_decision_id.clone(),
        policy_action: intent.requested_action,
        adapter_kind: EnforcementAdapterKind::ProcessControl,
        platform: ParentPlatform::Windows,
        target: intent.target.clone(),
        mode: EnforcementMode::TerminateProcess,
        capability,
        reason_codes: vec![policy::TEST_REASON_PARENT_BLOCK.to_string()],
        evidence_references: vec![evidence()],
        local_ai_result_id: None,
        parent_approval: None,
        dry_run: false,
        requested_at: policy::TEST_EVALUATED_AT.to_string(),
        expires_at: Some(policy::TEST_EXPIRES_AT.to_string()),
        rollback_token: Some(enforcement::TEST_ROLLBACK_TOKEN.to_string()),
    }
}

fn unavailable_status(
    capability: EnforcementCapabilityStatus,
    reason: EnforcementUnavailableReason,
) -> EnforcementUnavailableStatus {
    EnforcementUnavailableStatus {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        capability,
        unavailable_reason: reason,
        retryable: false,
        checked_at: policy::TEST_EVALUATED_AT.to_string(),
    }
}

fn enforcement_result(
    action: &EnforcementAction,
    capability: EnforcementCapabilityStatus,
    unavailable_status: EnforcementUnavailableStatus,
    reason: EnforcementUnavailableReason,
) -> EnforcementResult {
    EnforcementResult {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        result_id: enforcement::TEST_RESULT_ID.to_string(),
        action_id: action.action_id.clone(),
        status: EnforcementResultStatus::Unavailable,
        adapter_result_code: EnforcementAdapterResultCode::AdapterUnavailable,
        started_at: policy::TEST_EVALUATED_AT.to_string(),
        completed_at: Some(policy::TEST_EVALUATED_AT.to_string()),
        rollback_token: action.rollback_token.clone(),
        rollback_state: EnforcementRollbackState::Unavailable,
        unavailable_reason: Some(reason.as_protocol_str().to_string()),
        unavailable_status: Some(unavailable_status),
        failed_reason: None,
        next_check_at: action.expires_at.clone(),
        capability,
    }
}

fn enforcement_audit(
    action: EnforcementAction,
    result: EnforcementResult,
    capability: EnforcementCapabilityStatus,
    unavailable_status: EnforcementUnavailableStatus,
) -> EnforcementAuditEvent {
    EnforcementAuditEvent {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        audit_event_id: enforcement::TEST_AUDIT_EVENT_ID.to_string(),
        audit_event_kind: EnforcementAuditEventKind::Unavailable,
        action,
        result,
        capability,
        unavailable_status: Some(unavailable_status),
        policy_version: policy::TEST_POLICY_VERSION.to_string(),
        evidence_references: vec![evidence()],
        actor: None,
        parent_override: None,
        journal_sequence: Some(enforcement::TEST_JOURNAL_SEQUENCE.to_string()),
        observed_at: policy::TEST_EVALUATED_AT.to_string(),
    }
}

fn enforcement_timer(
    action: EnforcementAction,
    reason: EnforcementUnavailableReason,
) -> EnforcementTimerEvent {
    EnforcementTimerEvent {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        timer_event_id: enforcement::TEST_TIMER_EVENT_ID.to_string(),
        timer_event_kind: EnforcementTimerEventKind::Unavailable,
        action_id: action.action_id,
        policy_decision_id: policy::TEST_DECISION_ID.to_string(),
        evidence_references: vec![evidence()],
        scheduled_at: policy::TEST_EVALUATED_AT.to_string(),
        effective_at: None,
        rollback_token: Some(enforcement::TEST_ROLLBACK_TOKEN.to_string()),
        recovered_after_restart: false,
        unavailable_reason: Some(reason),
    }
}

fn target() -> PolicyTarget {
    PolicyTarget {
        target_id: enforcement::TEST_PROCESS_TARGET_ID.to_string(),
        target_type: PolicyTargetType::Process,
        target_value: enforcement::TEST_PROCESS_TARGET_VALUE.to_string(),
    }
}

fn evidence() -> ParentEvidenceReference {
    ParentEvidenceReference {
        evidence_reference_id: policy::TEST_EVIDENCE_ID.to_string(),
        kind: ParentEvidenceReferenceKind::ActivityEvent,
        observed_at: policy::TEST_EVALUATED_AT.to_string(),
    }
}

fn device() -> ParentDeviceReference {
    ParentDeviceReference {
        device_id: enforcement::TEST_CHILD_DEVICE_ID.to_string(),
        child_profile_id: Some(policy::TEST_CHILD_PROFILE_ID.to_string()),
        label: enforcement::TEST_CHILD_DEVICE_LABEL.to_string(),
        platform: enforcement::PLATFORM_WINDOWS.to_string(),
    }
}
