use super::protocol_lookup;
use crate::{constants, BrowserUnmanagedDetectionState};

impl BrowserUnmanagedDetectionState {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        protocol_lookup(
            value,
            [
                (
                    constants::browser::INTERVENTION_UNMANAGED_DETECTION_NONE,
                    Self::None,
                ),
                (
                    constants::browser::INTERVENTION_UNMANAGED_DETECTION_DETECTED,
                    Self::Detected,
                ),
                (
                    constants::browser::INTERVENTION_UNMANAGED_DETECTION_WARNED,
                    Self::Warned,
                ),
                (
                    constants::browser::INTERVENTION_UNMANAGED_DETECTION_TERMINATED,
                    Self::Terminated,
                ),
                (
                    constants::browser::INTERVENTION_UNMANAGED_DETECTION_MANUAL_REQUIRED,
                    Self::ManualRequired,
                ),
                (
                    constants::browser::INTERVENTION_UNMANAGED_DETECTION_UNAVAILABLE,
                    Self::Unavailable,
                ),
            ],
        )
    }
}
