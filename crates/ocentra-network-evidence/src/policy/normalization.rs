use super::NetworkEvidencePolicyMappingError;

pub(super) fn normalized_refs(
    refs: &[String],
    empty_error: NetworkEvidencePolicyMappingError,
) -> Result<Vec<String>, NetworkEvidencePolicyMappingError> {
    let mut normalized = Vec::new();
    for value in refs {
        let Some(ref_value) = normalize_ref(value) else {
            return Err(empty_error);
        };
        if !normalized.contains(&ref_value) {
            normalized.push(ref_value);
        }
    }
    (!normalized.is_empty())
        .then_some(normalized)
        .ok_or(empty_error)
}

pub(super) fn normalized_optional_ref(
    value: Option<&str>,
    empty_error: NetworkEvidencePolicyMappingError,
) -> Result<Option<String>, NetworkEvidencePolicyMappingError> {
    match value {
        Some(raw) => normalize_ref(raw).map(Some).ok_or(empty_error),
        None => Ok(None),
    }
}

pub(super) fn normalize_ref(value: &str) -> Option<String> {
    let trimmed = value.trim();
    (!trimmed.is_empty()).then(|| trimmed.to_owned())
}
