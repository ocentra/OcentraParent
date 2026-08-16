use ocentra_parent_agent_protocol::activity_surface::{
    ActivityReadModelState, ActivityScreenReadModel, ActivityScreenReadModelRow,
    ActivitySurfaceRequest, ActivitySurfaceScope, ActivitySurfaceScopeKind,
};
use ocentra_parent_agent_protocol::app_game_adapter_dispatch_preflight::AppGameAdapterDispatchPreflightReadModel;
use ocentra_parent_agent_protocol::app_game_adapter_dispatch_result::AppGameAdapterDispatchResultReadModel;
use ocentra_parent_agent_protocol::app_game_child_runtime_transport_receipt::{
    AppGameChildRuntimeTransportReceiptReadModel, AppGameChildRuntimeTransportReceiptRow,
};
use ocentra_parent_agent_protocol::app_game_notification_readiness::{
    AppGameNotificationReadinessReadModel, AppGameNotificationReadinessRow,
};
use ocentra_parent_agent_protocol::app_game_platform_proof_status::{
    AppGamePlatformProofStatusReadModel, AppGamePlatformProofStatusRow,
};
use ocentra_parent_agent_protocol::app_game_policy_readiness::{
    AppGamePolicyReadinessReadModel, AppGamePolicyReadinessRow,
};
use ocentra_parent_agent_protocol::app_game_timer_parent_surface_read_model::AppGameTimerParentSurfaceReadModel;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogLevel};
use ocentra_parent_agent_protocol::network_flow::{
    ActivityNetworkEndpoint, ActivityNetworkFlowCounters, ActivityNetworkFlowDigest,
    ActivityNetworkFlowObservation, ActivityNetworkFlowReadModel,
    NETWORK_FLOW_READ_MODEL_FIELD_ACTIVE_ROWS,
    NETWORK_FLOW_READ_MODEL_FIELD_DELETED_EVIDENCE_REFERENCE_IDS,
    NETWORK_FLOW_READ_MODEL_FIELD_EXPORTABLE_ROWS,
    NETWORK_FLOW_READ_MODEL_FIELD_LATEST_TOMBSTONE_EVENT_ID,
    NETWORK_FLOW_READ_MODEL_FIELD_LATEST_TOMBSTONE_OBSERVED_AT,
    NETWORK_FLOW_READ_MODEL_FIELD_TOMBSTONE_ROWS,
};
use ocentra_parent_agent_protocol::tracking::read_model::TrackingReadModel;
use ocentra_parent_agent_protocol::transport::AgentEventName;
use ocentra_parent_agent_protocol::transport::{AgentEventEnvelope, AgentPeer, AgentPeerRole};
use ocentra_parent_agent_protocol::{
    ActivityEvidenceKind, ActivityEvidenceRef, ACTIVITY_QUERY_SCHEMA_VERSION,
};
use ocentra_schema::parent_ui_bridge::{ParentRouteId, ParentUiAction, ParentUiActionKind};
use ocentra_schema::parent_ui_bridge::{ParentRouteSnapshot, ParentUiActionResult};
use serde_json::{json, Value};

#[path = "../unit/parent_ui_bridge/common.rs"]
mod common;
#[path = "parent_ui_bridge/lan_replay_tests.rs"]
mod lan_replay_tests;
#[path = "parent_ui_bridge/policy_request_resolution_tests.rs"]
mod policy_request_resolution_tests;
#[path = "parent_ui_bridge/runtime_and_activity_tests.rs"]
mod runtime_and_activity_tests;
#[path = "parent_ui_bridge/snapshot_and_dispatch_tests.rs"]
mod snapshot_and_dispatch_tests;
#[path = "../../src/parent_ui_bridge/tests_support.rs"]
mod tests_support;

use ocentra_parent_runtime_core::parent_ui_bridge::{
    dispatch_parent_ui_action, load_parent_route_snapshot, load_parent_subscription_event,
};
use tests_support::require_ok;

const LAN_DISCOVERY_REPORTED_EVENT: &str = "agent.lan-pairing.browser-discovery.reported";
