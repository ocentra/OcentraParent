use super::{
    constants, constants::enforcement, policy_constants as policy, EnforcementAction,
    EnforcementActiveTimerState, EnforcementAdapterKind, EnforcementAdapterResultCode,
    EnforcementAuditEvent, EnforcementAuditEventKind, EnforcementCapabilityState,
    EnforcementCapabilityStatus, EnforcementDependencyState, EnforcementIntent,
    EnforcementIntentSource, EnforcementMode, EnforcementPermissionState, EnforcementResult,
    EnforcementResultStatus, EnforcementRollbackState, EnforcementTimerEvent,
    EnforcementTimerEventKind, EnforcementUnavailableReason, EnforcementUnavailableStatus,
    ParentActionReference, ParentActorReference, ParentActorRole, ParentDeviceReference,
    ParentEvidenceReference, ParentEvidenceReferenceKind, ParentPlatform, PolicyAction,
    PolicyTarget, PolicyTargetType,
};
use ocentra_eventing::expect_value::{ExpectErrValue, ExpectValue};
use ocentra_parent_agent_protocol::enforcement::EnforcementAuditJournalEvent;

const TIMER_TRANSITION_CASES: &[(
    EnforcementTimerEventKind,
    &str,
    bool,
    Option<EnforcementUnavailableReason>,
)] = &[
    (
        EnforcementTimerEventKind::Created,
        enforcement::TIMER_CREATED,
        false,
        None,
    ),
    (
        EnforcementTimerEventKind::Extended,
        enforcement::TIMER_EXTENDED,
        false,
        None,
    ),
    (
        EnforcementTimerEventKind::Expired,
        enforcement::TIMER_EXPIRED,
        false,
        None,
    ),
    (
        EnforcementTimerEventKind::Cancelled,
        enforcement::TIMER_CANCELLED,
        false,
        None,
    ),
    (
        EnforcementTimerEventKind::RestartRecovered,
        enforcement::TIMER_RESTART_RECOVERED,
        true,
        None,
    ),
    (
        EnforcementTimerEventKind::RollbackRequested,
        enforcement::TIMER_ROLLBACK_REQUESTED,
        false,
        None,
    ),
    (
        EnforcementTimerEventKind::RollbackCompleted,
        enforcement::TIMER_ROLLBACK_COMPLETED,
        false,
        None,
    ),
    (
        EnforcementTimerEventKind::RecoveryNeeded,
        enforcement::TIMER_RECOVERY_NEEDED,
        false,
        Some(EnforcementUnavailableReason::AdapterError),
    ),
    (
        EnforcementTimerEventKind::Unavailable,
        enforcement::TIMER_UNAVAILABLE,
        false,
        Some(EnforcementUnavailableReason::AdapterUnavailable),
    ),
];

