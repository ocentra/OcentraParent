use ocentra_parent_agent_protocol::network_flow::ActivityNetworkFlowReadModel;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceReadModel;
use ocentra_parent_agent_protocol::tracking::read_model::{
    TrackingReadModel, TrackingReadModelCount, TrackingReadModelRow,
};
use ocentra_parent_agent_protocol::network_flow::{
    ActivityNetworkEndpoint, ActivityNetworkFlowCounters, ActivityNetworkFlowObservation,
};
use ocentra_parent_agent_protocol::lan_pairing::*;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::*;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::signed_discovery_relay_spine::*;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::*;
use ocentra_schema::parent_ui_bridge::{
    ParentActivityEvidenceRefSnapshot, ParentActivityNetworkEndpointSnapshot,
    ParentActivityNetworkFlowCountersSnapshot, ParentActivityNetworkFlowObservationSnapshot,
    ParentActivityNetworkFlowReadModelSnapshot, ParentActivityTrackingReadModelCountSnapshot,
    ParentActivityTrackingReadModelRowSnapshot, ParentActivityTrackingReadModelSnapshot,
    ParentBridgeConnectionState, ParentLanAddDeviceReadModelSnapshot,
    ParentLanAddDeviceScanSummarySnapshot, ParentLanServiceIdentityProbeEvidenceSnapshot,
    ParentNetworkEvidenceSummarySnapshot, ParentNetworkRuntimeEventChainStreamSnapshot,
    ParentPolicyPreviewReadModelSnapshot, ParentPortalRowSnapshot, ParentPortalShellStatusCardId,
    ParentPortalTone, ParentRouteDataSource, ParentRouteId,
};
use ocentra_schema::parent_ui_bridge::*;
use self::lan_history::*;
use self::lan_household::*;
use serde_json::Value;
use serde::Serialize;

#[path = "route_metadata/common.rs"]
pub(super) mod common;
#[path = "route_metadata/lan_history.rs"]
pub(super) mod lan_history;
#[path = "route_metadata/lan_household.rs"]
pub(super) mod lan_household;
#[path = "route_metadata/network.rs"]
pub(super) mod network;
#[path = "route_metadata/tracking.rs"]
pub(super) mod tracking;

pub(super) fn portal_row_snapshot(
    label: &str,
    order: u16,
    primary_area: &str,
    trend: String,
    tone: ParentPortalTone,
) -> ParentPortalRowSnapshot {
    common::portal_row_snapshot(label, order, primary_area, trend, tone)
}

pub(super) fn serialized_enum_label<T: Serialize>(value: &T) -> String {
    common::serialized_enum_label(value)
}

pub(super) fn data_source_label(data_source: &ParentRouteDataSource) -> &'static str {
    common::data_source_label(data_source)
}

pub(super) fn route_capability_state_for_data_source(
    data_source: &ParentRouteDataSource,
) -> &'static str {
    common::route_capability_state_for_data_source(data_source)
}

pub(super) fn season_label_for_connection(
    connection_state: &ParentBridgeConnectionState,
) -> &'static str {
    common::season_label_for_connection(connection_state)
}

pub(super) fn global_connection_state_for_connection(
    connection_state: &ParentBridgeConnectionState,
) -> &'static str {
    common::global_connection_state_for_connection(connection_state)
}

pub(super) fn connection_tone(connection_state: &ParentBridgeConnectionState) -> ParentPortalTone {
    common::connection_tone(connection_state)
}

pub(super) fn route_capability_tone(data_source: &ParentRouteDataSource) -> ParentPortalTone {
    common::route_capability_tone(data_source)
}

pub(super) fn data_source_tone(data_source: &ParentRouteDataSource) -> ParentPortalTone {
    common::data_source_tone(data_source)
}

pub(super) fn parent_portal_shell_status_card_id(
    segment: &'static str,
) -> ParentPortalShellStatusCardId {
    common::parent_portal_shell_status_card_id(segment)
}

pub(super) fn current_lan_add_device_read_model_value(
    read_model: Option<&LanBrowserAddDeviceReadModel>,
) -> Option<ParentLanAddDeviceReadModelSnapshot> {
    read_model.map(common::current_lan_add_device_read_model_value)
}

pub(super) fn network_flow_read_model_snapshot(
    read_model: &ActivityNetworkFlowReadModel,
) -> ParentActivityNetworkFlowReadModelSnapshot {
    network::network_flow_read_model_snapshot(read_model)
}

pub(super) fn network_evidence_summary_snapshot(
    network_runtime_event_chain_stream: Option<&ParentNetworkRuntimeEventChainStreamSnapshot>,
    policy_preview_read_model: Option<&ParentPolicyPreviewReadModelSnapshot>,
) -> Option<ParentNetworkEvidenceSummarySnapshot> {
    network::network_evidence_summary_snapshot(
        network_runtime_event_chain_stream,
        policy_preview_read_model,
    )
}

pub(crate) fn tracking_read_model_snapshot(
    read_model: &TrackingReadModel,
) -> ParentActivityTrackingReadModelSnapshot {
    tracking::tracking_read_model_snapshot(read_model)
}

pub(super) fn route_title(route: &ParentRouteId) -> &'static str {
    tracking::route_title(route)
}
