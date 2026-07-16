use ocentra_eventing::request::RequestCompletionOutcome;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::{
    identifiers::{
        TrackingCheckInId, TrackingChildDeviceId, TrackingChildProfileId, TrackingEvidenceRef,
        TrackingPolicyViolationId, TrackingTimestamp,
    },
    runtime_event::{
        TrackingChildCheckInDeliveryState, TrackingChildCheckInRequestState,
        TrackingChildCheckInRequestedEvent,
    },
};
use ocentra_parent_runtime_core::tracking_child_check_in_request_flow::publish_parent_tracking_child_check_in_request_event_flow;
use ocentra_parent_runtime_core::tracking_dispatch::{
    ChildAcknowledgementState, ChildRuntimeDispatchState, ChildRuntimePublishState,
    ParentRuntimeChangeRequest, ParentRuntimeOriginState, ParentRuntimeTarget,
};

macro_rules! result_or_unreachable {
    ($result:expr, $context:expr $(,)?) => {
        $result.expect($context)
    };
}

macro_rules! option_or_unreachable {
    ($option:expr, $context:expr $(,)?) => {
        $option.expect($context)
    };
}

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
    .await;
    let report = result_or_unreachable!(
        report,
        constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED,
    );

    assert_eq!(
        report.dispatch_decision.child_runtime_publish_state,
        ChildRuntimePublishState::Publish
    );
    assert_eq!(
        option_or_unreachable!(
            report.request_report.as_ref(),
            "awaited request should produce a report",
        )
        .response
        .delivery_state,
        TrackingChildCheckInDeliveryState::Requested
    );
    assert_eq!(
        option_or_unreachable!(
            report.child_runtime_request.as_ref(),
            "child runtime should receive the request",
        )
        .check_in_id
        .as_str(),
        constants::tracking_runtime::DEFAULT_CHILD_CHECK_IN_ID
    );
    assert_eq!(
        option_or_unreachable!(
            report.child_runtime_receipt.as_ref(),
            "child runtime receipt should be captured",
        )
        .delivery_state,
        TrackingChildCheckInDeliveryState::Requested
    );
    assert_eq!(
        option_or_unreachable!(
            report.child_runtime_completion.as_ref(),
            "completion report should be captured",
        )
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
    .await;
    let report = result_or_unreachable!(
        report,
        constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED,
    );

    assert!(report.request_report.is_none());
    assert_eq!(
        option_or_unreachable!(
            report.child_runtime_receipt.as_ref(),
            "child runtime receipt should be captured",
        )
        .delivery_state,
        TrackingChildCheckInDeliveryState::Requested
    );
    assert_eq!(
        option_or_unreachable!(
            report.child_runtime_completion.as_ref(),
            "completion report should be captured",
        )
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
    .await;
    let report = result_or_unreachable!(
        report,
        constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED,
    );

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
        child_device_id: result_or_unreachable!(
            TrackingChildDeviceId::parse(constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID),
            constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID,
        ),
        child_profile_id: result_or_unreachable!(
            TrackingChildProfileId::parse(constants::tracking_runtime::DEFAULT_CHILD_PROFILE_ID),
            constants::tracking_runtime::DEFAULT_CHILD_PROFILE_ID,
        ),
        check_in_id: result_or_unreachable!(
            TrackingCheckInId::parse(constants::tracking_runtime::DEFAULT_CHILD_CHECK_IN_ID),
            constants::tracking_runtime::DEFAULT_CHILD_CHECK_IN_ID,
        ),
        requested_at: result_or_unreachable!(
            TrackingTimestamp::parse(constants::tracking_runtime::DEFAULT_OBSERVED_AT),
            constants::tracking_runtime::DEFAULT_OBSERVED_AT,
        ),
        request_state: TrackingChildCheckInRequestState::Pending,
        delivery_state: TrackingChildCheckInDeliveryState::Queued,
        related_alert_id: result_or_unreachable!(
            TrackingPolicyViolationId::parse(
                constants::tracking_runtime::DEFAULT_POLICY_VIOLATION_ID,
            ),
            constants::tracking_runtime::DEFAULT_POLICY_VIOLATION_ID,
        ),
        include_location_if_permitted: true,
        expires_at: result_or_unreachable!(
            TrackingTimestamp::parse("2026-06-12T12:05:00Z"),
            "2026-06-12T12:05:00Z",
        ),
        evidence_refs: vec![result_or_unreachable!(
            TrackingEvidenceRef::parse(constants::tracking_runtime::DEFAULT_EVIDENCE_REF),
            constants::tracking_runtime::DEFAULT_EVIDENCE_REF,
        )],
        audit_refs: vec![String::from("audit.tracking.child-check-in.request")],
    }
}
