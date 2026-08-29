use crate::*;
use ocentra_parent_agent_core::enforcement_timer_state::{
    active_timer_state_from_outcome, active_timer_state_is_consistent, cancelled_timer_outcome,
    expired_timer_outcome, restart_recovered_timer_outcome, EnforcementTimerTransitionIds,
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
    EnforcementActiveTimerState, EnforcementAdapterKind, EnforcementAdapterResultCode,
    EnforcementAuditEventKind, EnforcementCapabilityState, EnforcementCapabilityStatus,
    EnforcementDependencyState, EnforcementIntent, EnforcementIntentSource, EnforcementMode,
    EnforcementPermissionState, EnforcementResultStatus, EnforcementRollbackState,
    EnforcementTimerEventKind, ParentActionReference, ParentPlatform,
};
use ocentra_parent_agent_protocol::policy_constants as policy;

use crate::test_text::{test_ok as ok, test_some as some, TestResult, TestText};
use ocentra_parent_agent_core::enforcement_adapter::EnforcementAdapterOutcome;

#[test]
fn active_timer_state_recovers_and_cancels_with_original_identity() -> TestResult {
    let outcome = ok(
        evaluate_enforcement_boundary(boundary_input()),
        enforcement::TEST_TIMER_EVENT_ID,
    )?;
    let state = some(
        active_timer_state_from_outcome(&outcome, policy::TEST_EVALUATED_AT),
        enforcement::TEST_TIMER_STATE_ID,
    )?;

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
        recovered_timer.evidence_references,
        outcome.action.evidence_references
    );
    assert_eq!(
        recovered_timer.rollback_token,
        outcome.action.rollback_token
    );
    assert_eq!(recovered.result.status, EnforcementResultStatus::NoOp);
    assert_eq!(
        some(
            active_timer_state_from_outcome(&recovered, policy::TEST_EVALUATED_AT),
            enforcement::TEST_TIMER_STATE_ID,
        )?
        .state_id,
        state.state_id
    );

    let cancelled = cancelled_timer_outcome(&state, transition_ids(), parent_action_reference());
    let cancelled_timer = some(
        cancelled.timer_event.as_ref(),
        enforcement::TEST_TIMER_EVENT_ID,
    )?;
    assert_eq!(
        cancelled.audit_event.audit_event_kind,
        EnforcementAuditEventKind::Cancelled
    );
    assert_eq!(
        cancelled_timer.timer_event_kind,
        EnforcementTimerEventKind::Cancelled
    );
    assert_eq!(cancelled.result.status, EnforcementResultStatus::Superseded);
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
fn persisted_recovery_state_remains_consistent_across_restart_cycles() -> TestResult {
    let outcome = ok(
        evaluate_enforcement_boundary(boundary_input()),
        enforcement::TEST_TIMER_EVENT_ID,
    )?;
    let state = some(
        active_timer_state_from_outcome(&outcome, policy::TEST_EVALUATED_AT),
        enforcement::TEST_TIMER_STATE_ID,
    )?;

    let recovered = restart_recovered_timer_outcome(
        &state,
        transition_ids_at("2026-05-20T20:50:00.000Z", "-recovered"),
    );
    let recovered_state = some(
        active_timer_state_from_outcome(&recovered, "2026-05-20T20:50:00.000Z"),
        enforcement::TEST_TIMER_STATE_ID,
    )?;
    let recovered_serialized = ok(
        serde_json::to_string(&recovered_state),
        enforcement::TEST_TIMER_STATE_ID,
    )?;
    let recovered_state: EnforcementActiveTimerState = ok(
        serde_json::from_str(&recovered_serialized),
        enforcement::TEST_TIMER_STATE_ID,
    )?;
    assert!(active_timer_state_is_consistent(&recovered_state));
    assert_eq!(
        recovered_state.result.started_at,
        "2026-05-20T20:50:00.000Z"
    );

    let recovered_again = restart_recovered_timer_outcome(
        &recovered_state,
        transition_ids_at("2026-05-20T20:55:00.000Z", "-recovered-again"),
    );
    let recovered_again_state = some(
        active_timer_state_from_outcome(&recovered_again, "2026-05-20T20:55:00.000Z"),
        enforcement::TEST_TIMER_STATE_ID,
    )?;
    assert!(active_timer_state_is_consistent(&recovered_again_state));
    assert_eq!(
        recovered_again_state.result.started_at,
        "2026-05-20T20:55:00.000Z"
    );

    Ok(())
}

