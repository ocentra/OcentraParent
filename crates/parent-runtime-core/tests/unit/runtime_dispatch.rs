use ocentra_child_runtime::tracking_runtime_flow::TrackingRuntimeEventFlow;
use ocentra_eventing::{
    bus::EventBus, envelope::DomainEvent, error::EventingError, request::RequestCompletionOutcome,
};
use ocentra_parent_agent_protocol::{
    constants, default_tracking_config_update_request,
    parent_tracking_config_updated_event_from_command, AgentCommandEnvelope, AgentCommandName,
    AgentMessageTarget, AgentPeer, AgentPeerRole, AgentRoute, LogFields,
    ParentTrackingConfigUpdatedEvent, TrackingChildCheckInDeliveryState,
    TrackingChildCheckInRequestState, TrackingChildCheckInRequestedEvent,
    TrackingConfigUpdateTargetScope, TrackingEvidenceRef, TrackingPolicyViolationId,
    TrackingTimestamp, AGENT_PROTOCOL_SCHEMA_VERSION,
};
use ocentra_parent_runtime_core::tracking_dispatch::{
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

#[tokio::test]
async fn tracking_child_check_in_request_publishes_trusted_intent_and_awaits_receipt() {
    let bus = EventBus::new();
    let runtime_flow = TrackingRuntimeEventFlow::with_bus(bus.clone())
        .await
        .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);
    let decision = route_parent_runtime_change(ParentRuntimeChangeRequest {
        target: ParentRuntimeTarget::ChildDevice,
        origin_state: ParentRuntimeOriginState::TrustedLocalUi,
        child_runtime_dispatch_state: ChildRuntimeDispatchState::Required,
        child_acknowledgement_state: ChildAcknowledgementState::Required,
    });

    let report = decision
        .publish_tracking_child_check_in_request(&bus, parent_requested_check_in_event())
        .await
        .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
        .expect("awaited check-in request should return a receipt");
    let (request, metadata, receipt, completion) = runtime_flow
        .latest_parent_requested_check_in()
        .expect("child runtime should record the parent check-in request");

    assert_eq!(
        report.response.delivery_state,
        TrackingChildCheckInDeliveryState::Requested
    );
    assert_eq!(report.response.reason_code, None);
    assert_eq!(receipt.check_in_id, request.check_in_id);
    assert_eq!(
        metadata.source.component.as_str(),
        constants::tracking_runtime::SOURCE_COMPONENT_PARENT_RUNTIME
    );
    assert_eq!(
        metadata
            .target_handler
            .as_ref()
            .expect("target handler should be preserved")
            .as_str(),
        constants::tracking_runtime::TARGET_HANDLER_CHILD_TRACKING_CHECK_IN_REQUESTER
    );
    assert_eq!(completion.outcome, RequestCompletionOutcome::Completed);
}

#[tokio::test]
async fn tracking_child_check_in_request_can_publish_without_waiting_for_receipt() {
    let bus = EventBus::new();
    let runtime_flow = TrackingRuntimeEventFlow::with_bus(bus.clone())
        .await
        .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);
    let decision = route_parent_runtime_change(ParentRuntimeChangeRequest {
        target: ParentRuntimeTarget::ChildDevice,
        origin_state: ParentRuntimeOriginState::TrustedLocalUi,
        child_runtime_dispatch_state: ChildRuntimeDispatchState::Required,
        child_acknowledgement_state: ChildAcknowledgementState::NotRequired,
    });

    let report = decision
        .publish_tracking_child_check_in_request(&bus, parent_requested_check_in_event())
        .await
        .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);
    let (_, _, receipt, completion) = runtime_flow
        .latest_parent_requested_check_in()
        .expect("child runtime should record the fire-and-forget check-in request");

    assert!(report.is_none());
    assert_eq!(
        receipt.delivery_state,
        TrackingChildCheckInDeliveryState::Requested
    );
    assert_eq!(completion.outcome, RequestCompletionOutcome::Late);
}

