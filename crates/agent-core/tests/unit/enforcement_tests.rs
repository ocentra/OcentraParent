use std::fmt::Debug;
#[cfg(windows)]
use std::process::{Child, Command, Stdio};

use crate::test_text::{TestResult, TestText};
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
    EnforcementAdapterKind, EnforcementAdapterResultCode, EnforcementCapabilityState,
    EnforcementCapabilityStatus, EnforcementDependencyState, EnforcementIntent,
    EnforcementIntentSource, EnforcementMode, EnforcementPermissionState, EnforcementResultStatus,
    EnforcementRollbackState, ParentPlatform,
};
use ocentra_parent_agent_protocol::policy_constants as policy;

fn ok<T, E: Debug>(result: Result<T, E>, context: impl std::fmt::Display) -> Result<T, TestText> {
    result.map_err(|error| TestText::from_display(format!("{}: {error:?}", context)))
}

fn err<T, E: Debug>(result: Result<T, E>, context: impl std::fmt::Display) -> Result<E, TestText> {
    match result {
        Ok(_) => Err(TestText::from_display(format!("{context}: expected error"))),
        Err(error) => Ok(error),
    }
}

fn some<T>(value: Option<T>, context: impl std::fmt::Display) -> Result<T, TestText> {
    value.ok_or_else(|| TestText::from_display(context))
}

#[test]
fn dry_run_decision_never_requests_adapter_execution() -> TestResult {
    let input = boundary_input(policy_decision(true), supported_capability());

    let outcome = ok(
        evaluate_enforcement_boundary(input),
        policy::TEST_DECISION_ID,
    )?;

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
            .as_ref()
            .ok_or_else(|| TestText::from_display(policy::TEST_EXPIRES_AT))?
            .timer_event_kind
            .as_protocol_str(),
        enforcement::TIMER_CREATED
    );
    let authorization = ok(
        authorize_enforcement_boundary(boundary_input(
            policy_decision(true),
            supported_capability(),
        )),
        enforcement::ADAPTER_DRY_RUN_NO_ACTION,
    )?;

    assert!(authorization.action.dry_run);
    assert_eq!(authorization.adapter_request, None);

    Ok(())
}

#[test]
fn mismatched_policy_decision_id_rejects_before_action_building() -> TestResult {
    let mut input = boundary_input(policy_decision(true), supported_capability());
    input.intent.policy_decision_id = enforcement::TEST_RESULT_ID.to_string();

    let rejected = err(
        evaluate_enforcement_boundary(input),
        enforcement::REJECTION_DECISION_ID_MISMATCH,
    )?;

    assert_eq!(
        rejected.as_protocol_str(),
        enforcement::REJECTION_DECISION_ID_MISMATCH
    );

    Ok(())
}

#[test]
fn missing_policy_evidence_rejects_before_adapter_path() -> TestResult {
    let mut decision = policy_decision(false);
    decision.evidence_references = Vec::new();
    let input = boundary_input(decision, supported_capability());

    let rejected = err(
        evaluate_enforcement_boundary(input),
        enforcement::REJECTION_MISSING_EVIDENCE,
    )?;

    assert_eq!(
        rejected.as_protocol_str(),
        enforcement::REJECTION_MISSING_EVIDENCE
    );

    Ok(())
}

#[test]
fn unavailable_capability_returns_auditable_unavailable_result() -> TestResult {
    let input = boundary_input(policy_decision(false), unavailable_capability());

    let outcome = ok(
        evaluate_enforcement_boundary(input),
        enforcement::ADAPTER_UNAVAILABLE,
    )?;

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
    let unavailable_status = some(
        outcome.result.unavailable_status.as_ref(),
        enforcement::UNAVAILABLE_UNSUPPORTED_PLATFORM,
    )?;
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
            .ok_or_else(|| TestText::from_display(enforcement::UNAVAILABLE_UNSUPPORTED_PLATFORM))?
            .unavailable_reason
            .as_protocol_str(),
        enforcement::UNAVAILABLE_UNSUPPORTED_PLATFORM
    );
    assert_eq!(
        outcome.audit_event.audit_event_kind.as_protocol_str(),
        enforcement::AUDIT_UNAVAILABLE
    );
    assert_eq!(outcome.adapter_request, None);

    Ok(())
}

#[test]
fn unsupported_action_returns_typed_unavailable_status_without_adapter_execution() -> TestResult {
    let input = boundary_input(policy_decision(false), unsupported_action_capability());

    let outcome = ok(
        evaluate_enforcement_boundary(input),
        enforcement::ADAPTER_UNAVAILABLE,
    )?;

    assert_eq!(
        outcome.result.status.as_protocol_str(),
        enforcement::RESULT_UNAVAILABLE
    );
    assert_eq!(
        outcome
            .result
            .unavailable_status
            .as_ref()
            .ok_or_else(|| TestText::from_display(enforcement::UNAVAILABLE_UNSUPPORTED_ACTION))?
            .unavailable_reason
            .as_protocol_str(),
        enforcement::UNAVAILABLE_UNSUPPORTED_ACTION
    );
    assert_eq!(
        outcome.result.unavailable_reason.as_deref(),
        Some(enforcement::UNAVAILABLE_UNSUPPORTED_ACTION)
    );
    assert_eq!(outcome.adapter_request, None);

    Ok(())
}