#[test]
fn persisted_active_timer_state_rejects_cross_record_identity_mutation() -> TestResult {
    let state = active_state()?;
    assert!(active_timer_state_is_consistent(&state));

    let mut mismatched_result = state.clone();
    mismatched_result.result.action_id.push("-other");
    assert!(!active_timer_state_is_consistent(&mismatched_result));

    let mut mismatched_timer = state;
    mismatched_timer.timer_event.action_id.push("-other");
    assert!(!active_timer_state_is_consistent(&mismatched_timer));

    Ok(())
}

#[test]
fn persisted_active_timer_state_rejects_corrupt_clock_relationships() -> TestResult {
    let state = active_state()?;
    assert!(active_timer_state_is_consistent(&state));

    let mut malformed_requested_at = state.clone();
    malformed_requested_at.action.requested_at = "not-a-timestamp".to_string();
    assert!(!active_timer_state_is_consistent(&malformed_requested_at));

    let mut missing_expiry = state.clone();
    missing_expiry.action.expires_at = None;
    assert!(!active_timer_state_is_consistent(&missing_expiry));

    let mut inverted_expiry = state.clone();
    inverted_expiry.action.expires_at = Some(policy::TEST_EVALUATED_AT.to_string());
    assert!(!active_timer_state_is_consistent(&inverted_expiry));

    let mut mismatched_schedule = state.clone();
    mismatched_schedule.timer_event.scheduled_at = policy::TEST_EXPIRES_AT.to_string();
    assert!(!active_timer_state_is_consistent(&mismatched_schedule));

    let mut malformed_stored_at = state;
    malformed_stored_at.stored_at = "not-a-timestamp".to_string();
    assert!(!active_timer_state_is_consistent(&malformed_stored_at));

    Ok(())
}

#[test]
fn expiry_transition_clears_active_state() -> TestResult {
    let expired = expired_timer_outcome(
        &active_state()?,
        transition_ids(),
        adapter_outcome(
            EnforcementResultStatus::Expired,
            EnforcementAdapterResultCode::TimerExpired,
            None,
            None,
            EnforcementRollbackState::NotRequired,
        ),
    );
    let expired_timer = some(
        expired.timer_event.as_ref(),
        enforcement::TEST_TIMER_EVENT_ID,
    )?;
    assert_eq!(
        expired_timer.timer_event_kind,
        EnforcementTimerEventKind::Expired
    );
    assert_eq!(expired.result.status, EnforcementResultStatus::Expired);
    assert_eq!(
        expired.audit_event.audit_event_kind,
        EnforcementAuditEventKind::Expired
    );
    assert!(expired.result.next_check_at.is_none());
    assert!(active_timer_state_from_outcome(&expired, policy::TEST_EVALUATED_AT).is_none());

    Ok(())
}

#[test]
fn expiry_transition_surfaces_rollback_completed_state() -> TestResult {
    let rollback_completed = expired_timer_outcome(
        &active_state()?,
        transition_ids(),
        adapter_outcome(
            EnforcementResultStatus::RolledBack,
            EnforcementAdapterResultCode::RollbackCompleted,
            None,
            None,
            EnforcementRollbackState::Completed,
        ),
    );
    let rollback_timer = some(
        rollback_completed.timer_event.as_ref(),
        enforcement::TEST_TIMER_EVENT_ID,
    )?;
    assert_eq!(
        rollback_timer.timer_event_kind,
        EnforcementTimerEventKind::RollbackCompleted
    );
    assert_eq!(
        rollback_completed.audit_event.audit_event_kind,
        EnforcementAuditEventKind::RollbackCompleted
    );
    assert_eq!(
        rollback_completed.result.rollback_state,
        EnforcementRollbackState::Completed
    );
    assert!(rollback_completed.result.next_check_at.is_none());
    assert!(
        active_timer_state_from_outcome(&rollback_completed, policy::TEST_EVALUATED_AT).is_none()
    );

    Ok(())
}