#[test]
fn enforcement_shapes_serialize_to_parent_domain_contract_names() {
    let capability = process_capability();
    let intent = enforcement_intent();
    let action = enforcement_action(&intent);
    let result = enforcement_result(&action, capability.clone());
    let audit = EnforcementAuditEvent {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        audit_event_id: enforcement::TEST_AUDIT_EVENT_ID.to_string(),
        audit_event_kind: EnforcementAuditEventKind::Succeeded,
        action: action.clone(),
        result,
        capability,
        unavailable_status: None,
        policy_version: policy::TEST_POLICY_VERSION.to_string(),
        evidence_references: vec![evidence()],
        actor: None,
        parent_override: None,
        journal_sequence: Some(enforcement::TEST_JOURNAL_SEQUENCE.to_string()),
        observed_at: policy::TEST_EVALUATED_AT.to_string(),
    };
    let timer = EnforcementTimerEvent {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        timer_event_id: enforcement::TEST_TIMER_EVENT_ID.to_string(),
        timer_event_kind: EnforcementTimerEventKind::RestartRecovered,
        action_id: action.action_id,
        policy_decision_id: policy::TEST_DECISION_ID.to_string(),
        evidence_references: vec![evidence()],
        scheduled_at: policy::TEST_EVALUATED_AT.to_string(),
        effective_at: Some(policy::TEST_EXPIRES_AT.to_string()),
        rollback_token: Some(enforcement::TEST_ROLLBACK_TOKEN.to_string()),
        recovered_after_restart: true,
        unavailable_reason: None,
    };

    let serialized_audit =
        serde_json::to_value(audit).expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    let serialized_timer =
        serde_json::to_value(timer).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        serialized_audit["auditEventKind"],
        enforcement::AUDIT_SUCCEEDED
    );
    assert_eq!(
        serialized_audit["result"]["status"],
        enforcement::RESULT_ACTUALLY_ENFORCED
    );
    assert_eq!(
        serialized_audit["result"]["rollbackState"],
        enforcement::ROLLBACK_AVAILABLE
    );
    assert_eq!(
        serialized_audit["result"]["capability"]["capabilityState"],
        enforcement::CAPABILITY_SUPPORTED
    );
    assert_eq!(
        serialized_audit["capability"]["capabilityState"],
        enforcement::CAPABILITY_SUPPORTED
    );
    assert_eq!(
        serialized_audit["unavailableStatus"],
        serde_json::Value::Null
    );
    assert_eq!(
        serialized_timer["timerEventKind"],
        enforcement::TIMER_RESTART_RECOVERED
    );
    assert_eq!(serialized_timer["recoveredAfterRestart"], true);
}

#[test]
fn enforcement_journal_projection_retains_audit_references_without_raw_target_value() {
    let capability = process_capability();
    let mut intent = enforcement_intent();
    intent.actor = Some(parent_actor());
    intent.parent_approval = Some(parent_action_reference());
    let action = enforcement_action(&intent);
    let result = enforcement_result(&action, capability.clone());
    let audit = EnforcementAuditEvent {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        audit_event_id: enforcement::TEST_AUDIT_EVENT_ID.to_string(),
        audit_event_kind: EnforcementAuditEventKind::Succeeded,
        action,
        result,
        capability,
        unavailable_status: None,
        policy_version: policy::TEST_POLICY_VERSION.to_string(),
        evidence_references: vec![evidence()],
        actor: intent.actor.clone(),
        parent_override: intent.parent_approval.clone(),
        journal_sequence: Some(enforcement::TEST_JOURNAL_SEQUENCE.to_string()),
        observed_at: policy::TEST_EVALUATED_AT.to_string(),
    };

    let journal = EnforcementAuditJournalEvent::from(&audit);
    let serialized =
        serde_json::to_value(&journal).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(journal.policy_decision_id, policy::TEST_DECISION_ID);
    assert_eq!(journal.target_id, enforcement::TEST_PROCESS_TARGET_ID);
    assert_eq!(journal.target_type, PolicyTargetType::Process);
    assert_eq!(journal.evidence_references, vec![evidence()]);
    assert_eq!(journal.actor, intent.actor);
    assert_eq!(journal.parent_override, intent.parent_approval);
    assert_eq!(journal.rollback_state, EnforcementRollbackState::Available);
    assert_eq!(serialized["policyAction"], policy::ACTION_BLOCK);
    assert_eq!(serialized["targetId"], enforcement::TEST_PROCESS_TARGET_ID);
    assert_eq!(serialized["targetType"], policy::TARGET_TYPE_PROCESS);
    assert_eq!(
        serialized["adapterKind"],
        enforcement::ADAPTER_KIND_PROCESS_CONTROL
    );
    assert_eq!(serialized["platform"], enforcement::PLATFORM_WINDOWS);
    assert_eq!(
        serialized["evidenceReferences"][0]["evidenceReferenceId"],
        policy::TEST_EVIDENCE_ID
    );
    assert_eq!(
        serialized["parentOverride"]["actionReferenceId"],
        enforcement::TEST_PARENT_ACTION_REFERENCE_ID
    );
    assert!(!serialized
        .as_object()
        .expect_value("journal projection object")
        .contains_key("targetValue"));
}

