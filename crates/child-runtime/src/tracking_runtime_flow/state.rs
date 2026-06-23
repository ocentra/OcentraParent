use std::{
    collections::BTreeSet,
    sync::{Arc, Mutex},
};

use ocentra_eventing::{envelope::EventMetadata, request::RequestCompletionReport};
use ocentra_parent_agent_protocol::tracking::identifiers::TrackingCheckInId;
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

#[derive(Clone, Debug, Default)]
pub(super) struct TrackingRuntimeEventState {
    pub(super) location_observed: Arc<Mutex<Option<TrackingLocationObservedEvent>>>,
    pub(super) evidence_recorded: Arc<Mutex<Option<TrackingEvidenceRecordedEvent>>>,
    pub(super) geofence_transition_detected:
        Arc<Mutex<Option<TrackingGeofenceTransitionDetectedEvent>>>,
    pub(super) expected_place_state_evaluated:
        Arc<Mutex<Option<TrackingExpectedPlaceStateEvaluatedEvent>>>,
    pub(super) child_check_in_recorded: Arc<Mutex<Option<TrackingChildCheckInRecordedEvent>>>,
    pub(super) parent_requested_check_in: Arc<Mutex<Option<TrackingChildCheckInRequestedEvent>>>,
    pub(super) parent_requested_check_in_metadata: Arc<Mutex<Option<EventMetadata>>>,
    pub(super) parent_requested_check_in_receipt:
        Arc<Mutex<Option<TrackingChildCheckInRequestReceipt>>>,
    pub(super) parent_requested_check_in_completion: Arc<Mutex<Option<RequestCompletionReport>>>,
    pub(super) ai_analysis_requested: Arc<Mutex<Option<TrackingAiAnalysisRequestedEvent>>>,
    pub(super) nearby_place_classified: Arc<Mutex<Option<TrackingNearbyPlaceClassifiedEvent>>>,
    pub(super) ai_boundary_decision: Arc<Mutex<Option<TrackingAiBoundaryDecision>>>,
    pub(super) alert_decision: Arc<Mutex<Option<TrackingAlertDecision>>>,
    pub(super) policy_violation_detected: Arc<Mutex<Option<TrackingPolicyViolationDetectedEvent>>>,
    pub(super) parent_notification_requested: Arc<Mutex<Option<ParentNotificationRequestedEvent>>>,
    pub(super) policy_violation_history: Arc<Mutex<Vec<TrackingPolicyViolationDetectedEvent>>>,
    pub(super) seen_parent_requested_check_in_ids: Arc<Mutex<BTreeSet<TrackingCheckInId>>>,
}
