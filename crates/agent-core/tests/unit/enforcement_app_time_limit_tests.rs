use crate::test_text::{test_err as err, test_ok as ok, test_some as some, TestResult, TestText};
use crate::*;
use ocentra_parent_agent_core::enforcement_timer_state::{
    active_timer_state_from_outcome, cancelled_timer_outcome, expired_timer_outcome,
    restart_recovered_timer_outcome, EnforcementTimerTransitionIds,
};
use ocentra_parent_agent_protocol::activity::policy::ParentActorReference;
use ocentra_parent_agent_protocol::activity::policy::ParentActorRole;
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
    EnforcementAdapterKind, EnforcementAdapterResultCode, EnforcementAuditEventKind,
    EnforcementCapabilityState, EnforcementCapabilityStatus, EnforcementDependencyState,
    EnforcementIntent, EnforcementIntentSource, EnforcementMode, EnforcementPermissionState,
    EnforcementResultStatus, EnforcementRollbackState, EnforcementTimerEventKind,
    EnforcementUnavailableReason, ParentActionReference, ParentPlatform,
};
use ocentra_parent_agent_protocol::policy_constants as policy;

#[test]
fn app_time_limit_capability_reports_platform_status_without_claiming_other_adapters() -> TestResult
{
    let capability = app_time_limit_capability(policy::TEST_EVALUATED_AT);

    assert_eq!(
        capability.adapter_kind,
        EnforcementAdapterKind::ProcessControl
    );
    assert_eq!(
        capability.permission_state,
        EnforcementPermissionState::NotRequired
    );

    #[cfg(windows)]
    {
        assert_eq!(
            capability.capability_state,
            EnforcementCapabilityState::Supported
        );
        assert_eq!(
            capability.dependency_state,
            EnforcementDependencyState::Installed
        );
        assert_eq!(
            capability.supported_actions,
            vec![EnforcementMode::TimeLimit]
        );
        assert_eq!(capability.degraded_reason, None);
    }

    #[cfg(not(windows))]
    {
        assert_eq!(
            capability.capability_state,
            EnforcementCapabilityState::Unavailable
        );
        assert_eq!(
            capability.dependency_state,
            EnforcementDependencyState::NotRequired
        );
        assert_eq!(capability.supported_actions, Vec::new());
        assert_eq!(
            capability.degraded_reason.as_deref(),
            Some(enforcement::UNAVAILABLE_UNSUPPORTED_PLATFORM)
        );
    }

    Ok(())
}

#[test]
fn app_time_limit_policy_handoff_creates_timer_state_without_immediate_adapter_execution(
) -> TestResult {
    let input = boundary_input(time_limit_decision(), supported_time_limit_capability());
    let authorized = ok(
        authorize_enforcement_boundary(input.clone()),
        enforcement::MODE_TIME_LIMIT,
    )?;

    assert_eq!(authorized.action.mode, EnforcementMode::TimeLimit);
    assert_eq!(
        authorized.action.adapter_kind,
        EnforcementAdapterKind::ProcessControl
    );
    assert_eq!(authorized.adapter_request, None);

    let target = ok(
        app_time_limit_target_from_action(&authorized.action, Some(42)),
        enforcement::TEST_PROCESS_TARGET_ID,
    )?;
    assert_eq!(target.pid, 42);
    assert_eq!(
        target.expected_process_name.as_str(),
        enforcement::TEST_PROCESS_TARGET_VALUE
    );

    let outcome = ok(
        evaluate_enforcement_boundary(input),
        enforcement::TIMER_CREATED,
    )?;
    let timer = some(
        outcome.timer_event.as_ref(),
        enforcement::TEST_TIMER_EVENT_ID,
    )?;
    assert_eq!(outcome.result.status, EnforcementResultStatus::NoOp);
    assert_eq!(timer.timer_event_kind, EnforcementTimerEventKind::Created);
    assert_eq!(
        active_timer_state_from_outcome(&outcome, policy::TEST_EVALUATED_AT)
            .ok_or_else(|| TestText::from_display(enforcement::TEST_TIMER_STATE_ID))?
            .action
            .policy_decision_id,
        policy::TEST_DECISION_ID
    );

    Ok(())
}

#[test]
fn app_time_limit_target_validation_rejects_unowned_or_unsupported_targets() -> TestResult {
    let action = ok(
        evaluate_enforcement_boundary(boundary_input(
            time_limit_decision(),
            supported_time_limit_capability(),
        )),
        enforcement::TIMER_CREATED,
    )?
    .action;

    let missing_process = err(
        app_time_limit_target_from_action(&action, None),
        enforcement::REJECTION_PROCESS_ID_REQUIRED,
    )?;
    assert_eq!(
        missing_process.as_protocol_str(),
        enforcement::REJECTION_PROCESS_ID_REQUIRED
    );

    let mut unsupported_mode = action.clone();
    unsupported_mode.mode = EnforcementMode::TerminateProcess;
    let rejected_mode = err(
        app_time_limit_target_from_action(&unsupported_mode, Some(42)),
        enforcement::REJECTION_UNSUPPORTED_CAPABILITY,
    )?;
    assert_eq!(
        rejected_mode.as_protocol_str(),
        enforcement::REJECTION_UNSUPPORTED_CAPABILITY
    );

    let mut device_target = action;
    device_target.target.target_type = PolicyTargetType::Device;
    let rejected_target = err(
        app_time_limit_target_from_action(&device_target, Some(42)),
        enforcement::REJECTION_TARGET_MISMATCH,
    )?;
    assert_eq!(
        rejected_target.as_protocol_str(),
        enforcement::REJECTION_TARGET_MISMATCH
    );

    Ok(())
}

