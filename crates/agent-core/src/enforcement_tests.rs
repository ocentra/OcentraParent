use super::*;
use ocentra_parent_agent_protocol::{
    constants::enforcement, policy_constants as policy, EnforcementAdapterKind,
    EnforcementCapabilityState, EnforcementCapabilityStatus, EnforcementDependencyState,
    EnforcementIntent, EnforcementIntentSource, EnforcementMode, EnforcementPermissionState,
    EnforcementRollbackState, ParentDeviceReference, ParentEvidenceReference,
    ParentEvidenceReferenceKind, ParentPlatform, PolicyAction, PolicyDecision,
    PolicyDecisionHandoffState, PolicyTarget, PolicyTargetType,
};

#[test]
fn dry_run_decision_never_requests_adapter_execution() {
    let input = boundary_input(policy_decision(true), supported_capability());

    let outcome = evaluate_enforcement_boundary(input).expect(policy::TEST_DECISION_ID);

    assert!(outcome.action.dry_run);
    assert_eq!(
        outcome.result.status.as_protocol_str(),
        enforcement::RESULT_WOULD_ENFORCE
    );
    assert_eq!(
        outcome.result.adapter_result_code.as_protocol_str(),
        enforcement::ADAPTER_DRY_RUN_NO_ACTION
    );
    assert_eq!(
        outcome.result.rollback_state.as_protocol_str(),
        enforcement::ROLLBACK_NOT_REQUIRED
    );
    assert_eq!(
        outcome.action.capability.capability_state.as_protocol_str(),
        enforcement::CAPABILITY_SUPPORTED
    );
    assert_eq!(outcome.result.unavailable_status, None);
    assert_eq!(outcome.adapter_request, None);
    assert_eq!(
        outcome.audit_event.audit_event_kind.as_protocol_str(),
        enforcement::AUDIT_ATTEMPTED
    );
    assert_eq!(
        outcome
            .timer_event
            .expect(policy::TEST_EXPIRES_AT)
            .timer_event_kind
            .as_protocol_str(),
        enforcement::TIMER_CREATED
    );
}

#[test]
fn mismatched_policy_decision_id_rejects_before_action_building() {
    let mut input = boundary_input(policy_decision(true), supported_capability());
    input.intent.policy_decision_id = enforcement::TEST_RESULT_ID.to_string();

    let rejected = evaluate_enforcement_boundary(input)
        .expect_err(enforcement::REJECTION_DECISION_ID_MISMATCH);

    assert_eq!(
        rejected.as_protocol_str(),
        enforcement::REJECTION_DECISION_ID_MISMATCH
    );
}

#[test]
fn missing_policy_evidence_rejects_before_adapter_path() {
    let mut decision = policy_decision(false);
    decision.evidence_references = Vec::new();
    let input = boundary_input(decision, supported_capability());

    let rejected =
        evaluate_enforcement_boundary(input).expect_err(enforcement::REJECTION_MISSING_EVIDENCE);

    assert_eq!(
        rejected.as_protocol_str(),
        enforcement::REJECTION_MISSING_EVIDENCE
    );
}

#[test]
fn unavailable_capability_returns_auditable_unavailable_result() {
    let input = boundary_input(policy_decision(false), unavailable_capability());

    let outcome = evaluate_enforcement_boundary(input).expect(enforcement::ADAPTER_UNAVAILABLE);

    assert_eq!(
        outcome.result.status.as_protocol_str(),
        enforcement::RESULT_UNAVAILABLE
    );
    assert_eq!(
        outcome.result.adapter_result_code.as_protocol_str(),
        enforcement::ADAPTER_UNSUPPORTED_PLATFORM
    );
    assert_eq!(
        outcome.result.rollback_state.as_protocol_str(),
        enforcement::ROLLBACK_UNAVAILABLE
    );
    let unavailable_status = outcome
        .result
        .unavailable_status
        .as_ref()
        .expect(enforcement::UNAVAILABLE_UNSUPPORTED_PLATFORM);
    assert_eq!(
        unavailable_status.unavailable_reason.as_protocol_str(),
        enforcement::UNAVAILABLE_UNSUPPORTED_PLATFORM
    );
    assert!(!unavailable_status.retryable);
    assert_eq!(
        outcome
            .audit_event
            .capability
            .capability_state
            .as_protocol_str(),
        enforcement::CAPABILITY_UNAVAILABLE
    );
    assert_eq!(
        outcome
            .audit_event
            .unavailable_status
            .as_ref()
            .expect(enforcement::UNAVAILABLE_UNSUPPORTED_PLATFORM)
            .unavailable_reason
            .as_protocol_str(),
        enforcement::UNAVAILABLE_UNSUPPORTED_PLATFORM
    );
    assert_eq!(
        outcome.audit_event.audit_event_kind.as_protocol_str(),
        enforcement::AUDIT_UNAVAILABLE
    );
    assert_eq!(outcome.adapter_request, None);
}

