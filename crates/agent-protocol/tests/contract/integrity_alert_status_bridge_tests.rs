use crate::{
    constants::{self, v08_integrity_alert_status_bridge as bridge},
    integrity_alert_status_bridge::{
        V08IntegrityAlertAuditState, V08IntegrityAlertDeliveryState,
        V08IntegrityAlertNotificationIntentState, V08IntegrityAlertParentVisibleStatus,
        V08IntegrityAlertState, V08IntegrityAlertStatusBridgeEntry,
        V08IntegrityAlertStatusBridgeReadModel,
    },
    policy_constants,
};
use ocentra_eventing::expect_value::ExpectValue;

#[test]
fn integrity_alert_status_bridge_serializes_stable_state_values() {
    assert_eq!(
        serde_json::to_value(V08IntegrityAlertState::StoppedOrRemoved)
            .expect_value(constants::error::AGENT_EVENT_SERIALIZES),
        bridge::STATE_STOPPED_OR_REMOVED
    );
    assert_eq!(
        serde_json::to_value(V08IntegrityAlertParentVisibleStatus::TamperReviewRequired)
            .expect_value(constants::error::AGENT_EVENT_SERIALIZES),
        bridge::STATUS_TAMPER_REVIEW_REQUIRED
    );
    assert_eq!(
        serde_json::to_value(V08IntegrityAlertDeliveryState::NotDeliveredProviderNotConfigured)
            .expect_value(constants::error::AGENT_EVENT_SERIALIZES),
        bridge::DELIVERY_PROVIDER_NOT_CONFIGURED
    );
}

#[test]
fn integrity_alert_status_bridge_preserves_non_claim_flags_and_refs() {
    let read_model = V08IntegrityAlertStatusBridgeReadModel {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        read_model_id: bridge::READ_MODEL_ID.to_string(),
        generated_at: policy_constants::TEST_EVALUATED_AT.to_string(),
        source_read_model_ids: vec![bridge::SOURCE_ENFORCEMENT_INTEGRITY_RUNTIME_AUDIT.to_string()],
        entries: vec![entry()],
    };
    let reparsed = serde_json::from_value::<V08IntegrityAlertStatusBridgeReadModel>(
        serde_json::to_value(read_model).expect_value(constants::error::AGENT_EVENT_SERIALIZES),
    )
    .expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    let reparsed_entry = reparsed.entries.first().expect_value(bridge::READ_MODEL_ID);

    assert_eq!(reparsed.read_model_id, bridge::READ_MODEL_ID);
    assert_eq!(
        reparsed_entry.delivery_state,
        V08IntegrityAlertDeliveryState::NotDeliveredProviderNotConfigured
    );
    assert!(!reparsed_entry.provider_delivery_claimed);
    assert!(!reparsed_entry.broad_blocking_claimed);
    assert!(!reparsed_entry.tamper_resistance_claimed);
    assert!(!reparsed_entry.mobile_enforcement_claimed);
    assert!(!reparsed_entry.stealth_persistence_claimed);
    assert!(!reparsed_entry.privilege_escalation_claimed);
    assert_eq!(
        reparsed_entry.notification_status_refs,
        vec![bridge::REF_NOTIFICATION_STATUS_PROVIDER_NOT_CONFIGURED.to_string()]
    );
}

fn entry() -> V08IntegrityAlertStatusBridgeEntry {
    V08IntegrityAlertStatusBridgeEntry {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        bridge_entry_id: bridge::ENTRY_PERMISSION_LOSS.to_string(),
        integrity_alert_state: V08IntegrityAlertState::PermissionLoss,
        parent_visible_status: V08IntegrityAlertParentVisibleStatus::PermissionActionRequired,
        notification_intent_state: V08IntegrityAlertNotificationIntentState::IntentCreated,
        delivery_state: V08IntegrityAlertDeliveryState::NotDeliveredProviderNotConfigured,
        audit_state: V08IntegrityAlertAuditState::AuditRefBacked,
        reason_code_ref: bridge::REF_REASON_PERMISSION_LOSS.to_string(),
        status_ref: bridge::REF_STATUS_PERMISSION_ACTION_REQUIRED.to_string(),
        notification_intent_refs: vec![bridge::REF_NOTIFICATION_INTENT_PERMISSION_LOSS.to_string()],
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
    }
}
