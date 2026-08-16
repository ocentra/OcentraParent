use super::*;

pub(super) fn normalized_refs(refs: &[String]) -> Result<Vec<String>, NetworkActionResultError> {
    let mut normalized = Vec::new();
    for value in refs {
        let Some(ref_value) = normalize_ref(value) else {
            return Err(NetworkActionResultError::EmptyEvidenceRef);
        };
        if !normalized.contains(&ref_value) {
            normalized.push(ref_value);
        }
    }
    if normalized.is_empty() {
        return Err(NetworkActionResultError::EmptyEvidenceRef);
    }
    Ok(normalized)
}

pub(super) fn normalized_optional_ref(
    value: Option<&str>,
    empty_error: NetworkActionResultError,
) -> Result<Option<String>, NetworkActionResultError> {
    match value {
        Some(raw) => normalize_ref(raw).map(Some).ok_or(empty_error),
        None => Ok(None),
    }
}

pub(super) fn normalized_artifact_ref(
    value: Option<&str>,
    artifact: NetworkActionResultRequiredArtifact,
) -> Result<Option<String>, NetworkActionResultError> {
    match value {
        Some(raw) => normalize_ref(raw)
            .map(Some)
            .ok_or(NetworkActionResultError::EmptyRequiredArtifactRef(artifact)),
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
