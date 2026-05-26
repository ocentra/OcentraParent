use super::*;
use ocentra_parent_agent_protocol::{
    constants::enforcement, policy_constants as policy, EnforcementAdapterKind,
    EnforcementAuditEventKind, EnforcementCapabilityState, EnforcementCapabilityStatus,
    EnforcementDependencyState, EnforcementIntent, EnforcementIntentSource, EnforcementMode,
    EnforcementPermissionState, EnforcementResultStatus, EnforcementTimerEventKind,
    ParentActionReference, ParentActorReference, ParentActorRole, ParentDeviceReference,
    ParentEvidenceReference, ParentEvidenceReferenceKind, ParentPlatform, PolicyAction,
    PolicyDecision, PolicyDecisionHandoffState, PolicyTarget, PolicyTargetType,
};

#[test]
fn active_timer_state_recovers_and_cancels_with_original_identity() {
    let outcome =
        evaluate_enforcement_boundary(boundary_input()).expect(enforcement::TEST_TIMER_EVENT_ID);
    let state = active_timer_state_from_outcome(&outcome, policy::TEST_EVALUATED_AT)
        .expect(enforcement::TEST_TIMER_STATE_ID);

    let recovered = restart_recovered_timer_outcome(&state, transition_ids());
    let recovered_timer = recovered
        .timer_event
        .as_ref()
        .expect(enforcement::TEST_TIMER_EVENT_ID);
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
    assert!(active_timer_state_from_outcome(&recovered, policy::TEST_EVALUATED_AT).is_some());

    let cancelled = cancelled_timer_outcome(&state, transition_ids(), parent_action_reference());
    let cancelled_timer = cancelled
        .timer_event
        .as_ref()
        .expect(enforcement::TEST_TIMER_EVENT_ID);
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
    EnforcementTimerTransitionIds {
        result_id: enforcement::TEST_RESULT_ID.to_string(),
        audit_event_id: enforcement::TEST_AUDIT_EVENT_ID.to_string(),
        timer_event_id: enforcement::TEST_TIMER_EVENT_ID.to_string(),
        observed_at: policy::TEST_EVALUATED_AT.to_string(),
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
