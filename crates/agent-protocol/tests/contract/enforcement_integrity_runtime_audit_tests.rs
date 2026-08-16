use crate::notification_provider_status_boundary::{
    V08NotificationEscalationReadiness, V08NotificationProviderDeliveryClaim,
    V08NotificationProviderStatus, V08NotificationProviderStatusBoundaryEntry,
    V08NotificationProviderStatusBoundaryReadModel, V08NotificationProviderStatusProofState,
    V08NotificationQuietHoursReadiness,
};
use crate::{
    constants::{
        self, v08_enforcement_integrity_runtime_audit as proof,
        v08_integrity_alert_status_bridge as bridge,
        v08_notification_provider_status_boundary as boundary,
    },
    integrity_alert_status_bridge::{
        V08IntegrityAlertAuditState, V08IntegrityAlertDeliveryState,
        V08IntegrityAlertNotificationIntentState, V08IntegrityAlertParentVisibleStatus,
        V08IntegrityAlertState, V08IntegrityAlertStatusBridgeEntry,
        V08IntegrityAlertStatusBridgeReadModel,
    },
    policy_constants, ParentPlatform, V08EnforcementIntegrityRuntimeAuditAuditState,
    V08EnforcementIntegrityRuntimeAuditChildState, V08EnforcementIntegrityRuntimeAuditEntry,
    V08EnforcementIntegrityRuntimeAuditExecution,
    V08EnforcementIntegrityRuntimeAuditIntegrityState,
    V08EnforcementIntegrityRuntimeAuditIntentState, V08EnforcementIntegrityRuntimeAuditReadModel,
    V08EnforcementIntegrityRuntimeAuditResult, V08EnforcementIntegrityRuntimeAuditRollbackState,
    V08EnforcementIntegrityRuntimeAuditSurface, V08EnforcementIntegrityRuntimeAuditTimerState,
};
use ocentra_eventing::expect_value::ExpectValue;

#[test]
fn enforcement_integrity_runtime_audit_serializes_stable_state_values() {
    assert_eq!(
        serde_json::to_value(V08EnforcementIntegrityRuntimeAuditResult::RolledBack)
            .expect_value(constants::error::AGENT_EVENT_SERIALIZES),
        proof::RESULT_ROLLED_BACK
    );
    assert_eq!(
        serde_json::to_value(
            V08EnforcementIntegrityRuntimeAuditExecution::DryRunNoAdapterExecution
        )
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES),
        proof::EXECUTION_DRY_RUN_NO_ADAPTER
    );
    assert_eq!(
        serde_json::to_value(V08EnforcementIntegrityRuntimeAuditIntegrityState::StaleHeartbeat)
            .expect_value(constants::error::AGENT_EVENT_SERIALIZES),
        proof::INTEGRITY_STALE_HEARTBEAT
    );
}

#[test]
fn enforcement_integrity_runtime_audit_read_model_preserves_non_claim_flags() {
    let entry = |audit_entry_id: &'static str, result, execution, integrity_state| {
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
    };
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
        integrity_alert_status_bridge: bridge_read_model(),
        notification_provider_status_boundary: provider_boundary_read_model(),
    };
    let reparsed = serde_json::from_value::<V08EnforcementIntegrityRuntimeAuditReadModel>(
        serde_json::to_value(read_model).expect_value(constants::error::AGENT_EVENT_SERIALIZES),
    )
    .expect_value(constants::error::AGENT_EVENT_SERIALIZES);

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
    assert_eq!(
        reparsed.integrity_alert_status_bridge.read_model_id,
        bridge::READ_MODEL_ID
    );
    assert!(reparsed
        .integrity_alert_status_bridge
        .entries
        .iter()
        .all(|entry| !entry.provider_delivery_claimed));
    assert_provider_boundary_preserves_non_claims(&reparsed.notification_provider_status_boundary);
}

