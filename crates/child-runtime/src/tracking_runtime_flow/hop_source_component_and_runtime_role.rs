use super::metadata::TrackingRuntimeHop;
use ocentra_parent_agent_protocol::constants;

impl TrackingRuntimeHop {
    pub(super) fn source_component(self) -> &'static str {
        match self {
            Self::LocationObserved
            | Self::EvidenceRecorded
            | Self::GeofenceTransitionDetected
            | Self::ExpectedPlaceStateEvaluated
            | Self::ChildCheckInRecorded
            | Self::AiAnalysisRequested => {
                constants::tracking_runtime::SOURCE_COMPONENT_CHILD_TRACKING_RUNTIME
            }
            Self::NearbyPlaceClassified => {
                constants::tracking_runtime::SOURCE_COMPONENT_CHILD_AI_RUNTIME
            }
            Self::PolicyViolationDetected => {
                constants::tracking_runtime::SOURCE_COMPONENT_CHILD_POLICY_RUNTIME
            }
            Self::ParentNotificationRequested => {
                constants::tracking_runtime::SOURCE_COMPONENT_CHILD_NOTIFICATION_RUNTIME
            }
        }
    }

    pub(super) fn runtime_role(self) -> &'static str {
        match self {
            Self::LocationObserved
            | Self::EvidenceRecorded
            | Self::GeofenceTransitionDetected
            | Self::ExpectedPlaceStateEvaluated
            | Self::ChildCheckInRecorded
            | Self::AiAnalysisRequested => constants::eventing_source::ROLE_AGENT,
            Self::NearbyPlaceClassified => constants::eventing_source::ROLE_ANALYZER,
            Self::PolicyViolationDetected => constants::eventing_source::ROLE_DECISION_ENGINE,
            Self::ParentNotificationRequested => {
                constants::eventing_source::ROLE_SIDE_EFFECT_ADAPTER
            }
        }
    }
}
