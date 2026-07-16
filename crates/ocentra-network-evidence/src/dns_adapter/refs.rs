use super::{NetworkDnsAdapterProofError, NetworkDnsAdapterRequiredArtifact};

pub(super) fn normalized_refs(refs: &[String]) -> Result<Vec<String>, NetworkDnsAdapterProofError> {
    let mut normalized = Vec::new();
    for value in refs {
        let Some(ref_value) = normalize_ref(value) else {
            return Err(NetworkDnsAdapterProofError::EmptyEvidenceRef);
        };
        if !normalized.contains(&ref_value) {
            normalized.push(ref_value);
        }
    }
    if normalized.is_empty() {
        return Err(NetworkDnsAdapterProofError::EmptyEvidenceRef);
    }
    Ok(normalized)
}

pub(super) fn normalized_local_ai_ref(
    value: Option<&str>,
) -> Result<Option<String>, NetworkDnsAdapterProofError> {
    match value {
        Some(raw) => normalize_ref(raw)
            .map(Some)
            .ok_or(NetworkDnsAdapterProofError::EmptyLocalAiResultRef),
        None => Ok(None),
    }
}

pub(super) fn normalized_artifact_ref(
    value: Option<&str>,
    artifact: NetworkDnsAdapterRequiredArtifact,
) -> Result<Option<String>, NetworkDnsAdapterProofError> {
    match value {
        Some(raw) => normalize_ref(raw).map(Some).ok_or(
            NetworkDnsAdapterProofError::EmptyRequiredArtifactRef(artifact),
        ),
        None => Ok(None),
    }
}

pub(super) fn normalize_ref(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_owned())
    }
}
