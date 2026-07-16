use ocentra_parent_agent_protocol::constants::v08_integrity_alert_status_bridge as bridge;
use ocentra_parent_agent_protocol::integrity_alert_status_bridge::V08IntegrityAlertAuditState;
use ocentra_parent_agent_protocol::integrity_alert_status_bridge::V08IntegrityAlertDeliveryState;
use ocentra_parent_agent_protocol::integrity_alert_status_bridge::V08IntegrityAlertNotificationIntentState;
use ocentra_parent_agent_protocol::integrity_alert_status_bridge::V08IntegrityAlertParentVisibleStatus;
use ocentra_parent_agent_protocol::integrity_alert_status_bridge::V08IntegrityAlertState;
use ocentra_parent_agent_protocol::integrity_alert_status_bridge::V08IntegrityAlertStatusBridgeEntry;
use ocentra_parent_agent_protocol::integrity_alert_status_bridge::V08IntegrityAlertStatusBridgeReadModel;
use ocentra_parent_agent_protocol::policy_constants;

#[derive(Clone, Copy)]
pub(crate) struct GeneratedAtTextRef<'a>(pub(crate) &'a str);

#[derive(Clone, Copy)]
struct StaticTextRefs(&'static [&'static str]);

struct BridgeTextList(Vec<String>);

pub(crate) fn v08_integrity_alert_status_bridge_read_model<'a>(
    generated_at: impl Into<GeneratedAtTextRef<'a>>,
) -> V08IntegrityAlertStatusBridgeReadModel {
    let generated_at = generated_at.into();
    V08IntegrityAlertStatusBridgeReadModel {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        read_model_id: bridge::READ_MODEL_ID.to_string(),
        generated_at: generated_at.0.to_string(),
        source_read_model_ids: vec![
            bridge::SOURCE_ENFORCEMENT_INTEGRITY_RUNTIME_AUDIT.to_string(),
            bridge::SOURCE_SUPPORTED_ADAPTER_RUNTIME_PROOF.to_string(),
            bridge::SOURCE_REPORTS_NOTIFICATIONS_SYNC.to_string(),
        ],
        entries: entry_specs()
            .iter()
            .map(|spec| entry_from_spec(spec, generated_at))
            .collect(),
    }
}

struct EntrySpec {
    bridge_entry_id: &'static str,
    integrity_alert_state: V08IntegrityAlertState,
    parent_visible_status: V08IntegrityAlertParentVisibleStatus,
    notification_intent_state: V08IntegrityAlertNotificationIntentState,
    audit_state: V08IntegrityAlertAuditState,
    reason_code_ref: &'static str,
    status_ref: &'static str,
    notification_intent_refs: &'static [&'static str],
    notification_status_refs: &'static [&'static str],
    audit_refs: &'static [&'static str],
    integrity_refs: &'static [&'static str],
    drill_in_refs: &'static [&'static str],
    manual_proof_requirements: &'static [&'static str],
    boundary: &'static str,
}

