use super::NetworkParentNotificationCandidateError;

pub(super) fn normalized_evidence_refs(
    refs: &[String],
) -> Result<Vec<String>, NetworkParentNotificationCandidateError> {
    let mut normalized = Vec::new();
    for value in refs {
        let Some(ref_value) = normalize_ref(value) else {
            return Err(NetworkParentNotificationCandidateError::EmptyEvidenceRef);
        };
        if !normalized.contains(&ref_value) {
            normalized.push(ref_value);
        }
    }
    (!normalized.is_empty())
        .then_some(normalized)
        .ok_or(NetworkParentNotificationCandidateError::EmptyEvidenceRef)
}

pub(super) fn normalized_optional_ref(
    value: Option<&str>,
) -> Result<Option<String>, NetworkParentNotificationCandidateError> {
    match value {
        Some(raw) => normalize_ref(raw)
            .map(Some)
            .ok_or(NetworkParentNotificationCandidateError::EmptyLocalAiResultRef),
        None => Ok(None),
    }
}

pub(super) fn normalize_ref(value: &str) -> Option<String> {
    let trimmed = value.trim();
    (!trimmed.is_empty()).then(|| trimmed.to_owned())
}
