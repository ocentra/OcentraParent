use ocentra_eventing::DomainEvent;
use ocentra_parent_agent_protocol::{
    constants, default_tracking_retention_settings_write_request, AgentRoute,
    ParentTrackingConfigUpdatedEvent, TrackingConfigUpdateTarget, TrackingConfigUpdateTargetScope,
    TrackingSourceMessageId, TrackingSourcePeerId, TrackingTargetDeviceId, TrackingTargetPlatform,
};
use ocentra_parent_runtime_core::{
    parent_runtime_target_from_tracking_scope, parent_runtime_tracking_dispatch_evaluated_event,
    route_parent_runtime_change, route_parent_tracking_config_update_event,
    route_parent_tracking_config_update_event_from_origin, ChildAcknowledgementState,
    ChildAcknowledgementWaitState, ChildRuntimeDispatchState, ChildRuntimePublishState,
    ParentAuditRetentionState, ParentRuntimeChangeRequest, ParentRuntimeOriginState,
    ParentRuntimeTarget, PARENT_RUNTIME_TRACKING_DISPATCH_EVALUATED_EVENT_TYPE,
};

#[test]
fn child_device_change_is_published_and_can_await_acknowledgement() {
    let decision = route_parent_runtime_change(ParentRuntimeChangeRequest {
        target: ParentRuntimeTarget::ChildDevice,
        origin_state: ParentRuntimeOriginState::TrustedLocalUi,
        child_runtime_dispatch_state: ChildRuntimeDispatchState::Required,
        child_acknowledgement_state: ChildAcknowledgementState::Required,
    });

    assert_eq!(
        decision.child_runtime_publish_state,
        ChildRuntimePublishState::Publish
    );
    assert_eq!(
        decision.parent_audit_retention_state,
        ParentAuditRetentionState::Retain
    );
    assert_eq!(
        decision.child_acknowledgement_wait_state,
        ChildAcknowledgementWaitState::Await
    );
}

#[test]
fn parent_only_change_stays_in_parent_runtime() {
    let decision = route_parent_runtime_change(ParentRuntimeChangeRequest {
        target: ParentRuntimeTarget::ParentOnly,
        origin_state: ParentRuntimeOriginState::TrustedLocalUi,
        child_runtime_dispatch_state: ChildRuntimeDispatchState::Required,
        child_acknowledgement_state: ChildAcknowledgementState::Required,
    });

    assert_eq!(
        decision.child_runtime_publish_state,
        ChildRuntimePublishState::DoNotPublish
    );
    assert_eq!(
        decision.parent_audit_retention_state,
        ParentAuditRetentionState::Retain
    );
    assert_eq!(
        decision.child_acknowledgement_wait_state,
        ChildAcknowledgementWaitState::DoNotAwait
    );
}

#[test]
fn household_change_is_published_without_waiting_when_acknowledgement_is_not_required() {
    let decision = route_parent_runtime_change(ParentRuntimeChangeRequest {
        target: ParentRuntimeTarget::Household,
        origin_state: ParentRuntimeOriginState::TrustedLocalUi,
        child_runtime_dispatch_state: ChildRuntimeDispatchState::Required,
        child_acknowledgement_state: ChildAcknowledgementState::NotRequired,
    });

    assert_eq!(
        decision.child_runtime_publish_state,
        ChildRuntimePublishState::Publish
    );
    assert_eq!(
        decision.parent_audit_retention_state,
        ParentAuditRetentionState::Retain
    );
    assert_eq!(
        decision.child_acknowledgement_wait_state,
        ChildAcknowledgementWaitState::DoNotAwait
    );
}

#[test]
fn explicit_no_child_dispatch_stays_parent_only_even_for_child_target() {
    let decision = route_parent_runtime_change(ParentRuntimeChangeRequest {
        target: ParentRuntimeTarget::ChildDevice,
        origin_state: ParentRuntimeOriginState::TrustedLocalUi,
        child_runtime_dispatch_state: ChildRuntimeDispatchState::NotRequired,
        child_acknowledgement_state: ChildAcknowledgementState::Required,
    });

    assert_eq!(
        decision.child_runtime_publish_state,
        ChildRuntimePublishState::DoNotPublish
    );
    assert_eq!(
        decision.parent_audit_retention_state,
        ParentAuditRetentionState::Retain
    );
    assert_eq!(
        decision.child_acknowledgement_wait_state,
        ChildAcknowledgementWaitState::DoNotAwait
    );
}

#[test]
fn untrusted_parent_origin_never_publishes_to_child_runtime() {
    let decision = route_parent_runtime_change(ParentRuntimeChangeRequest {
        target: ParentRuntimeTarget::ChildDevice,
        origin_state: ParentRuntimeOriginState::Untrusted,
        child_runtime_dispatch_state: ChildRuntimeDispatchState::Required,
        child_acknowledgement_state: ChildAcknowledgementState::Required,
    });

    assert_eq!(
        decision.child_runtime_publish_state,
        ChildRuntimePublishState::DoNotPublish
    );
    assert_eq!(
        decision.parent_audit_retention_state,
        ParentAuditRetentionState::Retain
    );
    assert_eq!(
        decision.child_acknowledgement_wait_state,
        ChildAcknowledgementWaitState::DoNotAwait
    );
}