#[test]
fn app_time_limit_expiry_cancel_and_restart_preserve_audit_identity() -> TestResult {
    let outcome = ok(
        evaluate_enforcement_boundary(boundary_input(
            time_limit_decision(),
            supported_time_limit_capability(),
        )),
        enforcement::TIMER_CREATED,
    )?;
    let state = active_timer_state_from_outcome(&outcome, policy::TEST_EVALUATED_AT)
        .ok_or_else(|| TestText::from_display(enforcement::TEST_TIMER_STATE_ID))?;

    let recovered = restart_recovered_timer_outcome(&state, transition_ids());
    let recovered_timer = some(
        recovered.timer_event.as_ref(),
        enforcement::TEST_TIMER_EVENT_ID,
    )?;
    assert_eq!(
        recovered_timer.timer_event_kind,
        EnforcementTimerEventKind::RestartRecovered
    );
    assert_eq!(recovered_timer.action_id, outcome.action.action_id);
    assert_eq!(
        recovered_timer.policy_decision_id,
        outcome.action.policy_decision_id
    );
    assert_eq!(
        active_timer_state_from_outcome(&recovered, policy::TEST_EVALUATED_AT)
            .ok_or_else(|| TestText::from_display(enforcement::TEST_TIMER_STATE_ID))?
            .action
            .action_id,
        outcome.action.action_id
    );

    let expired = expired_timer_outcome(&state, transition_ids(), expired_adapter_outcome());
    let expired_timer = some(
        expired.timer_event.as_ref(),
        enforcement::TEST_TIMER_EVENT_ID,
    )?;
    assert_eq!(expired.result.status, EnforcementResultStatus::Expired);
    assert_eq!(
        expired.result.adapter_result_code,
        EnforcementAdapterResultCode::ProcessTerminated
    );
    assert_eq!(
        expired.audit_event.audit_event_kind,
        EnforcementAuditEventKind::Expired
    );
    assert_eq!(
        expired_timer.timer_event_kind,
        EnforcementTimerEventKind::Expired
    );
    assert_eq!(expired_timer.action_id, outcome.action.action_id);
    assert_eq!(
        expired_timer.policy_decision_id,
        outcome.action.policy_decision_id
    );
    assert_eq!(
        expired_timer.evidence_references,
        outcome.action.evidence_references
    );
    assert_eq!(expired_timer.rollback_token, outcome.action.rollback_token);
    assert!(active_timer_state_from_outcome(&expired, policy::TEST_EVALUATED_AT).is_none());

    let cancelled = cancelled_timer_outcome(&state, transition_ids(), parent_action_reference());
    assert_eq!(cancelled.result.status, EnforcementResultStatus::Superseded);
    assert_eq!(
        cancelled.audit_event.audit_event_kind,
        EnforcementAuditEventKind::Cancelled
    );
    assert_eq!(
        cancelled
            .audit_event
            .parent_override
            .as_ref()
            .map(|reference| reference.action_reference_id.as_str()),
        Some(enforcement::TEST_PARENT_ACTION_REFERENCE_ID)
    );
    assert!(active_timer_state_from_outcome(&cancelled, policy::TEST_EVALUATED_AT).is_none());

    Ok(())
}

#[test]
fn app_time_limit_unavailable_expiry_reports_typed_unavailable_reason() -> TestResult {
    let outcome = ok(
        evaluate_enforcement_boundary(boundary_input(
            time_limit_decision(),
            supported_time_limit_capability(),
        )),
        enforcement::TIMER_CREATED,
    )?;
    let state = active_timer_state_from_outcome(&outcome, policy::TEST_EVALUATED_AT)
        .ok_or_else(|| TestText::from_display(enforcement::TEST_TIMER_STATE_ID))?;
    let unavailable = expired_timer_outcome(
        &state,
        transition_ids(),
        unavailable_app_time_limit_outcome(
            EnforcementUnavailableReason::UnsupportedPlatform,
            policy::TEST_EVALUATED_AT,
        ),
    );
    let unavailable_timer = some(
        unavailable.timer_event.as_ref(),
        enforcement::TEST_TIMER_EVENT_ID,
    )?;

    assert_eq!(
        unavailable.result.status,
        EnforcementResultStatus::Unavailable
    );
    assert_eq!(
        unavailable.result.unavailable_reason.as_deref(),
        Some(enforcement::UNAVAILABLE_UNSUPPORTED_PLATFORM)
    );
    assert_eq!(
        unavailable
            .result
            .unavailable_status
            .as_ref()
            .map(|status| status.unavailable_reason.as_protocol_str()),
        Some(enforcement::UNAVAILABLE_UNSUPPORTED_PLATFORM)
    );
    assert_eq!(
        unavailable.audit_event.audit_event_kind,
        EnforcementAuditEventKind::Unavailable
    );
    assert_eq!(
        unavailable
            .audit_event
            .unavailable_status
            .as_ref()
            .map(|status| status.unavailable_reason.as_protocol_str()),
        Some(enforcement::UNAVAILABLE_UNSUPPORTED_PLATFORM)
    );
    assert_eq!(
        unavailable_timer.timer_event_kind,
        EnforcementTimerEventKind::Unavailable
    );
    assert_eq!(
        unavailable_timer
            .unavailable_reason
            .as_ref()
            .map(|reason| reason.as_protocol_str()),
        Some(enforcement::UNAVAILABLE_UNSUPPORTED_PLATFORM)
    );

    Ok(())
}

