#![forbid(unsafe_code)]

//! Parent/controller runtime ownership boundary.
//!
//! This crate owns parent desktop/mobile runtime orchestration,
//! controller-side event handling, local assistant handoff, discovery entry
//! points, and parent-visible service state. Child evidence logic must stay in
//! child runtime feature crates.

use ocentra_parent_agent_protocol::{
    ParentTrackingConfigUpdatedEvent, TrackingConfigUpdateTargetScope,
};

pub const CRATE_NAME: &str = "ocentra-parent-runtime-core";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParentRuntimeTarget {
    Household,
    ChildDevice,
    ParentOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChildRuntimeDispatchState {
    Required,
    NotRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChildAcknowledgementState {
    Required,
    NotRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParentRuntimeOriginState {
    TrustedLocalUi,
    Untrusted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChildRuntimePublishState {
    Publish,
    DoNotPublish,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParentAuditRetentionState {
    Retain,
    DoNotRetain,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChildAcknowledgementWaitState {
    Await,
    DoNotAwait,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParentRuntimeChangeRequest {
    pub target: ParentRuntimeTarget,
    pub origin_state: ParentRuntimeOriginState,
    pub child_runtime_dispatch_state: ChildRuntimeDispatchState,
    pub child_acknowledgement_state: ChildAcknowledgementState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParentRuntimeDispatchDecision {
    pub target: ParentRuntimeTarget,
    pub child_runtime_publish_state: ChildRuntimePublishState,
    pub parent_audit_retention_state: ParentAuditRetentionState,
    pub child_acknowledgement_wait_state: ChildAcknowledgementWaitState,
}

pub fn route_parent_runtime_change(
    request: ParentRuntimeChangeRequest,
) -> ParentRuntimeDispatchDecision {
    let dispatch_required =
        request.child_runtime_dispatch_state == ChildRuntimeDispatchState::Required;
    let trusted_origin = request.origin_state == ParentRuntimeOriginState::TrustedLocalUi;
    let child_target = matches!(
        request.target,
        ParentRuntimeTarget::Household | ParentRuntimeTarget::ChildDevice
    );
    let publish_to_child_runtime = dispatch_required && trusted_origin && child_target;

    ParentRuntimeDispatchDecision {
        target: request.target,
        child_runtime_publish_state: if publish_to_child_runtime {
            ChildRuntimePublishState::Publish
        } else {
            ChildRuntimePublishState::DoNotPublish
        },
        parent_audit_retention_state: ParentAuditRetentionState::Retain,
        child_acknowledgement_wait_state: if publish_to_child_runtime
            && request.child_acknowledgement_state == ChildAcknowledgementState::Required
        {
            ChildAcknowledgementWaitState::Await
        } else {
            ChildAcknowledgementWaitState::DoNotAwait
        },
    }
}

pub fn route_parent_tracking_config_update_event(
    event: &ParentTrackingConfigUpdatedEvent,
    child_acknowledgement_state: ChildAcknowledgementState,
) -> ParentRuntimeDispatchDecision {
    route_parent_tracking_config_update_event_from_origin(
        event,
        child_acknowledgement_state,
        ParentRuntimeOriginState::TrustedLocalUi,
    )
}

pub fn route_parent_tracking_config_update_event_from_origin(
    event: &ParentTrackingConfigUpdatedEvent,
    child_acknowledgement_state: ChildAcknowledgementState,
    origin_state: ParentRuntimeOriginState,
) -> ParentRuntimeDispatchDecision {
    route_parent_runtime_change(ParentRuntimeChangeRequest {
        target: parent_runtime_target_from_tracking_scope(&event.target.scope),
        origin_state,
        child_runtime_dispatch_state: ChildRuntimeDispatchState::Required,
        child_acknowledgement_state,
    })
}

pub fn parent_runtime_target_from_tracking_scope(
    scope: &TrackingConfigUpdateTargetScope,
) -> ParentRuntimeTarget {
    match scope {
        TrackingConfigUpdateTargetScope::Family | TrackingConfigUpdateTargetScope::DeviceGroup => {
            ParentRuntimeTarget::Household
        }
        TrackingConfigUpdateTargetScope::ChildProfile
        | TrackingConfigUpdateTargetScope::ChildDevice => ParentRuntimeTarget::ChildDevice,
    }
}
