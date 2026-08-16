use ocentra_parent_agent_protocol::constants;

use super::{
    TrackingAcknowledgementStateValue, TrackingAiPurposeKind, TrackingCheckInStateValue,
    TrackingNotificationChannelKind, TrackingUncertaintyKind,
};

impl TrackingAcknowledgementStateValue {
    pub(crate) fn as_contract_text(self) -> &'static str {
        match self {
            Self::Acknowledged => constants::tracking_runtime::ACKNOWLEDGEMENT_STATE_ACKNOWLEDGED,
        }
    }
}

impl TrackingCheckInStateValue {
    pub(crate) fn as_contract_text(self) -> &'static str {
        match self {
            Self::Received => constants::tracking_runtime::CHECK_IN_STATE_RECEIVED,
        }
    }
}

impl TrackingAiPurposeKind {
    pub(crate) fn as_contract_text(self) -> &'static str {
        match self {
            Self::NearbyPlaceClassification => {
                constants::tracking_runtime::ALLOWED_AI_PURPOSE_NEARBY_PLACE_CLASSIFICATION
            }
        }
    }
}

impl TrackingUncertaintyKind {
    pub(crate) fn as_contract_text(self) -> &'static str {
        match self {
            Self::NearbyPlaceClassificationRequired => {
                constants::tracking_runtime::UNCERTAINTY_CODE_NEARBY_PLACE_CLASSIFICATION_REQUIRED
            }
        }
    }
}

impl TrackingNotificationChannelKind {
    pub(crate) fn as_contract_text(self) -> &'static str {
        match self {
            Self::ParentPortal => constants::tracking_runtime::NOTIFICATION_CHANNEL_PARENT_PORTAL,
        }
    }
}
