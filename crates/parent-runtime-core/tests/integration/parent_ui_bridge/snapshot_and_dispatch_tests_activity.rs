use super::*;

use crate::parent_ui_bridge::common::events::activity::{
    app_use_read_model_response_event, games_read_model_response_event,
};
use ocentra_parent_agent_protocol::browser_managed::{
    BrowserManagedSessionStatus, BrowserManagedState, BrowserQueryVisibilityLabel,
};
use ocentra_parent_agent_protocol::logging::LogLevel;
use ocentra_parent_agent_protocol::transport::{AgentEventEnvelope, AgentPeer, AgentPeerRole};
use ocentra_parent_agent_protocol::{
    BrowserCapabilityStatus, BrowserCustodyLabel, BROWSER_EVIDENCE_SCHEMA_VERSION,
};

#[test]
fn activity_route_projects_every_visible_activity_read_model() {
    let value = projected_route_snapshot_json(
        ParentRouteId::Activity,
        activity_route_projection(),
        TestContext("activity route projects visible activity read models"),
    );
    let live_activity = &value["liveActivity"];

    assert_eq!(value["connectionState"], json!("connected"));
    assert_eq!(live_activity["activityScreenReadModel"]["ok"], json!(true));
    assert_eq!(
        live_activity["activityAppUseReadModel"]["value"]["summary"],
        json!("App-use activity is ready.")
    );
    assert_eq!(
        live_activity["activityBrowserReadModel"]["value"]["summary"],
        json!("Browser activity ready")
    );
    assert_eq!(
        live_activity["activityGamesReadModel"]["value"]["summary"],
        json!("Games activity is ready.")
    );
    assert_eq!(
        live_activity["networkFlowReadModel"]["capabilityStatus"],
        json!("reported")
    );
    assert_eq!(
        live_activity["activityTrackingReadModel"]["ok"],
        json!(true)
    );
}

#[test]
fn capability_status_route_projects_real_service_capability_sources() {
    let value = projected_route_snapshot_json(
        ParentRouteId::CapabilityStatus,
        capability_status_projection(),
        TestContext("capability status route projects service read models"),
    );
    let live_activity = &value["liveActivity"];

    assert_eq!(value["connectionState"], json!("connected"));
    assert_eq!(live_activity["activityScreenReadModel"]["ok"], json!(true));
    assert_eq!(live_activity["activityAppUseReadModel"]["ok"], json!(true));
    assert_eq!(live_activity["activityGamesReadModel"]["ok"], json!(true));
    assert_eq!(
        live_activity["networkFlowReadModel"]["capabilityStatus"],
        json!("reported")
    );
    assert_eq!(
        live_activity["browserManagedStatus"]["capabilityStatus"],
        json!("tab-list-only")
    );
    assert_eq!(
        live_activity["activityTrackingReadModel"]["ok"],
        json!(true)
    );
}

fn capability_status_projection() -> Vec<ParentAgentServiceProjectionResponse> {
    let mut responses = lan_status_projection(sample_lan_read_model());
    responses.extend([
        projection_response(
            AgentCommandName::AgentNetworkFlowReadModelGet,
            network_flow_response_event(),
        ),
        projection_response(
            AgentCommandName::AgentNetworkRuntimeEventChainStreamGet,
            network_runtime_event_chain_response_event(),
        ),
        projection_response(
            AgentCommandName::AgentPolicyPreviewReadModelGet,
            policy_preview_response_event(),
        ),
        projection_response(
            AgentCommandName::AgentActivityTrackingReadModelGet,
            tracking_read_model_response_event(),
        ),
        projection_response(
            AgentCommandName::AgentActivityScreenReadModelGet,
            screen_read_model_response_event(),
        ),
        projection_response(
            AgentCommandName::AgentActivityAppUseReadModelGet,
            app_use_read_model_response_event(),
        ),
        projection_response(
            AgentCommandName::AgentActivityGamesReadModelGet,
            games_read_model_response_event(),
        ),
        projection_response(
            AgentCommandName::AgentBrowserManagedBridgePoll,
            browser_managed_status_response_event(),
        ),
    ]);
    responses
}

fn browser_managed_status_response_event() -> AgentEventEnvelope {
    let status = BrowserManagedSessionStatus {
        schema_version: BROWSER_EVIDENCE_SCHEMA_VERSION,
        checked_at: "2026-06-27T17:40:00Z".to_string(),
        managed_browser_session_id: None,
        browser_family: None,
        browser_channel: None,
        browser_version: None,
        profile_id: None,
        profile_path_ref: None,
        profile_root_ref: None,
        profile_scope_id: None,
        profile_lifecycle_state: None,
        policy_revision: None,
        process_id: None,
        bridge_kind: None,
        bridge_endpoint_ref: None,
        unmanaged_process_name: None,
        unmanaged_executable_path_ref: None,
        unmanaged_signature_ref: None,
        unmanaged_process_hash_ref: None,
        unmanaged_process_kind: None,
        unmanaged_detection_confidence: None,
        unmanaged_detection_reason: None,
        managed_state: BrowserManagedState::BridgeConnected,
        capability_status: BrowserCapabilityStatus::TabListOnly,
        degraded_reason: None,
        started_at: None,
        custody_label: BrowserCustodyLabel::ChildDeviceLocal,
        query_visibility: BrowserQueryVisibilityLabel::LiveLocal,
    };
    let mut payload = std::collections::BTreeMap::new();
    payload.insert(
        constants::field::BROWSER_MANAGED_STATUS_JSON.to_string(),
        LogFieldValue::String(require_ok(
            serde_json::to_string(&status),
            "browser managed status serializes",
        )),
    );
    AgentEventEnvelope {
        schema_version: 1,
        event_id: "agent.browser.managed.status.reported-capability".to_string(),
        correlation_id: "browser-managed-capability".to_string(),
        sent_at: "2026-06-27T17:40:00Z".to_string(),
        source: AgentPeer {
            peer_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            role: AgentPeerRole::AgentService,
        },
        target: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        event: AgentEventName::AgentBrowserManagedStatusReported,
        severity: LogLevel::Info,
        payload: payload.into(),
        snapshot: None,
    }
}
