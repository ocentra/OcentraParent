use ocentra_parent_agent_protocol::constants;

use super::{TrackingLocationRelationKind, TrackingRuntimeRef, TrackingTimestampKind};

impl TrackingRuntimeRef {
    pub(crate) fn as_contract_text(self) -> &'static str {
        match self {
            Self::ChildDevice => constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID,
            Self::ChildProfile => constants::tracking_runtime::DEFAULT_CHILD_PROFILE_ID,
            Self::Observation => constants::tracking_runtime::DEFAULT_OBSERVATION_ID,
            Self::ExpectedPlace => constants::tracking_runtime::DEFAULT_EXPECTED_PLACE_REF,
        }
    }
}

impl TrackingLocationRelationKind {
    pub(crate) fn as_contract_text(self) -> &'static str {
        match self {
            Self::UncertainNear => {
                constants::tracking_runtime::LOCATION_RELATION_UNCERTAIN_NEAR_EXPECTED_PLACE
            }
            Self::At => constants::tracking_runtime::LOCATION_RELATION_AT_EXPECTED_PLACE,
            Self::Away => constants::tracking_runtime::LOCATION_RELATION_AWAY_FROM_EXPECTED_PLACE,
        }
    }
}

impl TrackingTimestampKind {
    pub(crate) fn as_contract_text(self) -> &'static str {
        match self {
            Self::DefaultObservedAt => constants::tracking_runtime::DEFAULT_OBSERVED_AT,
        }
    }
}