#[test]
fn unsupported_action_returns_typed_unavailable_status_without_adapter_execution() {
    let input = boundary_input(policy_decision(false), unsupported_action_capability());

    let outcome = evaluate_enforcement_boundary(input).expect(enforcement::ADAPTER_UNAVAILABLE);

    assert_eq!(
        outcome.result.status.as_protocol_str(),
        enforcement::RESULT_UNAVAILABLE
    );
    assert_eq!(
        outcome
            .result
            .unavailable_status
            .as_ref()
            .expect(enforcement::UNAVAILABLE_UNSUPPORTED_ACTION)
            .unavailable_reason
            .as_protocol_str(),
        enforcement::UNAVAILABLE_UNSUPPORTED_ACTION
    );
    assert_eq!(
        outcome.result.unavailable_reason.as_deref(),
        Some(enforcement::UNAVAILABLE_UNSUPPORTED_ACTION)
    );
    assert_eq!(outcome.adapter_request, None);
}

#[test]
fn supported_non_dry_run_requires_adapter_outcome_for_process_control() {
    let input = boundary_input(policy_decision(false), supported_capability());

    let rejected = evaluate_enforcement_boundary(input)
        .expect_err(enforcement::REJECTION_ADAPTER_RESULT_REQUIRED);

    assert_eq!(
        rejected.as_protocol_str(),
        enforcement::REJECTION_ADAPTER_RESULT_REQUIRED
    );
}

#[test]
fn adapter_outcome_maps_to_success_result_and_audit() {
    let mut input = boundary_input(policy_decision(false), supported_capability());
    input.adapter_outcome = Some(EnforcementAdapterOutcome {
        status: ocentra_parent_agent_protocol::EnforcementResultStatus::ActuallyEnforced,
        adapter_result_code:
            ocentra_parent_agent_protocol::EnforcementAdapterResultCode::ProcessTerminated,
        completed_at: Some(policy::TEST_EVALUATED_AT.to_string()),
        unavailable_reason: None,
        failed_reason: None,
        rollback_token: Some(enforcement::TEST_ROLLBACK_TOKEN.to_string()),
        rollback_state: EnforcementRollbackState::Available,
    });

    let outcome =
        evaluate_enforcement_boundary(input).expect(enforcement::ADAPTER_PROCESS_TERMINATED);

    assert_eq!(
        outcome.result.status.as_protocol_str(),
        enforcement::RESULT_ACTUALLY_ENFORCED
    );
    assert_eq!(
        outcome.audit_event.audit_event_kind.as_protocol_str(),
        enforcement::AUDIT_SUCCEEDED
    );
    assert_eq!(
        outcome.result.rollback_state.as_protocol_str(),
        enforcement::ROLLBACK_AVAILABLE
    );
    assert_eq!(outcome.audit_event.unavailable_status, None);
    assert_eq!(outcome.adapter_request, None);
}

#[test]
fn process_adapter_reports_real_platform_result_with_explicit_rollback_state() {
    let outcome = terminate_owned_process(
        OwnedProcessTerminationTarget {
            pid: u32::MAX,
            expected_process_name: enforcement::TEST_PROCESS_TARGET_VALUE.to_string(),
        },
        policy::TEST_EVALUATED_AT,
    );

    #[cfg(windows)]
    {
        assert_eq!(
            outcome.adapter_result_code.as_protocol_str(),
            enforcement::ADAPTER_PROCESS_ALREADY_EXITED
        );
        assert_eq!(
            outcome.rollback_state.as_protocol_str(),
            enforcement::ROLLBACK_NOT_REQUIRED
        );
    }

    #[cfg(not(windows))]
    {
        assert_eq!(
            outcome.adapter_result_code.as_protocol_str(),
            enforcement::ADAPTER_UNSUPPORTED_PLATFORM
        );
        assert_eq!(
            outcome.rollback_state.as_protocol_str(),
            enforcement::ROLLBACK_UNAVAILABLE
        );
    }
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

fn unsupported_action_capability() -> EnforcementCapabilityStatus {
    EnforcementCapabilityStatus {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        platform: ParentPlatform::Windows,
        adapter_kind: EnforcementAdapterKind::ProcessControl,
        capability_state: EnforcementCapabilityState::Supported,
        permission_state: EnforcementPermissionState::NotRequired,
        dependency_state: EnforcementDependencyState::Installed,
        supported_actions: Vec::new(),
        degraded_reason: None,
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
