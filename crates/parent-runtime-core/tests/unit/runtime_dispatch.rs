use ocentra_child_runtime::tracking_runtime_flow::TrackingRuntimeEventFlow;
use ocentra_eventing::{
    bus::EventBus, envelope::DomainEvent, error::EventingError, request::RequestCompletionOutcome,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::policy_constants;
use ocentra_parent_agent_protocol::tracking::{
    config_update_event::{
        default_tracking_config_update_request, parent_tracking_config_updated_event_from_command,
        ParentTrackingConfigUpdatedEvent, TrackingConfigUpdateTargetScope,
    },
    identifiers::{
        TrackingCheckInId, TrackingChildDeviceId, TrackingChildProfileId, TrackingEvidenceRef,
        TrackingPolicyViolationId, TrackingTimestamp,
    },
    runtime_event::{
        TrackingChildCheckInDeliveryState, TrackingChildCheckInRequestState,
        TrackingChildCheckInRequestedEvent,
    },
};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentMessageTarget, AgentPeer, AgentPeerRole,
    AgentRoute,
};
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;
use ocentra_parent_runtime_core::tracking_dispatch::{
    parent_runtime_target_from_tracking_scope, parent_runtime_tracking_dispatch_evaluated_event,
    route_parent_runtime_change, route_parent_tracking_config_update_event,
    route_parent_tracking_config_update_event_from_origin, ChildAcknowledgementState,
    ChildAcknowledgementWaitState, ChildRuntimeDispatchState, ChildRuntimePublishState,
    ParentAuditRetentionState, ParentRuntimeChangeRequest, ParentRuntimeOriginState,
    ParentRuntimeTarget, PARENT_RUNTIME_TRACKING_DISPATCH_EVALUATED_EVENT_TYPE,
};

macro_rules! result_or_panic {
    ($result:expr, $context:expr $(,)?) => {
        $result.expect($context)
    };
}

macro_rules! option_or_panic {
    ($option:expr, $context:expr $(,)?) => {
        $option.expect($context)
    };
}

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
        result_or_panic!(dispatch.contract(), "parent runtime dispatch contract")
            .event_type
            .as_str(),
        PARENT_RUNTIME_TRACKING_DISPATCH_EVALUATED_EVENT_TYPE
    );
}

#[tokio::test]
async fn tracking_child_check_in_request_publishes_trusted_intent_and_awaits_receipt() {
    let bus = EventBus::root();
    let runtime_flow = result_or_panic!(
        TrackingRuntimeEventFlow::with_bus(bus.clone()).await,
        constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED,
    );
    let decision = route_parent_runtime_change(ParentRuntimeChangeRequest {
        target: ParentRuntimeTarget::ChildDevice,
        origin_state: ParentRuntimeOriginState::TrustedLocalUi,
        child_runtime_dispatch_state: ChildRuntimeDispatchState::Required,
        child_acknowledgement_state: ChildAcknowledgementState::Required,
    });

    let report = decision
        .publish_tracking_child_check_in_request(&bus, parent_requested_check_in_event())
        .await;
    let report = option_or_panic!(
        result_or_panic!(
            report,
            constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED,
        ),
        "awaited check-in request should return a receipt",
    );
    let (request, metadata, receipt, completion) = option_or_panic!(
        runtime_flow.latest_parent_requested_check_in(),
        "child runtime should record the parent check-in request",
    );

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
        option_or_panic!(
            metadata.target_handler.as_ref(),
            "target handler should be preserved",
        )
        .as_str(),
        constants::tracking_runtime::TARGET_HANDLER_CHILD_TRACKING_CHECK_IN_REQUESTER
    );
    assert_eq!(completion.outcome, RequestCompletionOutcome::Completed);
}

#[tokio::test]
async fn tracking_child_check_in_request_can_publish_without_waiting_for_receipt() {
    let bus = EventBus::root();
    let runtime_flow = result_or_panic!(
        TrackingRuntimeEventFlow::with_bus(bus.clone()).await,
        constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED,
    );
    let decision = route_parent_runtime_change(ParentRuntimeChangeRequest {
        target: ParentRuntimeTarget::ChildDevice,
        origin_state: ParentRuntimeOriginState::TrustedLocalUi,
        child_runtime_dispatch_state: ChildRuntimeDispatchState::Required,
        child_acknowledgement_state: ChildAcknowledgementState::NotRequired,
    });

    let report = decision
        .publish_tracking_child_check_in_request(&bus, parent_requested_check_in_event())
        .await;
    let report = result_or_panic!(
        report,
        constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED,
    );
    let (_, _, receipt, completion) = option_or_panic!(
        runtime_flow.latest_parent_requested_check_in(),
        "child runtime should record the fire-and-forget check-in request",
    );

    assert!(report.is_none());
    assert_eq!(
        receipt.delivery_state,
        TrackingChildCheckInDeliveryState::Requested
    );
    assert_eq!(completion.outcome, RequestCompletionOutcome::Late);
}