fn entry_specs() -> Vec<EntrySpec> {
    vec![
        EntrySpec {
            bridge_entry_id: bridge::ENTRY_PERMISSION_LOSS,
            integrity_alert_state: V08IntegrityAlertState::PermissionLoss,
            parent_visible_status: V08IntegrityAlertParentVisibleStatus::PermissionActionRequired,
            notification_intent_state: V08IntegrityAlertNotificationIntentState::IntentCreated,
            audit_state: V08IntegrityAlertAuditState::AuditRefBacked,
            reason_code_ref: bridge::REF_REASON_PERMISSION_LOSS,
            status_ref: bridge::REF_STATUS_PERMISSION_ACTION_REQUIRED,
            notification_intent_refs: &[bridge::REF_NOTIFICATION_INTENT_PERMISSION_LOSS],
            notification_status_refs: &[bridge::REF_NOTIFICATION_STATUS_PROVIDER_NOT_CONFIGURED],
            audit_refs: &[bridge::REF_AUDIT_PERMISSION_LOSS],
            integrity_refs: &[bridge::REF_INTEGRITY_PERMISSION_STATE],
            drill_in_refs: &[bridge::REF_DRILL_IN_PERMISSION_LOSS],
            manual_proof_requirements: &[bridge::REQUIREMENT_PERMISSION_RESTORE],
            boundary: bridge::BOUNDARY_PERMISSION_LOSS,
        },
        EntrySpec {
            bridge_entry_id: bridge::ENTRY_STALE_HEARTBEAT,
            integrity_alert_state: V08IntegrityAlertState::StaleHeartbeat,
            parent_visible_status: V08IntegrityAlertParentVisibleStatus::AgentHeartbeatStale,
            notification_intent_state: V08IntegrityAlertNotificationIntentState::IntentCreated,
            audit_state: V08IntegrityAlertAuditState::AuditRefBacked,
            reason_code_ref: bridge::REF_REASON_STALE_HEARTBEAT,
            status_ref: bridge::REF_STATUS_AGENT_HEARTBEAT_STALE,
            notification_intent_refs: &[bridge::REF_NOTIFICATION_INTENT_STALE_HEARTBEAT],
            notification_status_refs: &[bridge::REF_NOTIFICATION_STATUS_PROVIDER_NOT_CONFIGURED],
            audit_refs: &[bridge::REF_AUDIT_STALE_HEARTBEAT],
            integrity_refs: &[bridge::REF_INTEGRITY_HEARTBEAT],
            drill_in_refs: &[bridge::REF_DRILL_IN_STALE_HEARTBEAT],
            manual_proof_requirements: &[bridge::REQUIREMENT_FRESH_HEARTBEAT],
            boundary: bridge::BOUNDARY_STALE_HEARTBEAT,
        },
        EntrySpec {
            bridge_entry_id: bridge::ENTRY_STOPPED_OR_REMOVED,
            integrity_alert_state: V08IntegrityAlertState::StoppedOrRemoved,
            parent_visible_status: V08IntegrityAlertParentVisibleStatus::AgentStoppedOrRemoved,
            notification_intent_state: V08IntegrityAlertNotificationIntentState::IntentCreated,
            audit_state: V08IntegrityAlertAuditState::AuditRefBacked,
            reason_code_ref: bridge::REF_REASON_STOPPED_OR_REMOVED,
            status_ref: bridge::REF_STATUS_AGENT_STOPPED_OR_REMOVED,
            notification_intent_refs: &[bridge::REF_NOTIFICATION_INTENT_STOPPED_OR_REMOVED],
            notification_status_refs: &[bridge::REF_NOTIFICATION_STATUS_PROVIDER_NOT_CONFIGURED],
            audit_refs: &[bridge::REF_AUDIT_STOPPED_OR_REMOVED],
            integrity_refs: &[bridge::REF_INTEGRITY_SERVICE_STATE],
            drill_in_refs: &[bridge::REF_DRILL_IN_STOPPED_OR_REMOVED],
            manual_proof_requirements: &[
                bridge::REQUIREMENT_SERVICE_RESTART_RECOVERY,
                bridge::REQUIREMENT_UNINSTALL_DETECTION_ARTIFACT,
            ],
            boundary: bridge::BOUNDARY_STOPPED_OR_REMOVED,
        },
        EntrySpec {
            bridge_entry_id: bridge::ENTRY_TAMPER_MANUAL,
            integrity_alert_state: V08IntegrityAlertState::TamperManualRequired,
            parent_visible_status: V08IntegrityAlertParentVisibleStatus::TamperReviewRequired,
            notification_intent_state:
                V08IntegrityAlertNotificationIntentState::ManualReviewRequired,
            audit_state: V08IntegrityAlertAuditState::ManualRequired,
            reason_code_ref: bridge::REF_REASON_TAMPER_MANUAL,
            status_ref: bridge::REF_STATUS_TAMPER_REVIEW_REQUIRED,
            notification_intent_refs: &[bridge::REF_NOTIFICATION_INTENT_TAMPER_MANUAL],
            notification_status_refs: &[bridge::REF_NOTIFICATION_STATUS_PROVIDER_NOT_CONFIGURED],
            audit_refs: &[bridge::REF_AUDIT_TAMPER_MANUAL],
            integrity_refs: &[bridge::REF_INTEGRITY_TAMPER_SIGNAL],
            drill_in_refs: &[bridge::REF_DRILL_IN_TAMPER_MANUAL],
            manual_proof_requirements: &[
                bridge::REQUIREMENT_SERVICE_MANAGER_STOP_PROOF,
                bridge::REQUIREMENT_UNINSTALL_DETECTION_ARTIFACT,
                bridge::REQUIREMENT_SECURITY_REVIEW,
            ],
            boundary: bridge::BOUNDARY_TAMPER_MANUAL,
        },
    ]
}

fn entry_from_spec(
    spec: &EntrySpec,
    generated_at: GeneratedAtTextRef<'_>,
) -> V08IntegrityAlertStatusBridgeEntry {
    V08IntegrityAlertStatusBridgeEntry {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        bridge_entry_id: spec.bridge_entry_id.to_string(),
        integrity_alert_state: spec.integrity_alert_state,
        parent_visible_status: spec.parent_visible_status,
        notification_intent_state: spec.notification_intent_state,
        delivery_state: V08IntegrityAlertDeliveryState::NotDeliveredProviderNotConfigured,
        audit_state: spec.audit_state,
        reason_code_ref: spec.reason_code_ref.to_string(),
        status_ref: spec.status_ref.to_string(),
        notification_intent_refs: to_strings(StaticTextRefs(spec.notification_intent_refs)).0,
        notification_status_refs: to_strings(StaticTextRefs(spec.notification_status_refs)).0,
        audit_refs: to_strings(StaticTextRefs(spec.audit_refs)).0,
        integrity_refs: to_strings(StaticTextRefs(spec.integrity_refs)).0,
        drill_in_refs: to_strings(StaticTextRefs(spec.drill_in_refs)).0,
        manual_proof_requirements: to_strings(StaticTextRefs(spec.manual_proof_requirements)).0,
        boundary: spec.boundary.to_string(),
        provider_delivery_claimed: false,
        broad_blocking_claimed: false,
        tamper_resistance_claimed: false,
        mobile_enforcement_claimed: false,
        stealth_persistence_claimed: false,
        privilege_escalation_claimed: false,
        last_checked_at: generated_at.0.to_string(),
    }
}

fn to_strings(values: StaticTextRefs) -> BridgeTextList {
    BridgeTextList(values.0.iter().map(|value| (*value).to_string()).collect())
}
