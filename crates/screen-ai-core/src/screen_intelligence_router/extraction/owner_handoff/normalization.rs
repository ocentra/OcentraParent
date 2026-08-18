use super::super::super::{
    MANAGED_BROWSER_SENSITIVITY_UNAVAILABLE, MANAGED_BROWSER_STRUCTURED_SIGNAL_UNAVAILABLE,
};
use super::super::ManagedBrowserStructuredExtractionObservation;

pub(super) fn normalize(
    mut observation: ManagedBrowserStructuredExtractionObservation,
) -> ManagedBrowserStructuredExtractionObservation {
    if !observation.unavailable {
        return observation;
    }
    observation.structured_signal_digest =
        String::from(MANAGED_BROWSER_STRUCTURED_SIGNAL_UNAVAILABLE);
    observation.structured_body_digest.clear();
    observation.structured_sensitivity_digest =
        String::from(MANAGED_BROWSER_SENSITIVITY_UNAVAILABLE);
    observation.document_frame_id = None;
    observation.document_loader_id = None;
    observation.document_url_digest = None;
    observation.dom_overflow_redacted = false;
    observation.private_content_redacted = false;
    observation.protected_content_skipped = false;
    observation.fresh = false;
    observation
}