#[test]
fn manual_required_network_and_browser_targets_return_unavailable_audit_without_adapter_execution(
) -> TestResult {
    for (target_type, capability, expected_adapter_kind) in [
        (
            PolicyTargetType::Domain,
            network_control_capability(policy::TEST_EVALUATED_AT),
            EnforcementAdapterKind::NetworkControl,
        ),
        (
            PolicyTargetType::Site,
            managed_browser_control_capability(policy::TEST_EVALUATED_AT),
            EnforcementAdapterKind::ManagedBrowserControl,
        ),
    ] {
        let mut input = boundary_input(policy_decision(false), capability);
        input.intent.target.target_type = target_type;
        input.intent.target.target_value = enforcement::TEST_PROCESS_TARGET_VALUE.to_string();
        let outcome = ok(
            evaluate_enforcement_boundary(input),
            enforcement::UNAVAILABLE_MANUAL_REQUIRED,
        )?;

        assert_eq!(outcome.action.adapter_kind, expected_adapter_kind);
        assert_eq!(
            outcome.result.status.as_protocol_str(),
            enforcement::RESULT_UNAVAILABLE
        );
        #[cfg(windows)]
        assert_eq!(
            outcome
                .result
                .unavailable_status
                .as_ref()
                .map(|status| status.unavailable_reason.as_protocol_str()),
            Some(enforcement::UNAVAILABLE_MANUAL_REQUIRED)
        );
        assert_eq!(outcome.adapter_request, None);
    }

    Ok(())
}

#[test]
fn supported_non_dry_run_requires_adapter_outcome_for_process_control() -> TestResult {
    let input = boundary_input(policy_decision(false), supported_capability());

    let rejected = err(
        evaluate_enforcement_boundary(input),
        enforcement::REJECTION_ADAPTER_RESULT_REQUIRED,
    )?;

    assert_eq!(
        rejected.as_protocol_str(),
        enforcement::REJECTION_ADAPTER_RESULT_REQUIRED
    );

    let authorization = ok(
        authorize_enforcement_boundary(boundary_input(
            policy_decision(false),
            supported_capability(),
        )),
        enforcement::ADAPTER_PROCESS_TERMINATED,
    )?;

    assert!(!authorization.action.dry_run);
    assert_eq!(
        authorization
            .adapter_request
            .as_ref()
            .ok_or_else(|| TestText::from_display(enforcement::TEST_ACTION_ID))?
            .mode
            .as_protocol_str(),
        enforcement::MODE_TERMINATE_PROCESS
    );

    Ok(())
}

#[test]
fn adapter_outcome_maps_to_success_result_and_audit() -> TestResult {
    let mut input = boundary_input(policy_decision(false), supported_capability());
    input.adapter_outcome = Some(EnforcementAdapterOutcome {
        status: EnforcementResultStatus::ActuallyEnforced,
        adapter_result_code: EnforcementAdapterResultCode::ProcessTerminated,
        completed_at: Some(policy::TEST_EVALUATED_AT.to_string()),
        unavailable_reason: None,
        failed_reason: None,
        rollback_token: Some(enforcement::TEST_ROLLBACK_TOKEN.to_string()),
        rollback_state: EnforcementRollbackState::Available,
    });

    let outcome = ok(
        evaluate_enforcement_boundary(input),
        enforcement::ADAPTER_PROCESS_TERMINATED,
    )?;

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

    Ok(())
}

#[test]
fn process_adapter_reports_real_platform_result_with_explicit_rollback_state() -> TestResult {
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

    Ok(())
}

#[cfg(windows)]
struct OwnedProcessGuard(Child);

#[cfg(windows)]
impl Drop for OwnedProcessGuard {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

#[cfg(windows)]
#[test]
fn owned_process_adapter_terminates_a_real_owned_windows_process() -> TestResult {
    let child = Command::new("powershell.exe")
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-Command",
            "Start-Sleep -Seconds 60",
        ])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|error| TestText::from_display(format!("spawn owned process: {error}")))?;
    let mut child = OwnedProcessGuard(child);

    let outcome = terminate_owned_process(
        OwnedProcessTerminationTarget {
            pid: child.0.id(),
            expected_process_name: "powershell.exe".to_string(),
        },
        policy::TEST_EVALUATED_AT,
    );

    assert_eq!(
        outcome.adapter_result_code.as_protocol_str(),
        enforcement::ADAPTER_PROCESS_TERMINATED
    );
    assert_eq!(outcome.status, EnforcementResultStatus::ActuallyEnforced);
    assert_eq!(
        outcome.rollback_state.as_protocol_str(),
        enforcement::ROLLBACK_NOT_REQUIRED
    );
    let exit = child
        .0
        .wait()
        .map_err(|error| TestText::from_display(format!("wait owned process: {error}")))?;
    assert!(!exit.success());

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
