use ocentra_child_runtime::tracking_runtime_flow::TrackingRuntimeEventFlow;
use ocentra_eventing::{
    bus::publisher::RootEventPublisher, bus::EventBus, envelope::EventMetadata,
    error::EventingError, request::RequestCompletionReport, request::RequestReport,
};
use ocentra_parent_agent_protocol::tracking::runtime_event::{
    TrackingChildCheckInRequestReceipt, TrackingChildCheckInRequestedEvent,
};

use crate::tracking_dispatch::{
    route_parent_runtime_change, ChildRuntimePublishState, ParentRuntimeChangeRequest,
    ParentRuntimeDispatchDecision,
};

#[derive(Debug)]
pub struct ParentTrackingChildCheckInRequestEventFlowReport {
    pub dispatch_request: ParentRuntimeChangeRequest,
    pub dispatch_decision: ParentRuntimeDispatchDecision,
    pub request_report: Option<RequestReport<TrackingChildCheckInRequestReceipt>>,
    pub child_runtime_request: Option<TrackingChildCheckInRequestedEvent>,
    pub child_runtime_request_metadata: Option<EventMetadata>,
    pub child_runtime_receipt: Option<TrackingChildCheckInRequestReceipt>,
    pub child_runtime_completion: Option<RequestCompletionReport>,
}

pub struct ParentTrackingChildCheckInRequestEventFlow {
    bus: RootEventPublisher,
    child_runtime_flow: TrackingRuntimeEventFlow,
    dispatch_request: ParentRuntimeChangeRequest,
}

impl ParentTrackingChildCheckInRequestEventFlow {
    pub async fn new(dispatch_request: ParentRuntimeChangeRequest) -> Result<Self, EventingError> {
        let bus = EventBus::root();
        let child_runtime_flow = TrackingRuntimeEventFlow::with_bus(bus.clone()).await?;
        Ok(Self {
            bus,
            child_runtime_flow,
            dispatch_request,
        })
    }

    pub async fn publish_parent_tracking_child_check_in_request(
        &self,
        event: &TrackingChildCheckInRequestedEvent,
    ) -> Result<ParentTrackingChildCheckInRequestEventFlowReport, EventingError> {
        let dispatch_decision = route_parent_runtime_change(self.dispatch_request);
        let request_report = dispatch_decision
            .publish_tracking_child_check_in_request(&self.bus, event.clone())
            .await?;
        let child_runtime_state =
            if dispatch_decision.child_runtime_publish_state == ChildRuntimePublishState::Publish {
                self.child_runtime_flow.latest_parent_requested_check_in()
            } else {
                None
            };

        Ok(ParentTrackingChildCheckInRequestEventFlowReport {
            dispatch_request: self.dispatch_request,
            dispatch_decision,
            request_report,
            child_runtime_request: child_runtime_state
                .as_ref()
                .map(|(request, _, _, _)| request.clone()),
            child_runtime_request_metadata: child_runtime_state
                .as_ref()
                .map(|(_, metadata, _, _)| metadata.clone()),
            child_runtime_receipt: child_runtime_state
                .as_ref()
                .map(|(_, _, receipt, _)| receipt.clone()),
            child_runtime_completion: child_runtime_state
                .as_ref()
                .map(|(_, _, _, completion)| completion.clone()),
        })
    }
}

pub async fn publish_parent_tracking_child_check_in_request_event_flow(
    dispatch_request: ParentRuntimeChangeRequest,
    event: &TrackingChildCheckInRequestedEvent,
) -> Result<ParentTrackingChildCheckInRequestEventFlowReport, EventingError> {
    ParentTrackingChildCheckInRequestEventFlow::new(dispatch_request)
        .await?
        .publish_parent_tracking_child_check_in_request(event)
        .await
}