#[test]
fn app_time_limit_adapter_reports_real_platform_expiry_or_unavailable_result() -> TestResult {
    let outcome = expire_app_time_limit_for_owned_process(
        AppTimeLimitAdapterTarget {
            pid: u32::MAX,
            expected_process_name: enforcement::TEST_PROCESS_TARGET_VALUE.to_string(),
        },
        policy::TEST_EVALUATED_AT,
    );

    #[cfg(windows)]
    {
        assert_eq!(outcome.status, EnforcementResultStatus::Expired);
        assert_eq!(
            outcome.adapter_result_code,
            EnforcementAdapterResultCode::ProcessAlreadyExited
        );
        assert_eq!(
            outcome.rollback_state,
            EnforcementRollbackState::NotRequired
        );
    }

    #[cfg(not(windows))]
    {
        assert_eq!(outcome.status, EnforcementResultStatus::Unavailable);
        assert_eq!(
            outcome.adapter_result_code,
            EnforcementAdapterResultCode::UnsupportedPlatform
        );
        assert_eq!(
            outcome.rollback_state,
            EnforcementRollbackState::Unavailable
        );
        assert_eq!(
            outcome.unavailable_reason.as_deref(),
            Some(enforcement::UNAVAILABLE_UNSUPPORTED_PLATFORM)
        );
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

fn time_limit_decision() -> PolicyDecision {
    PolicyDecision {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        decision_id: policy::TEST_DECISION_ID.to_string(),
        action: PolicyAction::TimeLimit,
        reason_codes: vec![policy::TEST_REASON_PARENT_TIME_LIMIT.to_string()],
        evidence_references: vec![evidence()],
        rule_ids: vec![policy::TEST_TIME_LIMIT_RULE_ID.to_string()],
        local_ai_result_id: None,
        dry_run: false,
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

fn supported_time_limit_capability() -> EnforcementCapabilityStatus {
    EnforcementCapabilityStatus {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        platform: ParentPlatform::Windows,
        adapter_kind: EnforcementAdapterKind::ProcessControl,
        capability_state: EnforcementCapabilityState::Supported,
        permission_state: EnforcementPermissionState::NotRequired,
        dependency_state: EnforcementDependencyState::Installed,
        supported_actions: vec![EnforcementMode::TimeLimit],
        degraded_reason: None,
        last_checked_at: policy::TEST_EVALUATED_AT.to_string(),
    }
}

fn expired_adapter_outcome() -> EnforcementAdapterOutcome {
    EnforcementAdapterOutcome {
        status: EnforcementResultStatus::Expired,
        adapter_result_code: EnforcementAdapterResultCode::ProcessTerminated,
        completed_at: Some(policy::TEST_EXPIRES_AT.to_string()),
        unavailable_reason: None,
        failed_reason: None,
        rollback_token: Some(enforcement::TEST_ROLLBACK_TOKEN.to_string()),
        rollback_state: EnforcementRollbackState::NotRequired,
    }
}

fn transition_ids() -> EnforcementTimerTransitionIds {
    EnforcementTimerTransitionIds {
        result_id: enforcement::TEST_RESULT_ID.to_string(),
        audit_event_id: enforcement::TEST_AUDIT_EVENT_ID.to_string(),
        timer_event_id: enforcement::TEST_TIMER_EVENT_ID.to_string(),
        observed_at: policy::TEST_EVALUATED_AT.to_string(),
    }
}

fn parent_action_reference() -> ParentActionReference {
    ParentActionReference {
        action_reference_id: enforcement::TEST_PARENT_ACTION_REFERENCE_ID.to_string(),
        actor: ParentActorReference {
            actor_id: policy::TEST_PARENT_ACTOR_ID.to_string(),
            role: ParentActorRole::Parent,
        },
        policy_version: policy::TEST_POLICY_VERSION.to_string(),
        created_at: policy::TEST_EVALUATED_AT.to_string(),
    }
}

fn target() -> PolicyTarget {
    PolicyTarget {
        target_id: enforcement::TEST_PROCESS_TARGET_ID.to_string(),
        target_type: PolicyTargetType::App,
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
