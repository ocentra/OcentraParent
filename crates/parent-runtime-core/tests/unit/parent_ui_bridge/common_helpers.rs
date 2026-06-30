
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanBrowserAddDeviceReadModel, LanDiscoveryEventHistory, LanDiscoveryEventHistoryState,
    LanDiscoveryEventKind, LanDiscoveryEventRow,
};
use ocentra_schema::parent_ui_bridge::{
    ParentRouteLiveActivitySnapshot, ParentRoutePeerRole,
};
use serde_json::{json, Value};

use super::super::tests_support::sample_lan_read_model;
use super::super::LAN_DISCOVERY_REPORTED_EVENT;

pub(crate) fn require_some<T>(value: Option<T>, context: &str) -> T {
    match value {
        Some(value) => value,
        None => unreachable!("{context}: expected Some(_)"),
    }
}

pub(crate) fn assert_snapshot_field_is_none(
    value: &Option<super::super::ParentRouteSnapshot>,
    selector: impl FnOnce(&ParentRouteLiveActivitySnapshot) -> bool,
) {
    assert_eq!(
        value
            .as_ref()
            .and_then(|snapshot| snapshot.live_activity.as_ref())
            .map(selector),
        Some(false)
    );
}

pub(crate) fn assert_lan_scan_snapshot(result: &super::super::ParentUiActionResult) {
    let event = result
        .snapshot
        .as_ref()
        .and_then(|snapshot| snapshot.live_activity.as_ref())
        .and_then(|live_activity| live_activity.lan_pairing_browser_discovery_event.as_ref())
        .unwrap_or_else(|| unreachable!("LAN scan snapshot includes discovery event"));

    assert_eq!(
        result.message,
        "parent Rust facade requested LAN pairing browser discovery scan"
    );
    assert_eq!(
        result
            .snapshot
            .as_ref()
            .and_then(|snapshot| snapshot.live_activity.as_ref())
            .and_then(|live_activity| live_activity.lan_add_device_read_model.as_ref())
            .map(|_| ()),
        Some(())
    );
    assert_eq!(event.event.as_deref(), Some(LAN_DISCOVERY_REPORTED_EVENT));
    assert_eq!(
        event.correlation_id.as_ref().map(|value| value.as_str()),
        Some("lan")
    );
    assert_eq!(
        event.source_peer_id.as_ref().map(|value| value.as_str()),
        Some(constants::peer::LOCAL_DEV_AGENT)
    );
    assert_eq!(event.source_role, Some(ParentRoutePeerRole::AgentService));
    assert_eq!(
        event.target_peer_id.as_ref().map(|value| value.as_str()),
        Some(constants::peer::PORTAL_DEV)
    );
    assert_eq!(event.target_role, Some(ParentRoutePeerRole::Portal));
    assert_eq!(result.events.len(), 2);
    assert_eq!(
        result
            .events
            .first()
            .and_then(|value| value.event.as_deref()),
        Some("agent.connection.ready")
    );
    assert_eq!(
        result
            .events
            .last()
            .and_then(|value| value.event.as_deref()),
        Some(LAN_DISCOVERY_REPORTED_EVENT)
    );
}

pub(crate) fn assert_json_field_eq(value: Option<&Value>, field: &str, expected: &str) {
    assert_eq!(
        value
            .and_then(|value| value.get(field))
            .and_then(|value| value.as_str()),
        Some(expected),
    );
}

pub(crate) fn assert_panel_detail_value(details: &Value, label: &str, expected: &str) {
    let detail = details
        .as_array()
        .and_then(|details| {
            details
                .iter()
                .find(|detail| detail.get("label").and_then(|value| value.as_str()) == Some(label))
        })
        .expect("expected matching panel detail");
    assert_eq!(
        detail.get("value").and_then(|value| value.as_str()),
        Some(expected),
    );
}