#[test]
fn tracking_config_child_device_event_publishes_to_child_runtime_and_awaits_ack() {
    let event = parent_tracking_config_event(TrackingConfigUpdateTargetScope::ChildDevice);

    let decision =
        route_parent_tracking_config_update_event(&event, ChildAcknowledgementState::Required);

    assert_eq!(decision.target, ParentRuntimeTarget::ChildDevice);
    assert_eq!(
        decision.child_runtime_publish_state,
        ChildRuntimePublishState::Publish
    );
    assert_eq!(
        decision.child_acknowledgement_wait_state,
        ChildAcknowledgementWaitState::Await
    );
}

#[test]
fn tracking_config_family_event_publishes_as_household_scope_without_ack_wait() {
    let event = parent_tracking_config_event(TrackingConfigUpdateTargetScope::Family);

    let decision =
        route_parent_tracking_config_update_event(&event, ChildAcknowledgementState::NotRequired);

    assert_eq!(decision.target, ParentRuntimeTarget::Household);
    assert_eq!(
        decision.child_runtime_publish_state,
        ChildRuntimePublishState::Publish
    );
    assert_eq!(
        decision.child_acknowledgement_wait_state,
        ChildAcknowledgementWaitState::DoNotAwait
    );
}

#[test]
fn tracking_config_untrusted_origin_does_not_publish_to_child_runtime() {
    let event = parent_tracking_config_event(TrackingConfigUpdateTargetScope::ChildDevice);

    let decision = route_parent_tracking_config_update_event_from_origin(
        &event,
        ChildAcknowledgementState::Required,
        ParentRuntimeOriginState::Untrusted,
    );

    assert_eq!(decision.target, ParentRuntimeTarget::ChildDevice);
    assert_eq!(
        decision.child_runtime_publish_state,
        ChildRuntimePublishState::DoNotPublish
    );
    assert_eq!(
        decision.child_acknowledgement_wait_state,
        ChildAcknowledgementWaitState::DoNotAwait
    );
}

#[test]
fn tracking_config_child_profile_scope_maps_to_child_device_runtime_target() {
    assert_eq!(
        parent_runtime_target_from_tracking_scope(&TrackingConfigUpdateTargetScope::ChildProfile),
        ParentRuntimeTarget::ChildDevice
    );
    assert_eq!(
        parent_runtime_target_from_tracking_scope(&TrackingConfigUpdateTargetScope::DeviceGroup),
        ParentRuntimeTarget::Household
    );
}

#[test]
fn tracking_config_parent_runtime_records_typed_dispatch_event() {
    let event = parent_tracking_config_event(TrackingConfigUpdateTargetScope::ChildDevice);

    let dispatch = parent_runtime_tracking_dispatch_evaluated_event(
        &event,
        ChildAcknowledgementState::Required,
    );

    assert_eq!(dispatch.source_event, event);
    assert_eq!(
        dispatch.decision.child_runtime_publish_state,
        ChildRuntimePublishState::Publish
    );
    assert_eq!(
        dispatch.decision.child_acknowledgement_wait_state,
        ChildAcknowledgementWaitState::Await
    );
    assert_eq!(
        dispatch
            .contract()
            .expect("parent runtime dispatch contract")
            .event_type
            .as_str(),
        PARENT_RUNTIME_TRACKING_DISPATCH_EVALUATED_EVENT_TYPE
    );
}

fn parent_tracking_config_event(
    scope: TrackingConfigUpdateTargetScope,
) -> ParentTrackingConfigUpdatedEvent {
    let request = default_tracking_retention_settings_write_request();
    ParentTrackingConfigUpdatedEvent {
        source_command_id: request.command_id.clone(),
        source_message_id: TrackingSourceMessageId::parse(
            constants::tracking_retention_settings_write::COMMAND_ID,
        )
        .expect(constants::tracking_retention_settings_write::COMMAND_ID),
        source_peer_id: TrackingSourcePeerId::parse(constants::peer::PORTAL_DEV)
            .expect(constants::peer::PORTAL_DEV),
        target: TrackingConfigUpdateTarget {
            scope,
            device_id: TrackingTargetDeviceId::parse(constants::peer::LOCAL_DEV_AGENT)
                .expect(constants::peer::LOCAL_DEV_AGENT),
            platform: TrackingTargetPlatform::parse(
                ocentra_parent_agent_protocol::policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS,
            )
            .expect(
                ocentra_parent_agent_protocol::policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS,
            ),
            route: AgentRoute::Localhost,
        },
        config: request,
    }
}