#[tokio::test]
async fn tracking_child_check_in_request_rejects_duplicate_awaited_request_ids() {
    let bus = EventBus::root();
    let runtime_flow = result_or_panic!(
        TrackingRuntimeEventFlow::with_bus(bus.clone()).await,
        constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED,
    );
    let decision = route_parent_runtime_change(ParentRuntimeChangeRequest {
        target: ParentRuntimeTarget::ChildDevice,
        origin_state: ParentRuntimeOriginState::TrustedLocalUi,
        child_runtime_dispatch_state: ChildRuntimeDispatchState::Required,
        child_acknowledgement_state: ChildAcknowledgementState::Required,
    });
    let request = parent_requested_check_in_event();

    let first = decision
        .publish_tracking_child_check_in_request(&bus, request.clone())
        .await;
    let first = result_or_panic!(
        first,
        constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED,
    );
    let first = option_or_panic!(first, "first request should return a receipt");
    let duplicate = decision
        .publish_tracking_child_check_in_request(&bus, request)
        .await;

    assert_eq!(
        first.response.delivery_state,
        TrackingChildCheckInDeliveryState::Requested
    );
    assert!(matches!(
        duplicate,
        Err(EventingError::DuplicateRequest { ref request_id })
            if request_id.as_str() == constants::tracking_runtime::DEFAULT_CHILD_CHECK_IN_ID
    ));
    let (_, _, _, completion) = option_or_panic!(
        runtime_flow.latest_parent_requested_check_in(),
        "initial request should still be recorded",
    );
    assert_eq!(completion.outcome, RequestCompletionOutcome::Completed);
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
            platform: policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS.to_string(),
            route: AgentRoute::Localhost,
        },
        command: AgentCommandName::AgentActivityTrackingRetentionSettingsWrite,
        payload: LogFields::new(),
    }
}

fn parent_requested_check_in_event() -> TrackingChildCheckInRequestedEvent {
    TrackingChildCheckInRequestedEvent {
        child_device_id: result_or_panic!(
            TrackingChildDeviceId::parse(constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID),
            constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID,
        ),
        child_profile_id: result_or_panic!(
            TrackingChildProfileId::parse(constants::tracking_runtime::DEFAULT_CHILD_PROFILE_ID),
            constants::tracking_runtime::DEFAULT_CHILD_PROFILE_ID,
        ),
        check_in_id: result_or_panic!(
            TrackingCheckInId::parse(constants::tracking_runtime::DEFAULT_CHILD_CHECK_IN_ID),
            constants::tracking_runtime::DEFAULT_CHILD_CHECK_IN_ID,
        ),
        requested_at: result_or_panic!(
            TrackingTimestamp::parse(constants::tracking_runtime::DEFAULT_OBSERVED_AT),
            constants::tracking_runtime::DEFAULT_OBSERVED_AT,
        ),
        request_state: TrackingChildCheckInRequestState::Pending,
        delivery_state: TrackingChildCheckInDeliveryState::Queued,
        related_alert_id: result_or_panic!(
            TrackingPolicyViolationId::parse(
                constants::tracking_runtime::DEFAULT_POLICY_VIOLATION_ID,
            ),
            constants::tracking_runtime::DEFAULT_POLICY_VIOLATION_ID,
        ),
        include_location_if_permitted: true,
        expires_at: result_or_panic!(
            TrackingTimestamp::parse("2026-06-12T12:05:00Z"),
            "2026-06-12T12:05:00Z",
        ),
        evidence_refs: vec![result_or_panic!(
            TrackingEvidenceRef::parse(constants::tracking_runtime::DEFAULT_EVIDENCE_REF),
            constants::tracking_runtime::DEFAULT_EVIDENCE_REF,
        )],
        audit_refs: vec![String::from("audit.tracking.child-check-in.request")],
    }
}
