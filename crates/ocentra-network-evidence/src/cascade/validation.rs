use super::{NetworkCascadeSource, NetworkCascadeSourceKind, NetworkEvidenceCascadeError};

pub(super) fn validate_cascade_input(
    sources: &[NetworkCascadeSource],
) -> Result<(), NetworkEvidenceCascadeError> {
    for source in sources {
        if source.source_ref.trim().is_empty() {
            return Err(NetworkEvidenceCascadeError::EmptySourceRef);
        }
        if source.decrypted_payload_available {
            return Err(NetworkEvidenceCascadeError::UnsupportedDecryptedPayloadClaim);
        }
        if source.exact_url_available
            && source.source_kind != NetworkCascadeSourceKind::ManagedBrowserExactUrl
        {
            return Err(
                NetworkEvidenceCascadeError::UnsupportedNetworkExactUrlClaim(source.source_kind),
            );
        }
    }
    Ok(())
}