#[test]
fn unsupported_status_values_do_not_deserialize() {
    let payload = serde_json::json!({
        "schemaVersion": policy::CONTRACT_SCHEMA_VERSION_V0_6,
        "resultId": enforcement::TEST_RESULT_ID,
        "actionId": enforcement::TEST_ACTION_ID,
        "status": "blocked-by-label",
        "adapterResultCode": enforcement::ADAPTER_NO_OP,
        "startedAt": policy::TEST_EVALUATED_AT,
        "completedAt": null,
        "rollbackToken": null,
        "rollbackState": enforcement::ROLLBACK_NOT_REQUIRED,
        "unavailableReason": null,
        "unavailableStatus": null,
        "failedReason": null,
        "nextCheckAt": null,
        "capability": process_capability()
    });

    let parsed = serde_json::from_value::<EnforcementResult>(payload);

    let error = parsed.expect_err_value("expected invalid enforcement status to fail");
    let message = error.to_string();
    assert!(
        message.contains("blocked-by-label"),
        "expected invalid status error to mention blocked-by-label, got {message}"
    );
}

#[test]
fn unavailable_status_serializes_typed_capability_reason() {
    let capability = unavailable_capability();
    let unavailable = EnforcementUnavailableStatus {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        capability,
        unavailable_reason: EnforcementUnavailableReason::UnsupportedPlatform,
        retryable: false,
        checked_at: policy::TEST_EVALUATED_AT.to_string(),
    };

    let serialized =
        serde_json::to_value(unavailable).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        serialized["unavailableReason"],
        enforcement::UNAVAILABLE_UNSUPPORTED_PLATFORM
    );
    assert_eq!(
        serialized["capability"]["capabilityState"],
        enforcement::CAPABILITY_UNAVAILABLE
    );
    assert_eq!(serialized["retryable"], false);
}

#[test]
fn parent_approval_and_override_serialize_as_audit_references() {
    let capability = process_capability();
    let mut intent = enforcement_intent();
    intent.actor = Some(parent_actor());
    intent.parent_approval = Some(parent_action_reference());
    let action = enforcement_action(&intent);
    let result = enforcement_result(&action, capability.clone());
    let audit = EnforcementAuditEvent {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        audit_event_id: enforcement::TEST_AUDIT_EVENT_ID.to_string(),
        audit_event_kind: EnforcementAuditEventKind::Succeeded,
        action: action.clone(),
        result,
        capability,
        unavailable_status: None,
        policy_version: policy::TEST_POLICY_VERSION.to_string(),
        evidence_references: vec![evidence()],
        actor: intent.actor,
        parent_override: action.parent_approval.clone(),
        journal_sequence: Some(enforcement::TEST_JOURNAL_SEQUENCE.to_string()),
        observed_at: policy::TEST_EVALUATED_AT.to_string(),
    };

    let serialized_action =
        serde_json::to_value(action).expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    let serialized_audit =
        serde_json::to_value(audit).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        serialized_action["parentApproval"]["actionReferenceId"],
        enforcement::TEST_PARENT_ACTION_REFERENCE_ID
    );
    assert_eq!(
        serialized_action["parentApproval"]["actor"]["actorId"],
        policy::TEST_PARENT_ACTOR_ID
    );
    assert_eq!(
        serialized_action["parentApproval"]["actor"]["role"],
        policy::ACTOR_ROLE_PARENT
    );
    assert_eq!(
        serialized_action["parentApproval"]["policyVersion"],
        policy::TEST_POLICY_VERSION
    );
    assert_eq!(
        serialized_action["parentApproval"]["createdAt"],
        policy::TEST_EVALUATED_AT
    );
    assert_eq!(
        serialized_audit["actor"]["actorId"],
        policy::TEST_PARENT_ACTOR_ID
    );
    assert_eq!(
        serialized_audit["parentOverride"],
        serialized_action["parentApproval"]
    );
}

