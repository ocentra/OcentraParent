use crate::{
    constants::{self, v08_enforcement_integrity_runtime_audit as proof},
    policy_constants, ParentPlatform, V08EnforcementIntegrityRuntimeAuditAuditState,
    V08EnforcementIntegrityRuntimeAuditChildState, V08EnforcementIntegrityRuntimeAuditEntry,
    V08EnforcementIntegrityRuntimeAuditExecution,
    V08EnforcementIntegrityRuntimeAuditIntegrityState,
    V08EnforcementIntegrityRuntimeAuditIntentState, V08EnforcementIntegrityRuntimeAuditReadModel,
    V08EnforcementIntegrityRuntimeAuditResult, V08EnforcementIntegrityRuntimeAuditRollbackState,
    V08EnforcementIntegrityRuntimeAuditSurface, V08EnforcementIntegrityRuntimeAuditTimerState,
};

#[test]
fn enforcement_integrity_runtime_audit_serializes_stable_state_values() {
    assert_eq!(
        serde_json::to_value(V08EnforcementIntegrityRuntimeAuditResult::RolledBack)
            .expect(constants::error::AGENT_EVENT_SERIALIZES),
        proof::RESULT_ROLLED_BACK
    );
    assert_eq!(
        serde_json::to_value(
            V08EnforcementIntegrityRuntimeAuditExecution::DryRunNoAdapterExecution
        )
        .expect(constants::error::AGENT_EVENT_SERIALIZES),
        proof::EXECUTION_DRY_RUN_NO_ADAPTER
    );
    assert_eq!(
        serde_json::to_value(V08EnforcementIntegrityRuntimeAuditIntegrityState::StaleHeartbeat)
            .expect(constants::error::AGENT_EVENT_SERIALIZES),
        proof::INTEGRITY_STALE_HEARTBEAT
    );
}

#[test]
fn enforcement_integrity_runtime_audit_read_model_preserves_non_claim_flags() {
    let read_model = V08EnforcementIntegrityRuntimeAuditReadModel {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        read_model_id: proof::READ_MODEL_ID.to_string(),
        generated_at: policy_constants::TEST_EVALUATED_AT.to_string(),
        source_read_model_ids: vec![
            proof::SOURCE_SUPPORTED_ADAPTER_RUNTIME_PROOF.to_string(),
            proof::SOURCE_ENFORCEMENT_AUDIT_JOURNAL.to_string(),
        ],
        entries: vec![
            entry(
                proof::ENTRY_APP_TIME_LIMIT_SUCCEEDED,
                V08EnforcementIntegrityRuntimeAuditResult::Succeeded,
                V08EnforcementIntegrityRuntimeAuditExecution::ExecutedSupportedBoundary,
                V08EnforcementIntegrityRuntimeAuditIntegrityState::Running,
            ),
            entry(
                proof::ENTRY_TAMPER_MANUAL,
                V08EnforcementIntegrityRuntimeAuditResult::ManualRequired,
                V08EnforcementIntegrityRuntimeAuditExecution::ManualRequiredNoExecution,
                V08EnforcementIntegrityRuntimeAuditIntegrityState::TamperSignalManualRequired,
            ),
        ],
    };
    let reparsed = serde_json::from_value::<V08EnforcementIntegrityRuntimeAuditReadModel>(
        serde_json::to_value(read_model).expect(constants::error::AGENT_EVENT_SERIALIZES),
    )
    .expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(reparsed.read_model_id, proof::READ_MODEL_ID);
    assert_eq!(reparsed.entries.len(), 2);
    assert!(reparsed
        .entries
        .iter()
        .all(|entry| !entry.broad_installed_app_blocking_claimed));
    assert!(reparsed
        .entries
        .iter()
        .all(|entry| !entry.host_network_domain_blocking_claimed));
    assert!(reparsed
        .entries
        .iter()
        .all(|entry| !entry.exact_active_tab_enforcement_claimed));
    assert!(reparsed
        .entries
        .iter()
        .all(|entry| !entry.tamper_hardening_claimed));
    assert!(reparsed
        .entries
        .iter()
        .all(|entry| !entry.stealth_persistence_claimed));
    assert!(reparsed
        .entries
        .iter()
        .all(|entry| !entry.privilege_escalation_claimed));
}

fn entry(
    audit_entry_id: &str,
    result: V08EnforcementIntegrityRuntimeAuditResult,
    execution: V08EnforcementIntegrityRuntimeAuditExecution,
    integrity_state: V08EnforcementIntegrityRuntimeAuditIntegrityState,
) -> V08EnforcementIntegrityRuntimeAuditEntry {
    V08EnforcementIntegrityRuntimeAuditEntry {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        audit_entry_id: audit_entry_id.to_string(),
        surface: V08EnforcementIntegrityRuntimeAuditSurface::AppGameTimeLimit,
        platform: ParentPlatform::Windows,
        result,
        execution,
        intent_state: V08EnforcementIntegrityRuntimeAuditIntentState::Validated,
        timer_state: V08EnforcementIntegrityRuntimeAuditTimerState::ActiveTimerBacked,
        rollback_state: V08EnforcementIntegrityRuntimeAuditRollbackState::RollbackTokenBacked,
        child_state: V08EnforcementIntegrityRuntimeAuditChildState::ReasonRefBacked,
        integrity_state,
        audit_state: V08EnforcementIntegrityRuntimeAuditAuditState::AuditBacked,
        policy_decision_refs: vec![proof::REF_POLICY_DECISION.to_string()],
        evidence_refs: vec![proof::REF_APP_SESSION_EVIDENCE.to_string()],
        adapter_outcome_refs: vec![proof::REF_ADAPTER_OUTCOME.to_string()],
        audit_refs: vec![proof::REF_ENFORCEMENT_AUDIT.to_string()],
        rollback_refs: vec!["rollback-token-ref".to_string()],
        timer_refs: vec![proof::REF_TIMER_STATE.to_string()],
        child_status_refs: vec![proof::REF_CHILD_STATUS.to_string()],
        integrity_refs: vec![proof::REF_INTEGRITY_HEARTBEAT.to_string()],
        parent_intent_refs: Vec::new(),
        manual_proof_requirements: Vec::new(),
        boundary: "fixture boundary".to_string(),
        broad_installed_app_blocking_claimed: false,
        host_network_domain_blocking_claimed: false,
        exact_active_tab_enforcement_claimed: false,
        notification_delivery_claimed: false,
        tamper_hardening_claimed: false,
        mobile_privilege_claimed: false,
        stealth_persistence_claimed: false,
        privilege_escalation_claimed: false,
        last_checked_at: policy_constants::TEST_EVALUATED_AT.to_string(),
    }
}
