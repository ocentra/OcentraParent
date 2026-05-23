use super::*;
use ocentra_parent_agent_protocol::{
    constants::enforcement, policy_constants as policy, EnforcementAdapterKind,
    EnforcementAdapterResultCode, EnforcementCapabilityState, EnforcementCapabilityStatus,
    EnforcementDependencyState, EnforcementIntent, EnforcementIntentSource, EnforcementMode,
    EnforcementPermissionState, EnforcementResultStatus, EnforcementRollbackState,
    EnforcementTimerEvent, EnforcementTimerEventKind, ParentDeviceReference,
    ParentEvidenceReference, ParentEvidenceReferenceKind, ParentPlatform, PolicyAction,
    PolicyDecision, PolicyDecisionHandoffState, PolicyTarget, PolicyTargetType,
};

#[test]
fn timer_event_derives_restart_recovery_unavailable_expiry_and_rollback_states() {
    let mut restart_input = boundary_input(policy_decision(true), supported_capability());
    restart_input.intent.source = EnforcementIntentSource::SystemRecovery;
    assert_timer(
        evaluate_enforcement_boundary(restart_input)
            .expect(enforcement::TIMER_RESTART_RECOVERED)
            .timer_event,
        enforcement::TIMER_RESTART_RECOVERED,
        Some(policy::TEST_EXPIRES_AT),
        None,
    );

    assert_timer(
        evaluate_enforcement_boundary(boundary_input(
            policy_decision(false),
            unavailable_capability(),
        ))
        .expect(enforcement::TIMER_UNAVAILABLE)
        .timer_event,
        enforcement::TIMER_UNAVAILABLE,
        None,
        Some(enforcement::UNAVAILABLE_UNSUPPORTED_PLATFORM),
    );

    assert_timer(
        evaluate_enforcement_boundary(adapter_input(
            EnforcementResultStatus::Failed,
            EnforcementAdapterResultCode::AdapterFailed,
            EnforcementRollbackState::Failed,
        ))
        .expect(enforcement::TIMER_RECOVERY_NEEDED)
        .timer_event,
        enforcement::TIMER_RECOVERY_NEEDED,
        None,
        Some(enforcement::UNAVAILABLE_ADAPTER_ERROR),
    );

    assert_timer(
        evaluate_enforcement_boundary(adapter_input(
            EnforcementResultStatus::Expired,
            EnforcementAdapterResultCode::TimerExpired,
            EnforcementRollbackState::NotRequired,
        ))
        .expect(enforcement::TIMER_EXPIRED)
        .timer_event,
        enforcement::TIMER_EXPIRED,
        Some(policy::TEST_EXPIRES_AT),
        None,
    );

    assert_timer(
        evaluate_enforcement_boundary(adapter_input(
            EnforcementResultStatus::RolledBack,
            EnforcementAdapterResultCode::RollbackCompleted,
            EnforcementRollbackState::Completed,
        ))
        .expect(enforcement::TIMER_ROLLBACK_COMPLETED)
        .timer_event,
        enforcement::TIMER_ROLLBACK_COMPLETED,
        None,
        None,
    );
}

#[test]
fn explicit_timer_transition_builds_extended_and_cancelled_events() {
    let mut extended_input = boundary_input(policy_decision(true), supported_capability());
    extended_input.timer_event_kind = Some(EnforcementTimerEventKind::Extended);
    assert_timer(
        evaluate_enforcement_boundary(extended_input)
            .expect(enforcement::TIMER_EXTENDED)
            .timer_event,
        enforcement::TIMER_EXTENDED,
        Some(policy::TEST_EXPIRES_AT),
        None,
    );

    let mut cancelled_input = boundary_input(policy_decision(true), supported_capability());
    cancelled_input.timer_event_kind = Some(EnforcementTimerEventKind::Cancelled);
    assert_timer(
        evaluate_enforcement_boundary(cancelled_input)
            .expect(enforcement::TIMER_CANCELLED)
            .timer_event,
        enforcement::TIMER_CANCELLED,
        None,
        None,
    );
}

fn assert_timer(
    timer: Option<EnforcementTimerEvent>,
    expected_kind: &str,
    expected_effective_at: Option<&str>,
    expected_reason: Option<&str>,
) {
    let timer = timer.expect(enforcement::TEST_TIMER_EVENT_ID);
    assert_eq!(timer.timer_event_kind.as_protocol_str(), expected_kind);
    assert_eq!(timer.effective_at.as_deref(), expected_effective_at);
    assert_eq!(
        timer
            .unavailable_reason
            .map(|reason| reason.as_protocol_str()),
        expected_reason
    );
}

