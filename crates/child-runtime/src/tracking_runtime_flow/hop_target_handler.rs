use super::metadata::TrackingRuntimeHop;
use ocentra_parent_agent_protocol::constants;

impl TrackingRuntimeHop {
    pub(super) fn target_handler(self) -> &'static str {
        match self {
            Self::LocationObserved
            | Self::EvidenceRecorded
            | Self::GeofenceTransitionDetected
            | Self::ChildCheckInRecorded => {
                constants::tracking_runtime::TARGET_HANDLER_CHILD_TRACKING_OBSERVER
            }
            Self::ExpectedPlaceStateEvaluated => {
                constants::tracking_runtime::TARGET_HANDLER_CHILD_POLICY_EXPECTED_PLACE_EVALUATOR
            }
            Self::AiAnalysisRequested => {
                constants::tracking_runtime::TARGET_HANDLER_CHILD_AI_TRACKING_ANALYZER
            }
            Self::NearbyPlaceClassified => {
                constants::tracking_runtime::TARGET_HANDLER_CHILD_POLICY_TRACKING_ANALYZER
            }
            Self::PolicyViolationDetected | Self::ParentNotificationRequested => {
                constants::tracking_runtime::TARGET_HANDLER_CHILD_NOTIFICATION_POLICY_BRIDGE
            }
        }
    }
}
