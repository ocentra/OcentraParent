use ocentra_parent_agent_protocol::constants;

use super::trusted_delivery_store::TrustedDeliveryStoreError;
use super::{EnforcementPayloadError, EnforcementText};

#[derive(Clone, Copy, Debug)]
pub(crate) enum TrustedDeliveryError {
    Payload,
    Missing,
    Replay,
    Mismatch,
    Store,
}

impl TrustedDeliveryError {
    pub(crate) fn protocol_reason(self) -> EnforcementText {
        EnforcementText(
            match self {
                Self::Payload | Self::Missing => {
                    constants::household_mesh::REJECTION_UNAUTHENTICATED_MESSAGE
                }
                Self::Replay => constants::household_mesh::REJECTION_REPLAYED_MESSAGE,
                Self::Mismatch => constants::enforcement::REJECTION_TARGET_MISMATCH,
                Self::Store => constants::enforcement::UNAVAILABLE_MANUAL_REQUIRED,
            }
            .to_string(),
        )
    }
}

impl From<EnforcementPayloadError> for TrustedDeliveryError {
    fn from(_: EnforcementPayloadError) -> Self {
        Self::Payload
    }
}

impl From<TrustedDeliveryStoreError> for TrustedDeliveryError {
    fn from(error: TrustedDeliveryStoreError) -> Self {
        match error {
            TrustedDeliveryStoreError::Missing => Self::Missing,
            TrustedDeliveryStoreError::Replay => Self::Replay,
            TrustedDeliveryStoreError::Other => Self::Store,
        }
    }
}

impl From<serde_json::Error> for TrustedDeliveryError {
    fn from(_: serde_json::Error) -> Self {
        Self::Store
    }
}
