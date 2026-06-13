#![forbid(unsafe_code)]

//! Parent/controller runtime ownership boundary.
//!
//! This crate owns parent desktop/mobile runtime orchestration,
//! controller-side event handling, local assistant handoff, discovery entry
//! points, and parent-visible service state. Child evidence logic must stay in
//! child runtime feature crates.

mod tracking_config_update_flow;
mod tracking_dispatch;

pub const CRATE_NAME: &str = "ocentra-parent-runtime-core";

pub use tracking_dispatch::{
    parent_runtime_target_from_tracking_scope, parent_runtime_tracking_dispatch_evaluated_event,
    parent_runtime_tracking_dispatch_evaluated_event_from_origin,
    route_parent_runtime_change, route_parent_tracking_config_update_event,
    route_parent_tracking_config_update_event_from_origin, ChildAcknowledgementState,
    ChildAcknowledgementWaitState, ChildRuntimeDispatchState, ChildRuntimePublishState,
    ParentAuditRetentionState, ParentRuntimeChangeRequest, ParentRuntimeDispatchDecision,
    ParentRuntimeDispatchId, ParentRuntimeOriginState, ParentRuntimeTarget,
    ParentRuntimeTrackingDispatchEvaluatedEvent, PARENT_RUNTIME_TRACKING_DISPATCH_EVALUATED_EVENT_TYPE,
};
pub use tracking_config_update_flow::{
    publish_parent_tracking_config_updated_event_flow, ParentTrackingConfigUpdateEventFlow,
    ParentTrackingConfigUpdateEventFlowReport,
};
