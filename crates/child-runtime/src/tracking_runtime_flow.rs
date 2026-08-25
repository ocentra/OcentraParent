use metadata::{tracking_runtime_metadata, TrackingRuntimeHop};
use ocentra_eventing::bus::reports::handler::EventMetricsSnapshot;
use ocentra_eventing::{
    bus::publisher::RootEventPublisher, bus::subscriber::SubscriptionReport, bus::EventBus,
    envelope::EventMetadata, error::EventingError, request::RequestCompletionReport,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::runtime_event::{
    ParentNotificationRequestedEvent, TrackingAiAnalysisRequestedEvent,
    TrackingChildCheckInRecordedEvent, TrackingChildCheckInRequestReceipt,
    TrackingChildCheckInRequestedEvent, TrackingEvidenceRecordedEvent,
    TrackingExpectedPlaceStateEvaluatedEvent, TrackingGeofenceTransitionDetectedEvent,
    TrackingLocationObservedEvent, TrackingNearbyPlaceClassifiedEvent,
    TrackingPolicyViolationDetectedEvent,
};
use ocentra_tracking_core::ai_boundary::TrackingAiBoundaryDecision;
use ocentra_tracking_core::alerting::TrackingAlertDecision;
use state::TrackingRuntimeEventState;
use subscriptions::{
    subscribe_child_ai_tracking_analysis_events, subscribe_child_notification_policy_events,
    subscribe_child_policy_tracking_analysis_events,
    subscribe_child_policy_tracking_expected_place_events,
    subscribe_child_tracking_check_in_request_events, subscribe_tracking_location_observed_events,
};

mod check_in_requests;
mod hop_source_component_and_runtime_role;
mod hop_target_handler;
mod metadata;
mod state;
mod state_analysis;
mod state_check_in;
mod state_observation;
mod subscriptions;

#[derive(Clone, Debug, PartialEq)]
pub struct TrackingRuntimeEventFlowReport {
    pub tracking_subscription_report: SubscriptionReport,
    pub child_check_in_request_subscription_report: SubscriptionReport,
    pub child_ai_subscription_report: SubscriptionReport,
    pub child_policy_subscription_report: SubscriptionReport,
    pub child_expected_place_policy_subscription_report: SubscriptionReport,
    pub child_notification_subscription_report: SubscriptionReport,
    pub evidence_recorded: TrackingEvidenceRecordedEvent,
    pub geofence_transition_detected: Option<TrackingGeofenceTransitionDetectedEvent>,
    pub expected_place_state_evaluated: Option<TrackingExpectedPlaceStateEvaluatedEvent>,
    pub child_check_in_recorded: Option<TrackingChildCheckInRecordedEvent>,
    pub ai_analysis_requested: Option<TrackingAiAnalysisRequestedEvent>,
    pub nearby_place_classified: Option<TrackingNearbyPlaceClassifiedEvent>,
    pub ai_boundary_decision: Option<TrackingAiBoundaryDecision>,
    pub alert_decision: Option<TrackingAlertDecision>,
    pub policy_violation_detected: Option<TrackingPolicyViolationDetectedEvent>,
    pub parent_notification_requested: Option<ParentNotificationRequestedEvent>,
    pub parent_requested_check_in: Option<TrackingChildCheckInRequestedEvent>,
    pub parent_requested_check_in_receipt: Option<TrackingChildCheckInRequestReceipt>,
    pub parent_requested_check_in_completion: Option<RequestCompletionReport>,
}

pub struct TrackingRuntimeEventFlow {
    bus: RootEventPublisher,
    state: TrackingRuntimeEventState,
    tracking_subscription_report: SubscriptionReport,
    child_check_in_request_subscription_report: SubscriptionReport,
    child_ai_subscription_report: SubscriptionReport,
    child_policy_subscription_report: SubscriptionReport,
    child_expected_place_policy_subscription_report: SubscriptionReport,
    child_notification_subscription_report: SubscriptionReport,
}

impl TrackingRuntimeEventFlow {
    pub async fn new() -> Result<Self, EventingError> {
        Self::with_bus(EventBus::new()).await
    }

    pub async fn with_bus(bus: RootEventPublisher) -> Result<Self, EventingError> {
        let state = TrackingRuntimeEventState::default();
        let tracking_subscription_report =
            subscribe_tracking_location_observed_events(&bus, state.clone()).await?;
        let child_check_in_request_subscription_report =
            subscribe_child_tracking_check_in_request_events(&bus, state.clone()).await?;
        let child_ai_subscription_report =
            subscribe_child_ai_tracking_analysis_events(&bus, state.clone()).await?;
        let child_expected_place_policy_subscription_report =
            subscribe_child_policy_tracking_expected_place_events(&bus, state.clone()).await?;
        let child_policy_subscription_report =
            subscribe_child_policy_tracking_analysis_events(&bus, state.clone()).await?;
        let child_notification_subscription_report =
            subscribe_child_notification_policy_events(&bus, state.clone()).await?;

        Ok(Self {
            bus,
            state,
            tracking_subscription_report,
            child_check_in_request_subscription_report,
            child_ai_subscription_report,
            child_policy_subscription_report,
            child_expected_place_policy_subscription_report,
            child_notification_subscription_report,
        })
    }

    pub async fn publish_location_observed(
        &self,
        event: TrackingLocationObservedEvent,
    ) -> Result<TrackingRuntimeEventFlowReport, EventingError> {
        let validation =
            ocentra_tracking_core::location_validation::validate_tracking_location_observation(
                &event,
            );
        if validation.result_state
            == ocentra_tracking_core::location_validation::TrackingLocationValidationResultState::Rejected
        {
            return Err(EventingError::InvalidValue {
                field: constants::tracking_runtime::FIELD_LOCATION_VALIDATION,
                value: validation.validation_state.to_string(),
            });
        }

        self.state.reset_for_new_observation();
        let correlation_suffix = event.observation_id.as_str().to_owned();
        let recorded_at = event.observed_at.clone();
        self.bus
            .publish(
                event,
                tracking_runtime_metadata(
                    TrackingRuntimeHop::LocationObserved,
                    &correlation_suffix,
                    &recorded_at,
                )?,
            )
            .await?;

        self.report()
    }

    pub async fn metrics_snapshot(&self) -> EventMetricsSnapshot {
        self.bus.metrics_snapshot().await
    }

    pub fn latest_parent_requested_check_in(
        &self,
    ) -> Option<(
        TrackingChildCheckInRequestedEvent,
        EventMetadata,
        TrackingChildCheckInRequestReceipt,
        RequestCompletionReport,
    )> {
        Some((
            self.state.parent_requested_check_in()?,
            self.state.parent_requested_check_in_metadata()?,
            self.state.parent_requested_check_in_receipt()?,
            self.state.parent_requested_check_in_completion()?,
        ))
    }

    fn report(&self) -> Result<TrackingRuntimeEventFlowReport, EventingError> {
        Ok(TrackingRuntimeEventFlowReport {
            tracking_subscription_report: self.tracking_subscription_report.clone(),
            child_check_in_request_subscription_report: self
                .child_check_in_request_subscription_report
                .clone(),
            child_ai_subscription_report: self.child_ai_subscription_report.clone(),
            child_policy_subscription_report: self.child_policy_subscription_report.clone(),
            child_expected_place_policy_subscription_report: self
                .child_expected_place_policy_subscription_report
                .clone(),
            child_notification_subscription_report: self
                .child_notification_subscription_report
                .clone(),
            evidence_recorded: self.state.evidence_recorded()?,
            geofence_transition_detected: self.state.geofence_transition_detected(),
            expected_place_state_evaluated: self.state.expected_place_state_evaluated(),
            child_check_in_recorded: self.state.child_check_in_recorded(),
            ai_analysis_requested: self.state.ai_analysis_requested(),
            nearby_place_classified: self.state.nearby_place_classified(),
            ai_boundary_decision: self.state.ai_boundary_decision(),
            alert_decision: self.state.alert_decision(),
            policy_violation_detected: self.state.policy_violation_detected(),
            parent_notification_requested: self.state.parent_notification_requested(),
            parent_requested_check_in: self.state.parent_requested_check_in(),
            parent_requested_check_in_receipt: self.state.parent_requested_check_in_receipt(),
            parent_requested_check_in_completion: self.state.parent_requested_check_in_completion(),
        })
    }
}

pub async fn publish_child_tracking_location_observed_event(
    event: TrackingLocationObservedEvent,
) -> Result<TrackingRuntimeEventFlowReport, EventingError> {
    TrackingRuntimeEventFlow::new()
        .await?
        .publish_location_observed(event)
        .await
}