fn bridge_read_model() -> V08IntegrityAlertStatusBridgeReadModel {
    V08IntegrityAlertStatusBridgeReadModel {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        read_model_id: bridge::READ_MODEL_ID.to_string(),
        generated_at: policy_constants::TEST_EVALUATED_AT.to_string(),
        source_read_model_ids: vec![bridge::SOURCE_ENFORCEMENT_INTEGRITY_RUNTIME_AUDIT.to_string()],
        entries: vec![V08IntegrityAlertStatusBridgeEntry {
            schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
            bridge_entry_id: bridge::ENTRY_PERMISSION_LOSS.to_string(),
            integrity_alert_state: V08IntegrityAlertState::PermissionLoss,
            parent_visible_status: V08IntegrityAlertParentVisibleStatus::PermissionActionRequired,
            notification_intent_state: V08IntegrityAlertNotificationIntentState::IntentCreated,
            delivery_state: V08IntegrityAlertDeliveryState::NotDeliveredProviderNotConfigured,
            audit_state: V08IntegrityAlertAuditState::AuditRefBacked,
            reason_code_ref: bridge::REF_REASON_PERMISSION_LOSS.to_string(),
            status_ref: bridge::REF_STATUS_PERMISSION_ACTION_REQUIRED.to_string(),
            notification_intent_refs: vec![
                bridge::REF_NOTIFICATION_INTENT_PERMISSION_LOSS.to_string()
            ],
            notification_status_refs: vec![
                bridge::REF_NOTIFICATION_STATUS_PROVIDER_NOT_CONFIGURED.to_string()
            ],
            audit_refs: vec![bridge::REF_AUDIT_PERMISSION_LOSS.to_string()],
            integrity_refs: vec![bridge::REF_INTEGRITY_PERMISSION_STATE.to_string()],
            drill_in_refs: vec![bridge::REF_DRILL_IN_PERMISSION_LOSS.to_string()],
            manual_proof_requirements: vec![bridge::REQUIREMENT_PERMISSION_RESTORE.to_string()],
            boundary: bridge::BOUNDARY_PERMISSION_LOSS.to_string(),
            provider_delivery_claimed: false,
            broad_blocking_claimed: false,
            tamper_resistance_claimed: false,
            mobile_enforcement_claimed: false,
            stealth_persistence_claimed: false,
            privilege_escalation_claimed: false,
            last_checked_at: policy_constants::TEST_EVALUATED_AT.to_string(),
        }],
    }
}

fn assert_provider_boundary_preserves_non_claims(
    read_model: &V08NotificationProviderStatusBoundaryReadModel,
) {
    assert_eq!(read_model.read_model_id, boundary::READ_MODEL_ID);
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.provider_delivery_observed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.delivered_notification_claimed));
}

fn provider_boundary_read_model() -> V08NotificationProviderStatusBoundaryReadModel {
    V08NotificationProviderStatusBoundaryReadModel {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        read_model_id: boundary::READ_MODEL_ID.to_string(),
        generated_at: policy_constants::TEST_EVALUATED_AT.to_string(),
        source_read_model_ids: vec![boundary::SOURCE_REPORTS_NOTIFICATIONS_SYNC.to_string()],
        entries: vec![V08NotificationProviderStatusBoundaryEntry {
            schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
            status_entry_id: boundary::ENTRY_DELIVERED.to_string(),
            provider_status: V08NotificationProviderStatus::Delivered,
            status_proof_state: V08NotificationProviderStatusProofState::DeliveryReceiptRequired,
            quiet_hours_readiness: V08NotificationQuietHoursReadiness::DeferNoncritical,
            escalation_readiness: V08NotificationEscalationReadiness::WaitingWindow,
            delivery_claim_state: V08NotificationProviderDeliveryClaim::ReceiptRequired,
            notification_intent_ref: boundary::REF_NOTIFICATION_INTENT.to_string(),
            notification_status_ref: boundary::REF_STATUS_DELIVERED.to_string(),
            provider_attempt_ref: boundary::REF_ATTEMPT_DELIVERED.to_string(),
            audit_refs: vec![boundary::REF_AUDIT.to_string()],
            preference_refs: vec![boundary::REF_PARENT_PREFERENCES.to_string()],
            readiness_refs: vec![
                boundary::REF_QUIET_DEFER_NONCRITICAL.to_string(),
                boundary::REF_ESCALATION_WAITING_WINDOW.to_string(),
            ],
            provider_receipt_refs: vec![boundary::REF_PROVIDER_RECEIPT_REQUIRED.to_string()],
            manual_proof_requirements: vec![
                boundary::REQUIREMENT_PROVIDER_RECEIPT_ARTIFACT.to_string()
            ],
            minimal_payload_boundary: boundary::BOUNDARY_DELIVERED.to_string(),
            provider_delivery_implemented: false,
            provider_delivery_observed: false,
            delivered_notification_claimed: false,
            sensitive_provider_payload_claimed: false,
            provider_stores_child_evidence_claimed: false,
            last_checked_at: policy_constants::TEST_EVALUATED_AT.to_string(),
        }],
    }
}
