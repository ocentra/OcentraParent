use super::{
    constants, constants::enforcement, policy_constants as policy, EnforcementAction,
    EnforcementAdapterKind, EnforcementAdapterResultCode, EnforcementAuditEvent,
    EnforcementAuditEventKind, EnforcementCapabilityState, EnforcementCapabilityStatus,
    EnforcementDependencyState, EnforcementIntent, EnforcementIntentSource, EnforcementMode,
    EnforcementPermissionState, EnforcementResult, EnforcementResultStatus,
    EnforcementRollbackState, EnforcementUnavailableReason, EnforcementUnavailableStatus,
    ParentDeviceReference, ParentEvidenceReference, ParentEvidenceReferenceKind, ParentPlatform,
    PolicyAction, PolicyTarget, PolicyTargetType,
};
use ocentra_eventing::{envelope::DomainEvent, expect_value::ExpectValue};
use ocentra_parent_agent_protocol::enforcement::EnforcementAuditJournalEvent;

#[test]
fn unavailable_audit_event_serializes_result_capability_and_unavailable_status_boundary() {
    let capability = degraded_capability();
    let unavailable_status = unavailable_status(capability.clone());
    let action = enforcement_action(&enforcement_intent(), capability.clone());
    let result = enforcement_result(&action, capability.clone(), unavailable_status.clone());
    let audit = enforcement_audit(action, result, capability, unavailable_status);

    let serialized =
        serde_json::to_value(audit).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(serialized["auditEventKind"], enforcement::AUDIT_UNAVAILABLE);
    assert_eq!(serialized["capability"], serialized["result"]["capability"]);
    assert_eq!(
        serialized["unavailableStatus"],
        serialized["result"]["unavailableStatus"]
    );
    assert_eq!(
        serialized["unavailableStatus"]["unavailableReason"],
        enforcement::UNAVAILABLE_ADAPTER_UNAVAILABLE
    );
}

#[test]
fn audit_event_projects_to_a_redacted_typed_eventing_journal_contract() {
    let capability = degraded_capability();
    let unavailable_status = unavailable_status(capability.clone());
    let action = enforcement_action(&enforcement_intent(), capability.clone());
    let result = enforcement_result(&action, capability.clone(), unavailable_status.clone());
    let audit = enforcement_audit(action, result, capability, unavailable_status);
    let journal = EnforcementAuditJournalEvent::from(&audit);
    let contract = journal.contract().expect_value("journal event contract");
    let aggregate_key = journal
        .aggregate_key()
        .expect_value("journal aggregate key");
    let idempotency_key = journal
        .idempotency_key()
        .expect_value("journal idempotency key");

    assert_eq!(
        contract.event_type.as_str(),
        enforcement::EVENT_AUDIT_JOURNAL_RECORDED
    );
    assert_eq!(
        contract.schema_version.value(),
        enforcement::EVENT_SCHEMA_VERSION
    );
    assert_eq!(
        aggregate_key.as_str(),
        format!(
            "{}{}",
            enforcement::EVENTING_AGGREGATE_AUDIT_PREFIX,
            audit.action.action_id
        )
    );
    assert_eq!(
        idempotency_key.as_str(),
        format!(
            "{}{}",
            enforcement::EVENTING_IDEMPOTENCY_AUDIT_PREFIX,
            audit.audit_event_id
        )
    );
    assert_eq!(journal.audit_event_id, audit.audit_event_id);
    assert_eq!(journal.action_id, audit.action.action_id);
    assert_eq!(journal.result_id, audit.result.result_id);
    assert_eq!(journal.audit_event_kind, audit.audit_event_kind);
    assert_eq!(journal.result_status, audit.result.status);
    assert_eq!(
        journal.adapter_result_code,
        audit.result.adapter_result_code
    );
    assert_eq!(journal.capability_state, audit.capability.capability_state);
    assert_eq!(journal.observed_at, audit.observed_at);
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

fn unavailable_status(capability: EnforcementCapabilityStatus) -> EnforcementUnavailableStatus {
    EnforcementUnavailableStatus {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        capability,
        unavailable_reason: EnforcementUnavailableReason::AdapterUnavailable,
        retryable: true,
        checked_at: policy::TEST_EVALUATED_AT.to_string(),
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