#[test]
fn timer_event_kinds_serialize_to_contract_literals() {
    let action = enforcement_action(&enforcement_intent());

    for (index, (timer_event_kind, expected_kind, recovered_after_restart, unavailable_reason)) in
        TIMER_TRANSITION_CASES.iter().enumerate()
    {
        let timer = EnforcementTimerEvent {
            schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
            timer_event_id: format!("{}-{index}", enforcement::TEST_TIMER_EVENT_ID),
            timer_event_kind: *timer_event_kind,
            action_id: action.action_id.clone(),
            policy_decision_id: policy::TEST_DECISION_ID.to_string(),
            evidence_references: vec![evidence()],
            scheduled_at: policy::TEST_EVALUATED_AT.to_string(),
            effective_at: Some(policy::TEST_EXPIRES_AT.to_string()),
            rollback_token: Some(enforcement::TEST_ROLLBACK_TOKEN.to_string()),
            recovered_after_restart: *recovered_after_restart,
            unavailable_reason: *unavailable_reason,
        };
        let serialized =
            serde_json::to_value(timer).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

        assert_eq!(serialized["timerEventKind"], *expected_kind);
        assert_eq!(
            serialized["actionId"],
            serde_json::Value::from(action.action_id.as_str())
        );
        assert_eq!(
            serialized["policyDecisionId"],
            serde_json::Value::from(action.policy_decision_id.as_str())
        );
        assert_eq!(
            serialized["evidenceReferences"][0]["evidenceReferenceId"],
            serde_json::Value::from(action.evidence_references[0].evidence_reference_id.as_str())
        );
        assert_eq!(
            serialized["rollbackToken"],
            serde_json::Value::from(
                action
                    .rollback_token
                    .as_deref()
                    .expect_value(enforcement::TEST_ROLLBACK_TOKEN)
            )
        );
        assert_eq!(
            serialized["recoveredAfterRestart"],
            *recovered_after_restart
        );
        assert_eq!(
            serialized["unavailableReason"],
            unavailable_reason
                .map(|reason| serde_json::Value::from(reason.as_protocol_str()))
                .unwrap_or(serde_json::Value::Null)
        );
    }
}

#[test]
fn active_timer_state_serializes_action_result_audit_and_timer() {
    let capability = process_capability();
    let mut intent = enforcement_intent();
    intent.actor = Some(parent_actor());
    intent.parent_approval = Some(parent_action_reference());
    let action = enforcement_action(&intent);
    let result = enforcement_result(&action, capability.clone());
    let audit = EnforcementAuditEvent {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        audit_event_id: enforcement::TEST_AUDIT_EVENT_ID.to_string(),
        audit_event_kind: EnforcementAuditEventKind::Cancelled,
        action: action.clone(),
        result: result.clone(),
        capability,
        unavailable_status: None,
        policy_version: policy::TEST_POLICY_VERSION.to_string(),
        evidence_references: vec![evidence()],
        actor: intent.actor,
        parent_override: action.parent_approval.clone(),
        journal_sequence: Some(enforcement::TEST_JOURNAL_SEQUENCE.to_string()),
        observed_at: policy::TEST_EVALUATED_AT.to_string(),
    };
    let timer = EnforcementTimerEvent {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        timer_event_id: enforcement::TEST_TIMER_EVENT_ID.to_string(),
        timer_event_kind: EnforcementTimerEventKind::Cancelled,
        action_id: action.action_id.clone(),
        policy_decision_id: policy::TEST_DECISION_ID.to_string(),
        evidence_references: vec![evidence()],
        scheduled_at: policy::TEST_EVALUATED_AT.to_string(),
        effective_at: None,
        rollback_token: Some(enforcement::TEST_ROLLBACK_TOKEN.to_string()),
        recovered_after_restart: false,
        unavailable_reason: None,
    };
    let state = EnforcementActiveTimerState {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        state_id: enforcement::TEST_TIMER_STATE_ID.to_string(),
        action,
        result,
        audit_event: audit,
        timer_event: timer,
        app_game_session: None,
        stored_at: policy::TEST_EVALUATED_AT.to_string(),
    };

    let serialized =
        serde_json::to_value(state).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(serialized["stateId"], enforcement::TEST_TIMER_STATE_ID);
    assert_eq!(
        serialized["auditEvent"]["auditEventKind"],
        enforcement::AUDIT_CANCELLED
    );
    assert_eq!(
        serialized["timerEvent"]["timerEventKind"],
        enforcement::TIMER_CANCELLED
    );
    assert_eq!(
        serialized["action"]["parentApproval"]["actionReferenceId"],
        enforcement::TEST_PARENT_ACTION_REFERENCE_ID
    );
}

