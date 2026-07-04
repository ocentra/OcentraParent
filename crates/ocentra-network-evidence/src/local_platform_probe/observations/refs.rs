use super::{normalize_ref, NetworkLocalPlatformProbeError, NetworkPlatformClaimTarget};

pub(super) fn normalized_refs(
    target: NetworkPlatformClaimTarget,
    refs: &[String],
) -> Result<Vec<String>, NetworkLocalPlatformProbeError> {
    let mut normalized = Vec::new();
    for value in refs {
        let Some(ref_value) = normalize_ref(value) else {
            return Err(NetworkLocalPlatformProbeError::EmptyObservationEvidenceRef(
                target,
            ));
        };
        if !normalized.contains(&ref_value) {
            normalized.push(ref_value);
        }
    }
    (!normalized.is_empty()).then_some(normalized).ok_or(
        NetworkLocalPlatformProbeError::EmptyObservationEvidenceRef(target),
    )
}
