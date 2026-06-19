use std::collections::BTreeMap;

use ocentra_parent_agent_protocol::{
    constants::v08_integrity_alert_status_bridge as bridge,
    integrity_alert_status_bridge::{V08IntegrityAlertState, V08IntegrityAlertStatusBridgeEntry},
    policy_constants,
};

use super::integrity_alert_status_bridge_read_model::v08_integrity_alert_status_bridge_read_model;

#[test]
fn integrity_alert_status_bridge_read_model_covers_required_parent_visible_states() {
    let read_model =
        v08_integrity_alert_status_bridge_read_model(policy_constants::TEST_EVALUATED_AT);
    let state_counts = count_states(&read_model.entries);

    assert_eq!(read_model.read_model_id, bridge::READ_MODEL_ID);
    assert_eq!(read_model.entries.len(), 4);
    assert_eq!(state_count(&state_counts, bridge::STATE_PERMISSION_LOSS), 1);
    assert_eq!(state_count(&state_counts, bridge::STATE_STALE_HEARTBEAT), 1);
    assert_eq!(
        state_count(&state_counts, bridge::STATE_STOPPED_OR_REMOVED),
        1
    );
    assert_eq!(
        state_count(&state_counts, bridge::STATE_TAMPER_MANUAL_REQUIRED),
        1
    );
    assert!(read_model
        .source_read_model_ids
        .contains(&bridge::SOURCE_REPORTS_NOTIFICATIONS_SYNC.to_string()));
}

#[test]
fn integrity_alert_status_bridge_read_model_preserves_non_claims_and_refs() {
    let read_model =
        v08_integrity_alert_status_bridge_read_model(policy_constants::TEST_EVALUATED_AT);

    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.provider_delivery_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.broad_blocking_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.tamper_resistance_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.mobile_enforcement_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.stealth_persistence_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.privilege_escalation_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.audit_refs.is_empty()));
    assert!(read_model.entries.iter().all(|entry| {
        entry
            .notification_status_refs
            .contains(&bridge::REF_NOTIFICATION_STATUS_PROVIDER_NOT_CONFIGURED.to_string())
    }));
}

fn count_states(entries: &[V08IntegrityAlertStatusBridgeEntry]) -> BTreeMap<&'static str, usize> {
    entries.iter().fold(BTreeMap::new(), |mut counts, entry| {
        *counts
            .entry(state_name(entry.integrity_alert_state))
            .or_default() += 1;
        counts
    })
}

fn state_name(state: V08IntegrityAlertState) -> &'static str {
    match state {
        V08IntegrityAlertState::PermissionLoss => bridge::STATE_PERMISSION_LOSS,
        V08IntegrityAlertState::StaleHeartbeat => bridge::STATE_STALE_HEARTBEAT,
        V08IntegrityAlertState::StoppedOrRemoved => bridge::STATE_STOPPED_OR_REMOVED,
        V08IntegrityAlertState::TamperManualRequired => bridge::STATE_TAMPER_MANUAL_REQUIRED,
    }
}

fn state_count(counts: &BTreeMap<&'static str, usize>, state: &'static str) -> usize {
    *counts.get(state).unwrap_or(&0)
}