pub(crate) fn sample_lan_read_model_with_explicit_history() -> LanBrowserAddDeviceReadModel {
    let mut read_model = sample_lan_read_model();
    read_model.discovery_event_history = LanDiscoveryEventHistory {
        schema_version: 1,
        generated_at: "2026-06-23T00:00:03Z".to_string(),
        state: LanDiscoveryEventHistoryState::Ready,
        latest_event_id: None,
        latest_observed_at: None,
        rows: vec![
            LanDiscoveryEventRow {
                schema_version: 1,
                event_id: "lan-history-1".to_string(),
                event_kind: LanDiscoveryEventKind::ScanStarted,
                occurred_at: "2026-06-23T00:00:01Z".to_string(),
                previous_event_id: None,
                scan_session_id: Some("lan-scan-1".to_string()),
                affected_device_id: None,
                evidence_id: None,
                summary: "LAN discovery scan started".to_string(),
            },
            LanDiscoveryEventRow {
                schema_version: 1,
                event_id: "lan-history-2".to_string(),
                event_kind: LanDiscoveryEventKind::DeviceFound,
                occurred_at: "2026-06-23T00:00:02Z".to_string(),
                previous_event_id: Some("lan-history-1".to_string()),
                scan_session_id: Some("lan-scan-1".to_string()),
                affected_device_id: Some("network-neighbor-1".to_string()),
                evidence_id: Some("evidence-1".to_string()),
                summary: "Study Laptop was found on the local network".to_string(),
            },
        ],
    };
    read_model
}

pub(crate) fn sample_lan_read_model_with_history_state(
    state: LanDiscoveryEventHistoryState,
) -> LanBrowserAddDeviceReadModel {
    let mut read_model = sample_lan_read_model();
    read_model.discovery_event_history = LanDiscoveryEventHistory {
        schema_version: 1,
        generated_at: "2026-06-23T00:00:03Z".to_string(),
        state,
        latest_event_id: None,
        latest_observed_at: None,
        rows: Vec::new(),
    };
    read_model
}

pub(crate) fn assert_app_game_snapshots_are_empty(
    dispatch_execute: &super::super::ParentUiActionResult,
    timer_setup: &super::super::ParentUiActionResult,
) {
    assert_snapshot_field_is_none(&dispatch_execute.snapshot, |live_activity| {
        live_activity
            .app_game_adapter_dispatch_panel
            .as_ref()
            .map(|panel| {
                !panel.preflight_panel.rows.is_empty() || !panel.result_panel.rows.is_empty()
            })
            .unwrap_or(false)
    });
    assert_snapshot_field_is_none(&timer_setup.snapshot, |live_activity| {
        live_activity
            .app_game_timer_parent_surface_panel
            .as_ref()
            .map(|panel| {
                !panel.rows.is_empty()
                    || !panel.parent_action_rows.is_empty()
                    || !panel.parent_preference_setup_rows.is_empty()
            })
            .unwrap_or(false)
    });
}

pub(crate) fn assert_network_policy_bridge_snapshot(
    live_activity: &serde_json::Value,
    expected_streamed_event_count: serde_json::Value,
) {
    assert_eq!(
        live_activity["networkEvidenceSummary"]["analyzerAlertRef"],
        json!("event.network.analyzer.alert.1")
    );
    assert_eq!(
        live_activity["networkEvidenceSummary"]["detectionResultRef"],
        json!("event.network.detection.result.1")
    );
    assert_eq!(
        live_activity["networkEvidenceSummary"]["aiAuditRef"],
        json!("event.ai.analysis.completed.1")
    );
    assert_eq!(
        live_activity["networkEvidenceSummary"]["riskBudgetRef"],
        json!("event.network.risk-budget.1")
    );
    assert_eq!(
        live_activity["networkEvidenceSummary"]["policyDecisionRef"],
        json!("event.policy.decision.completed.1")
    );
    assert_eq!(
        live_activity["networkEvidenceSummary"]["networkEvidenceGrade"],
        json!("A")
    );
    assert_eq!(
        live_activity["networkEvidenceSummary"]["interventionResultRef"],
        json!("event.enforcement.result.observed.1")
    );
    assert_eq!(
        live_activity["networkRuntimeEventChainStream"]["streamedEventCount"],
        expected_streamed_event_count
    );
    assert_eq!(
        live_activity["networkRuntimeEventChainStream"]["events"][0]["eventType"],
        json!("ai.analysis.completed")
    );
    assert_eq!(
        live_activity["networkRuntimeEventChainStream"]["events"][1]["eventType"],
        json!("policy.decision.completed")
    );
    assert_eq!(
        live_activity["networkRuntimeEventChainStream"]["events"][2]["eventType"],
        json!("enforcement.result.observed")
    );
    assert_eq!(
        live_activity["policyPreviewPanel"]["summaryDetails"][1]["label"],
        json!("Policy check")
    );
    assert_eq!(
        live_activity["policyPreviewPanel"]["summaryDetails"][1]["value"],
        json!("policy-preview.network.1")
    );
    assert_eq!(
        live_activity["policyPreviewPanel"]["summaryDetails"][4]["value"],
        json!("Proof missing")
    );
}
