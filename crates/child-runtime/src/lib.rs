#![forbid(unsafe_code)]

//! Child runtime orchestration ownership boundary.
//!
//! This crate composes child-side feature domains, shared eventing, runtime
//! preflight gates, and parent-to-child command application. Parent runtime and
//! portal UI code must route through this boundary instead of owning child
//! tracking, app/game, browser, LAN, network, screen, AI, policy, notification,
//! storage, remote-access, or enforcement decisions directly.

mod child_domain_runtime_flow;
mod runtime_gate;
mod tracking_config_update_flow;
mod tracking_runtime_flow;

pub const CRATE_NAME: &str = "ocentra-child-runtime";

pub fn tracking_runtime_crate_name() -> &'static str {
    ocentra_tracking_core::CRATE_NAME
}

pub use child_domain_runtime_flow::{
    publish_child_domain_observed_event, publish_default_child_domain_runtime_flows,
    ChildDomainRuntimeEventFlow, ChildDomainRuntimeFlowReport,
};
pub use ocentra_parent_agent_protocol::{
    child_tracking_config_updated_event_from_parent,
    parent_tracking_config_updated_event_from_command,
    tracking_config_update_applied_event_from_child, ChildTrackingConfigUpdatedEvent,
    ParentTrackingConfigUpdatedEvent, TrackingConfigEffectiveState,
    TrackingConfigUpdateAppliedEvent, TrackingConfigUpdateEventName,
    TrackingConfigUpdateResponse, TrackingConfigUpdateResponseState, TrackingConfigUpdateTarget,
    TrackingConfigUpdateTargetScope,
};
pub use ocentra_tracking_core::TrackingRetentionSettingsWriteAppliedState;
pub use runtime_gate::{
    child_runtime_remote_upload_allowed, evaluate_child_runtime_enforcement,
    evaluate_child_runtime_preflight, evaluate_child_runtime_remote_access,
    record_child_runtime_preflight_decision, ChildRuntimeAggregateId,
    ChildRuntimeEnforcementDecision, ChildRuntimeManualReviewState, ChildRuntimePreflightDecision,
    ChildRuntimePreflightDecisionId, ChildRuntimePreflightDecisionRecordedEvent,
    ChildRuntimePreflightInput, ChildRuntimePreflightRequestId,
    ChildRuntimePreflightRequestedEvent, ChildRuntimeRemoteAccessDecision, ChildRuntimeStartState,
    CHILD_RUNTIME_PREFLIGHT_DECISION_RECORDED_EVENT_TYPE,
    CHILD_RUNTIME_PREFLIGHT_REQUESTED_EVENT_TYPE,
};
pub use tracking_runtime_flow::{
    publish_child_tracking_location_observed_event, TrackingRuntimeEventFlow,
    TrackingRuntimeEventFlowReport,
};
pub use tracking_config_update_flow::{
    publish_parent_tracking_config_updated_event, subscribe_child_tracking_config_updated_events,
    subscribe_child_tracking_config_applied_events, subscribe_parent_tracking_config_updated_events,
    tracking_config_update_applied_event_type, tracking_config_update_child_event_type,
    tracking_config_update_event_bus, tracking_config_update_parent_event_type,
    tracking_retention_settings_durable_store_path, TrackingConfigUpdateAppliedReport,
    TrackingConfigUpdateEventFlow, TrackingConfigUpdateEventFlowReport,
    TrackingConfigUpdateEventState,
};