#[test]
fn expiry_transition_surfaces_rollback_unavailable_state() -> TestResult {
    let rollback_unavailable = expired_timer_outcome(
        &active_state()?,
        transition_ids(),
        adapter_outcome(
            EnforcementResultStatus::Unavailable,
            EnforcementAdapterResultCode::AdapterUnavailable,
            Some(TestText::from_display(
                enforcement::UNAVAILABLE_ADAPTER_UNAVAILABLE,
            )),
            None,
            EnforcementRollbackState::Unavailable,
        ),
    );
    let unavailable_timer = some(
        rollback_unavailable.timer_event.as_ref(),
        enforcement::TEST_TIMER_EVENT_ID,
    )?;
    assert_eq!(
        unavailable_timer.timer_event_kind,
        EnforcementTimerEventKind::Unavailable
    );
    assert_eq!(
        unavailable_timer
            .unavailable_reason
            .as_ref()
            .map(|reason| reason.as_protocol_str()),
        Some(enforcement::UNAVAILABLE_ADAPTER_UNAVAILABLE)
    );
    assert_eq!(
        rollback_unavailable.audit_event.audit_event_kind,
        EnforcementAuditEventKind::Unavailable
    );
    assert_eq!(
        rollback_unavailable
            .result
            .unavailable_status
            .as_ref()
            .map(|status| (
                status.unavailable_reason.as_protocol_str(),
                status.retryable,
                status.checked_at.as_str(),
            )),
        Some((
            enforcement::UNAVAILABLE_ADAPTER_UNAVAILABLE,
            true,
            policy::TEST_EVALUATED_AT,
        ))
    );
    assert!(
        active_timer_state_from_outcome(&rollback_unavailable, policy::TEST_EVALUATED_AT).is_none()
    );

    Ok(())
}

#[test]
fn failed_expiry_transition_requests_recovery() -> TestResult {
    let recovery_needed = expired_timer_outcome(
        &active_state()?,
        transition_ids(),
        adapter_outcome(
            EnforcementResultStatus::Failed,
            EnforcementAdapterResultCode::AdapterFailed,
            None,
            Some(TestText::from_display(enforcement::ADAPTER_FAILED)),
            EnforcementRollbackState::Failed,
        ),
    );
    let recovery_timer = some(
        recovery_needed.timer_event.as_ref(),
        enforcement::TEST_TIMER_EVENT_ID,
    )?;
    assert_eq!(
        recovery_timer.timer_event_kind,
        EnforcementTimerEventKind::RecoveryNeeded
    );
    assert_eq!(
        recovery_timer
            .unavailable_reason
            .as_ref()
            .map(|reason| reason.as_protocol_str()),
        Some(enforcement::UNAVAILABLE_ADAPTER_ERROR)
    );
    assert_eq!(
        recovery_needed.audit_event.audit_event_kind,
        EnforcementAuditEventKind::Failed
    );
    assert_eq!(
        recovery_needed.result.failed_reason.as_deref(),
        Some(enforcement::ADAPTER_FAILED)
    );
    assert!(active_timer_state_from_outcome(&recovery_needed, policy::TEST_EVALUATED_AT).is_none());

    Ok(())
}

fn active_state() -> Result<EnforcementActiveTimerState, TestText> {
    let outcome = ok(
        evaluate_enforcement_boundary(boundary_input()),
        enforcement::TEST_TIMER_EVENT_ID,
    )?;
    some(
        active_timer_state_from_outcome(&outcome, policy::TEST_EVALUATED_AT),
        enforcement::TEST_TIMER_STATE_ID,
    )
}

