use super::{NetworkActivityClassifierInput, NetworkClassifierError};

pub(super) fn validate_input(
    input: &NetworkActivityClassifierInput,
) -> Result<(), NetworkClassifierError> {
    if let Some(hint) = &input.cdn_hint {
        if hint.confidence_percent > 100 {
            return Err(NetworkClassifierError::InvalidCdnConfidence(
                hint.confidence_percent,
            ));
        }
        if hint.source_ref.trim().is_empty() {
            return Err(NetworkClassifierError::EmptyCdnSourceRef);
        }
    }
    if let Some(hint) = &input.process_hint {
        if hint.confidence_percent > 100 {
            return Err(NetworkClassifierError::InvalidProcessConfidence(
                hint.confidence_percent,
            ));
        }
        if hint.source_ref.trim().is_empty() {
            return Err(NetworkClassifierError::EmptyProcessSourceRef);
        }
    }
    if let Some(confirmation) = &input.browser_confirmation {
        if confirmation.source_ref.trim().is_empty() {
            return Err(NetworkClassifierError::EmptyBrowserConfirmationRef);
        }
    }

    Ok(())
}
