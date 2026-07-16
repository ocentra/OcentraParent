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
fn degraded_unavailable_audit_and_timer_serialize_recovery_data() {
    let (serialized_audit, serialized_timer) = serialized_degraded_unavailable_events();

    assert_eq!(
        serialized_audit["auditEventKind"],
        enforcement::AUDIT_UNAVAILABLE
    );
    assert_eq!(
        serialized_audit["capability"]["capabilityState"],
        enforcement::CAPABILITY_DEGRADED
    );
    assert_eq!(
        serialized_audit["unavailableStatus"]["unavailableReason"],
        enforcement::UNAVAILABLE_ADAPTER_UNAVAILABLE
    );
    assert_eq!(
        serialized_timer["timerEventKind"],
        enforcement::TIMER_UNAVAILABLE
    );
    assert_eq!(
        serialized_timer["unavailableReason"],
        enforcement::UNAVAILABLE_ADAPTER_UNAVAILABLE
    );
}

#[test]
fn manual_required_capability_serializes_as_unavailable_proof_state() {
    let capability = EnforcementCapabilityStatus {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        platform: ParentPlatform::Windows,
        adapter_kind: EnforcementAdapterKind::NetworkControl,
        capability_state: EnforcementCapabilityState::ManualRequired,
        permission_state: EnforcementPermissionState::Unknown,
        dependency_state: EnforcementDependencyState::Unknown,
        supported_actions: vec![EnforcementMode::TemporaryBlock],
        degraded_reason: Some(enforcement::UNAVAILABLE_MANUAL_REQUIRED.to_string()),
        last_checked_at: policy::TEST_EVALUATED_AT.to_string(),
    };
    let unavailable = EnforcementUnavailableStatus {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        capability,
        unavailable_reason: EnforcementUnavailableReason::ManualRequired,
        retryable: false,
        checked_at: policy::TEST_EVALUATED_AT.to_string(),
    };

    let serialized =
        serde_json::to_value(unavailable).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        serialized["capability"]["capabilityState"],
        enforcement::CAPABILITY_MANUAL_REQUIRED
    );
    assert_eq!(
        serialized["unavailableReason"],
        enforcement::UNAVAILABLE_MANUAL_REQUIRED
    );
    assert_eq!(serialized["retryable"], false);
}

fn serialized_degraded_unavailable_events() -> (serde_json::Value, serde_json::Value) {
    let capability = degraded_capability();
    let intent = enforcement_intent();
    let action = enforcement_action(&intent, capability.clone());
    let unavailable = unavailable_status(capability.clone());
    let result = enforcement_result(&action, capability.clone(), unavailable.clone());
    let audit = enforcement_audit(action.clone(), result, capability, unavailable);
    let timer = enforcement_timer(action);

    (
        serde_json::to_value(audit).expect_value(constants::error::AGENT_EVENT_SERIALIZES),
        serde_json::to_value(timer).expect_value(constants::error::AGENT_EVENT_SERIALIZES),
    )
}

fn degraded_capability() -> EnforcementCapabilityStatus {
    EnforcementCapabilityStatus {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        platform: ParentPlatform::Windows,
        adapter_kind: EnforcementAdapterKind::ProcessControl,
        capability_state: EnforcementCapabilityState::Degraded,
        permission_state: EnforcementPermissionState::Allowed,
        dependency_state: EnforcementDependencyState::Installed,
        supported_actions: vec![EnforcementMode::TerminateProcess],
        degraded_reason: Some(enforcement::UNAVAILABLE_ADAPTER_UNAVAILABLE.to_string()),
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

fn unavailable_status(capability: EnforcementCapabilityStatus) -> EnforcementUnavailableStatus {
    EnforcementUnavailableStatus {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        capability,
        unavailable_reason: EnforcementUnavailableReason::AdapterUnavailable,
        retryable: true,
        checked_at: policy::TEST_EVALUATED_AT.to_string(),
    }
}

fn enforcement_result(
    action: &EnforcementAction,
    capability: EnforcementCapabilityStatus,
    unavailable_status: EnforcementUnavailableStatus,
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
        unavailable_reason: Some(enforcement::UNAVAILABLE_ADAPTER_UNAVAILABLE.to_string()),
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

fn enforcement_timer(action: EnforcementAction) -> EnforcementTimerEvent {
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
        unavailable_reason: Some(EnforcementUnavailableReason::AdapterUnavailable),
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
