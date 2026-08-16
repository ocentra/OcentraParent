use crate::test_text::{test_ok as ok, test_some as some, TestResult};
use crate::*;
use ocentra_parent_agent_protocol::activity::policy::ParentEvidenceReference;
use ocentra_parent_agent_protocol::activity::policy::ParentEvidenceReferenceKind;
use ocentra_parent_agent_protocol::activity::policy::PolicyAction;
use ocentra_parent_agent_protocol::activity::policy::PolicyDecision;
use ocentra_parent_agent_protocol::activity::policy::PolicyDecisionHandoffState;
use ocentra_parent_agent_protocol::activity::policy::PolicyTarget;
use ocentra_parent_agent_protocol::activity::policy::PolicyTargetType;
use ocentra_parent_agent_protocol::activity::policy_context::ParentDeviceReference;
use ocentra_parent_agent_protocol::constants::enforcement;
use ocentra_parent_agent_protocol::enforcement::{
    EnforcementAdapterKind, EnforcementCapabilityState, EnforcementCapabilityStatus,
    EnforcementDependencyState, EnforcementIntent, EnforcementIntentSource,
    EnforcementPermissionState, EnforcementUnavailableReason, ParentPlatform,
};
use ocentra_parent_agent_protocol::policy_constants as policy;

#[test]
fn permission_and_dependency_unavailable_capabilities_stop_before_adapter() -> TestResult {
    for (capability, expected_reason) in [
        (
            unavailable_capability(
                EnforcementPermissionState::MissingPermission,
                EnforcementDependencyState::Installed,
                enforcement::UNAVAILABLE_MISSING_PERMISSION,
            ),
            EnforcementUnavailableReason::MissingPermission,
        ),
        (
            unavailable_capability(
                EnforcementPermissionState::Allowed,
                EnforcementDependencyState::Missing,
                enforcement::UNAVAILABLE_MISSING_DEPENDENCY,
            ),
            EnforcementUnavailableReason::MissingDependency,
        ),
    ] {
        let expected_reason_text = expected_reason.as_protocol_str();
        let input = boundary_input(policy_decision(false), capability);
        let outcome = ok(evaluate_enforcement_boundary(input), expected_reason_text)?;
        let unavailable_status = some(
            outcome.result.unavailable_status.as_ref(),
            expected_reason_text,
        )?;
        let timer = some(outcome.timer_event.as_ref(), enforcement::TIMER_UNAVAILABLE)?;

        assert_eq!(
            outcome.result.status.as_protocol_str(),
            enforcement::RESULT_UNAVAILABLE
        );
        assert_eq!(
            outcome.result.adapter_result_code.as_protocol_str(),
            enforcement::ADAPTER_UNAVAILABLE
        );
        assert_eq!(
            outcome.result.unavailable_reason.as_deref(),
            Some(expected_reason_text)
        );
        assert_eq!(unavailable_status.unavailable_reason, expected_reason);
        assert!(!unavailable_status.retryable);
        assert_eq!(
            outcome
                .audit_event
                .unavailable_status
                .as_ref()
                .map(|status| status.unavailable_reason),
            Some(expected_reason)
        );
        assert_eq!(timer.unavailable_reason, Some(expected_reason));
        assert_eq!(outcome.adapter_request, None);
    }

    Ok(())
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

fn unavailable_capability(
    permission_state: EnforcementPermissionState,
    dependency_state: EnforcementDependencyState,
    reason: impl std::fmt::Display,
) -> EnforcementCapabilityStatus {
    EnforcementCapabilityStatus {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        platform: ParentPlatform::Windows,
        adapter_kind: EnforcementAdapterKind::ProcessControl,
        capability_state: EnforcementCapabilityState::Unavailable,
        permission_state,
        dependency_state,
        supported_actions: Vec::new(),
        degraded_reason: Some(reason.to_string()),
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