fn adapter_input(
    status: EnforcementResultStatus,
    adapter_result_code: EnforcementAdapterResultCode,
    rollback_state: EnforcementRollbackState,
) -> EnforcementBoundaryInput {
    let mut input = boundary_input(policy_decision(false), supported_capability());
    input.adapter_outcome = Some(EnforcementAdapterOutcome {
        status,
        adapter_result_code,
        completed_at: Some(policy::TEST_EXPIRES_AT.to_string()),
        unavailable_reason: None,
        failed_reason: None,
        rollback_token: Some(enforcement::TEST_ROLLBACK_TOKEN.to_string()),
        rollback_state,
    });
    input
}

fn boundary_input(
    decision: PolicyDecision,
    capability: EnforcementCapabilityStatus,
) -> EnforcementBoundaryInput {
    EnforcementBoundaryInput {
        intent: intent(decision.action),
        decision,
        capability,
        action_id: enforcement::TEST_ACTION_ID.to_string(),
        result_id: enforcement::TEST_RESULT_ID.to_string(),
        audit_event_id: enforcement::TEST_AUDIT_EVENT_ID.to_string(),
        timer_event_id: enforcement::TEST_TIMER_EVENT_ID.to_string(),
        rollback_token: Some(enforcement::TEST_ROLLBACK_TOKEN.to_string()),
        policy_version: policy::TEST_POLICY_VERSION.to_string(),
        requested_at: policy::TEST_EVALUATED_AT.to_string(),
        completed_at: Some(policy::TEST_EVALUATED_AT.to_string()),
        adapter_outcome: None,
        timer_event_kind: None,
    }
}

fn policy_decision(dry_run: bool) -> PolicyDecision {
    PolicyDecision {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        decision_id: policy::TEST_DECISION_ID.to_string(),
        action: PolicyAction::Block,
        reason_codes: vec![policy::TEST_REASON_PARENT_BLOCK.to_string()],
        evidence_references: vec![evidence()],
        rule_ids: vec![policy::TEST_BLOCK_RULE_ID.to_string()],
        local_ai_result_id: None,
        dry_run,
        enforcement_handoff_state: PolicyDecisionHandoffState::HandedOff,
        expires_at: Some(policy::TEST_EXPIRES_AT.to_string()),
    }
}

fn intent(action: PolicyAction) -> EnforcementIntent {
    EnforcementIntent {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        intent_id: enforcement::TEST_INTENT_ID.to_string(),
        source: EnforcementIntentSource::LocalPolicyEvaluator,
        actor: None,
        device: device(),
        policy_decision_id: policy::TEST_DECISION_ID.to_string(),
        target: target(),
        requested_action: action,
        evidence_references: vec![evidence()],
        parent_approval: None,
        idempotency_key: enforcement::TEST_IDEMPOTENCY_KEY.to_string(),
    }
}

fn supported_capability() -> EnforcementCapabilityStatus {
    EnforcementCapabilityStatus {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        platform: ParentPlatform::Windows,
        adapter_kind: EnforcementAdapterKind::ProcessControl,
        capability_state: EnforcementCapabilityState::Supported,
        permission_state: EnforcementPermissionState::NotRequired,
        dependency_state: EnforcementDependencyState::Installed,
        supported_actions: vec![EnforcementMode::TerminateProcess],
        degraded_reason: None,
        last_checked_at: policy::TEST_EVALUATED_AT.to_string(),
    }
}

fn unavailable_capability() -> EnforcementCapabilityStatus {
    EnforcementCapabilityStatus {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        platform: ParentPlatform::Windows,
        adapter_kind: EnforcementAdapterKind::ProcessControl,
        capability_state: EnforcementCapabilityState::Unavailable,
        permission_state: EnforcementPermissionState::NotRequired,
        dependency_state: EnforcementDependencyState::NotRequired,
        supported_actions: Vec::new(),
        degraded_reason: Some(enforcement::UNAVAILABLE_UNSUPPORTED_PLATFORM.to_string()),
        last_checked_at: policy::TEST_EVALUATED_AT.to_string(),
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
