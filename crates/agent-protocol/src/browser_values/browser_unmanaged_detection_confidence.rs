use super::protocol_lookup;
use crate::{constants, BrowserUnmanagedDetectionConfidence};

impl BrowserUnmanagedDetectionConfidence {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        protocol_lookup(
            value,
            [
                (
                    constants::browser::UNMANAGED_DETECTION_CONFIDENCE_HIGH,
                    Self::High,
                ),
                (
                    constants::browser::UNMANAGED_DETECTION_CONFIDENCE_MEDIUM,
                    Self::Medium,
                ),
                (
                    constants::browser::UNMANAGED_DETECTION_CONFIDENCE_LOW,
                    Self::Low,
                ),
            ],
        )
    }
}
