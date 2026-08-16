use super::normalize::normalize_ref;
use super::{NetworkCrossSliceEvidenceBundleError, NetworkCrossSliceEvidenceSource};
use crate::cascade::NetworkCascadeSourceKind;

pub(super) fn validate_bundle_sources(
    sources: &[NetworkCrossSliceEvidenceSource],
) -> Result<(), NetworkCrossSliceEvidenceBundleError> {
    for source in sources {
        if normalize_ref(&source.evidence_ref).is_none() {
            return Err(NetworkCrossSliceEvidenceBundleError::EmptyEvidenceRef);
        }
        if source.decrypted_payload_available {
            return Err(NetworkCrossSliceEvidenceBundleError::UnsupportedDecryptedPayloadClaim);
        }
        if source.exact_url_available
            && source.source_kind != NetworkCascadeSourceKind::ManagedBrowserExactUrl
        {
            return Err(
                NetworkCrossSliceEvidenceBundleError::UnsupportedNetworkExactUrlClaim(
                    source.source_kind,
                ),
            );
        }
        if source.policy_action_authority {
            return Err(NetworkCrossSliceEvidenceBundleError::UnsupportedPolicyAuthorityClaim);
        }
        if source.adapter_action_authority {
            return Err(NetworkCrossSliceEvidenceBundleError::UnsupportedAdapterAuthorityClaim);
        }
    }
    Ok(())
}