fn process_capability() -> EnforcementCapabilityStatus {
    EnforcementCapabilityStatus {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        platform: ParentPlatform::Windows,
        adapter_kind: EnforcementAdapterKind::ProcessControl,
        capability_state: EnforcementCapabilityState::Supported,
        permission_state: EnforcementPermissionState::NotRequired,
        dependency_state: EnforcementDependencyState::Installed,
        supported_actions: vec![
            EnforcementMode::TerminateProcess,
            EnforcementMode::TemporaryBlock,
        ],
        degraded_reason: None,
        last_checked_at: policy::TEST_EVALUATED_AT.to_string(),
    }
}

fn unavailable_capability() -> EnforcementCapabilityStatus {
    EnforcementCapabilityStatus {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        platform: ParentPlatform::Linux,
        adapter_kind: EnforcementAdapterKind::ProcessControl,
        capability_state: EnforcementCapabilityState::Unavailable,
        permission_state: EnforcementPermissionState::NotRequired,
        dependency_state: EnforcementDependencyState::NotRequired,
        supported_actions: Vec::new(),
        degraded_reason: Some(enforcement::UNAVAILABLE_UNSUPPORTED_PLATFORM.to_string()),
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

fn enforcement_action(intent: &EnforcementIntent) -> EnforcementAction {
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
        capability: process_capability(),
        reason_codes: vec![policy::TEST_REASON_PARENT_BLOCK.to_string()],
        evidence_references: vec![evidence()],
        local_ai_result_id: None,
        parent_approval: intent.parent_approval.clone(),
        dry_run: false,
        requested_at: policy::TEST_EVALUATED_AT.to_string(),
        expires_at: Some(policy::TEST_EXPIRES_AT.to_string()),
        rollback_token: Some(enforcement::TEST_ROLLBACK_TOKEN.to_string()),
    }
}

fn enforcement_result(
    action: &EnforcementAction,
    capability: EnforcementCapabilityStatus,
) -> EnforcementResult {
    EnforcementResult {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        result_id: enforcement::TEST_RESULT_ID.to_string(),
        action_id: action.action_id.clone(),
        status: EnforcementResultStatus::ActuallyEnforced,
        adapter_result_code: EnforcementAdapterResultCode::ProcessTerminated,
        started_at: policy::TEST_EVALUATED_AT.to_string(),
        completed_at: Some(policy::TEST_EVALUATED_AT.to_string()),
        rollback_token: action.rollback_token.clone(),
        rollback_state: EnforcementRollbackState::Available,
        unavailable_reason: None,
        unavailable_status: None,
        failed_reason: None,
        next_check_at: None,
        capability,
    }
}

fn parent_actor() -> ParentActorReference {
    ParentActorReference {
        actor_id: policy::TEST_PARENT_ACTOR_ID.to_string(),
        role: ParentActorRole::Parent,
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