fn adapter_outcome(
    status: EnforcementResultStatus,
    adapter_result_code: EnforcementAdapterResultCode,
    unavailable_reason: Option<TestText>,
    failed_reason: Option<TestText>,
    rollback_state: EnforcementRollbackState,
) -> EnforcementAdapterOutcome {
    EnforcementAdapterOutcome {
        status,
        adapter_result_code,
        completed_at: Some(policy::TEST_EVALUATED_AT.to_string()),
        unavailable_reason: unavailable_reason.map(|reason| reason.to_string()),
        failed_reason: failed_reason.map(|reason| reason.to_string()),
        rollback_token: Some(enforcement::TEST_ROLLBACK_TOKEN.to_string()),
        rollback_state,
    }
}

fn boundary_input() -> EnforcementBoundaryInput {
    EnforcementBoundaryInput {
        intent: intent(),
        decision: policy_decision(),
        capability: timer_capability(),
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

fn transition_ids() -> EnforcementTimerTransitionIds {
    transition_ids_at(policy::TEST_EVALUATED_AT, "")
}

fn transition_ids_at(observed_at: &str, suffix: &str) -> EnforcementTimerTransitionIds {
    EnforcementTimerTransitionIds {
        result_id: format!("{}{}", enforcement::TEST_RESULT_ID, suffix),
        audit_event_id: format!("{}{}", enforcement::TEST_AUDIT_EVENT_ID, suffix),
        timer_event_id: format!("{}{}", enforcement::TEST_TIMER_EVENT_ID, suffix),
        observed_at: observed_at.to_string(),
    }
}

fn intent() -> EnforcementIntent {
    EnforcementIntent {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        intent_id: enforcement::TEST_INTENT_ID.to_string(),
        source: EnforcementIntentSource::ParentPortal,
        actor: Some(parent_actor()),
        device: device(),
        policy_decision_id: policy::TEST_DECISION_ID.to_string(),
        target: target(),
        requested_action: PolicyAction::AskParent,
        evidence_references: vec![evidence()],
        parent_approval: None,
        idempotency_key: enforcement::TEST_IDEMPOTENCY_KEY.to_string(),
    }
}

fn policy_decision() -> PolicyDecision {
    PolicyDecision {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        decision_id: policy::TEST_DECISION_ID.to_string(),
        action: PolicyAction::AskParent,
        reason_codes: vec![policy::TEST_REASON_PARENT_BLOCK.to_string()],
        evidence_references: vec![evidence()],
        rule_ids: vec![policy::TEST_BLOCK_RULE_ID.to_string()],
        local_ai_result_id: None,
        dry_run: false,
        enforcement_handoff_state: PolicyDecisionHandoffState::HandedOff,
        expires_at: Some(policy::TEST_EXPIRES_AT.to_string()),
    }
}

fn timer_capability() -> EnforcementCapabilityStatus {
    EnforcementCapabilityStatus {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        platform: ParentPlatform::Windows,
        adapter_kind: EnforcementAdapterKind::TimerControl,
        capability_state: EnforcementCapabilityState::Supported,
        permission_state: EnforcementPermissionState::NotRequired,
        dependency_state: EnforcementDependencyState::Installed,
        supported_actions: vec![EnforcementMode::AskParent],
        degraded_reason: None,
        last_checked_at: policy::TEST_EVALUATED_AT.to_string(),
    }
}

fn parent_action_reference() -> ParentActionReference {
    ParentActionReference {
        action_reference_id: enforcement::TEST_PARENT_ACTION_REFERENCE_ID.to_string(),
        actor: parent_actor(),
        policy_version: policy::TEST_POLICY_VERSION.to_string(),
        created_at: policy::TEST_EVALUATED_AT.to_string(),
    }
}

fn parent_actor() -> ParentActorReference {
    ParentActorReference {
        actor_id: policy::TEST_PARENT_ACTOR_ID.to_string(),
        role: ParentActorRole::Parent,
    }
}

fn target() -> PolicyTarget {
    PolicyTarget {
        target_id: enforcement::TEST_PROCESS_TARGET_ID.to_string(),
        target_type: PolicyTargetType::Device,
        target_value: enforcement::TEST_CHILD_DEVICE_ID.to_string(),
    }
}

fn evidence() -> ParentEvidenceReference {
    ParentEvidenceReference {
        evidence_reference_id: policy::TEST_EVIDENCE_ID.to_string(),
        kind: ParentEvidenceReferenceKind::PolicyDecision,
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
