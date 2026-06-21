use ocentra_eventing::request::RequestCompletionOutcome;
use ocentra_parent_agent_protocol::{
    constants, TrackingChildCheckInDeliveryState, TrackingChildCheckInRequestState,
    TrackingChildCheckInRequestedEvent, TrackingEvidenceRef, TrackingPolicyViolationId,
    TrackingTimestamp,
};
use ocentra_parent_runtime_core::tracking_child_check_in_request_flow::publish_parent_tracking_child_check_in_request_event_flow;
use ocentra_parent_runtime_core::tracking_dispatch::{
    ChildAcknowledgementState, ChildRuntimeDispatchState, ChildRuntimePublishState,
    ParentRuntimeChangeRequest, ParentRuntimeOriginState, ParentRuntimeTarget,
};

#[tokio::test]
async fn parent_tracking_child_check_in_flow_returns_completed_receipt_when_awaited() {
    let report = publish_parent_tracking_child_check_in_request_event_flow(
        ParentRuntimeChangeRequest {
            target: ParentRuntimeTarget::ChildDevice,
            origin_state: ParentRuntimeOriginState::TrustedLocalUi,
            child_runtime_dispatch_state: ChildRuntimeDispatchState::Required,
            child_acknowledgement_state: ChildAcknowledgementState::Required,
        },
        &parent_requested_check_in_event(),
    )
    .await
    .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);

    assert_eq!(
        report.dispatch_decision.child_runtime_publish_state,
        ChildRuntimePublishState::Publish
    );
    assert_eq!(
        report
            .request_report
            .as_ref()
            .expect("awaited request should produce a report")
            .response
            .delivery_state,
        TrackingChildCheckInDeliveryState::Requested
    );
    assert_eq!(
        report
            .child_runtime_request
            .as_ref()
            .expect("child runtime should receive the request")
            .check_in_id
            .as_str(),
        constants::tracking_runtime::DEFAULT_CHILD_CHECK_IN_ID
    );
    assert_eq!(
        report
            .child_runtime_receipt
            .as_ref()
            .expect("child runtime receipt should be captured")
            .delivery_state,
        TrackingChildCheckInDeliveryState::Requested
    );
    assert_eq!(
        report
            .child_runtime_completion
            .as_ref()
            .expect("completion report should be captured")
            .outcome,
        RequestCompletionOutcome::Completed
    );
}

#[tokio::test]
async fn parent_tracking_child_check_in_flow_records_fire_and_forget_receipt() {
    let report = publish_parent_tracking_child_check_in_request_event_flow(
        ParentRuntimeChangeRequest {
            target: ParentRuntimeTarget::ChildDevice,
            origin_state: ParentRuntimeOriginState::TrustedLocalUi,
            child_runtime_dispatch_state: ChildRuntimeDispatchState::Required,
            child_acknowledgement_state: ChildAcknowledgementState::NotRequired,
        },
        &parent_requested_check_in_event(),
    )
    .await
    .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);

    assert!(report.request_report.is_none());
    assert_eq!(
        report
            .child_runtime_receipt
            .as_ref()
            .expect("child runtime receipt should be captured")
            .delivery_state,
        TrackingChildCheckInDeliveryState::Requested
    );
    assert_eq!(
        report
            .child_runtime_completion
            .as_ref()
            .expect("completion report should be captured")
            .outcome,
        RequestCompletionOutcome::Late
    );
}

#[tokio::test]
async fn parent_tracking_child_check_in_flow_skips_child_runtime_when_dispatch_is_blocked() {
    let report = publish_parent_tracking_child_check_in_request_event_flow(
        ParentRuntimeChangeRequest {
            target: ParentRuntimeTarget::ParentOnly,
            origin_state: ParentRuntimeOriginState::Untrusted,
            child_runtime_dispatch_state: ChildRuntimeDispatchState::Required,
            child_acknowledgement_state: ChildAcknowledgementState::Required,
        },
        &parent_requested_check_in_event(),
    )
    .await
    .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);

    assert_eq!(
        report.dispatch_decision.child_runtime_publish_state,
        ChildRuntimePublishState::DoNotPublish
    );
    assert!(report.request_report.is_none());
    assert!(report.child_runtime_request.is_none());
    assert!(report.child_runtime_receipt.is_none());
    assert!(report.child_runtime_completion.is_none());
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