#[tokio::test]
async fn tracking_child_check_in_request_rejects_duplicate_awaited_request_ids() {
    let bus = EventBus::new();
    let runtime_flow = TrackingRuntimeEventFlow::with_bus(bus.clone())
        .await
        .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);
    let decision = route_parent_runtime_change(ParentRuntimeChangeRequest {
        target: ParentRuntimeTarget::ChildDevice,
        origin_state: ParentRuntimeOriginState::TrustedLocalUi,
        child_runtime_dispatch_state: ChildRuntimeDispatchState::Required,
        child_acknowledgement_state: ChildAcknowledgementState::Required,
    });
    let request = parent_requested_check_in_event();

    let first = decision
        .publish_tracking_child_check_in_request(&bus, request.clone())
        .await
        .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);
    let duplicate = decision
        .publish_tracking_child_check_in_request(&bus, request)
        .await
        .expect_err("duplicate request id should be rejected");

    assert!(first.is_some());
    assert!(matches!(
        duplicate,
        EventingError::DuplicateRequest { ref request_id }
            if request_id.as_str() == constants::tracking_runtime::DEFAULT_CHILD_CHECK_IN_ID
    ));
    assert_eq!(
        runtime_flow
            .latest_parent_requested_check_in()
            .expect("initial request should still be recorded")
            .3
            .outcome,
        RequestCompletionOutcome::Completed
    );
}

fn parent_tracking_config_event(
    scope: TrackingConfigUpdateTargetScope,
) -> ParentTrackingConfigUpdatedEvent {
    let request = default_tracking_config_update_request();
    let mut event =
        parent_tracking_config_updated_event_from_command(&tracking_config_command(), request);
    event.target.scope = scope;
    event
}

fn tracking_config_command() -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::tracking_retention_settings_write::COMMAND_ID.to_string(),
        sent_at: constants::tracking_retention_settings_write::ACCEPTED_AT.to_string(),
        source: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform:
                ocentra_parent_agent_protocol::policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS
                    .to_string(),
            route: AgentRoute::Localhost,
        },
        command: AgentCommandName::AgentActivityTrackingRetentionSettingsWrite,
        payload: LogFields::new(),
    }
}

fn parent_requested_check_in_event() -> TrackingChildCheckInRequestedEvent {
    TrackingChildCheckInRequestedEvent {
        child_device_id: ocentra_parent_agent_protocol::TrackingChildDeviceId::parse(
            constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID,
        )
        .expect(constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID),
        child_profile_id: ocentra_parent_agent_protocol::TrackingChildProfileId::parse(
            constants::tracking_runtime::DEFAULT_CHILD_PROFILE_ID,
        )
        .expect(constants::tracking_runtime::DEFAULT_CHILD_PROFILE_ID),
        check_in_id: ocentra_parent_agent_protocol::TrackingCheckInId::parse(
            constants::tracking_runtime::DEFAULT_CHILD_CHECK_IN_ID,
        )
        .expect(constants::tracking_runtime::DEFAULT_CHILD_CHECK_IN_ID),
        requested_at: TrackingTimestamp::parse(constants::tracking_runtime::DEFAULT_OBSERVED_AT)
            .expect(constants::tracking_runtime::DEFAULT_OBSERVED_AT),
        request_state: TrackingChildCheckInRequestState::Pending,
        delivery_state: TrackingChildCheckInDeliveryState::Queued,
        related_alert_id: TrackingPolicyViolationId::parse(
            constants::tracking_runtime::DEFAULT_POLICY_VIOLATION_ID,
        )
        .expect(constants::tracking_runtime::DEFAULT_POLICY_VIOLATION_ID),
        include_location_if_permitted: true,
        expires_at: TrackingTimestamp::parse("2026-06-12T12:05:00Z").expect("2026-06-12T12:05:00Z"),
        evidence_refs: vec![TrackingEvidenceRef::parse(
            constants::tracking_runtime::DEFAULT_EVIDENCE_REF,
        )
        .expect(constants::tracking_runtime::DEFAULT_EVIDENCE_REF)],
        audit_refs: vec![String::from("audit.tracking.child-check-in.request")],
    }
}
