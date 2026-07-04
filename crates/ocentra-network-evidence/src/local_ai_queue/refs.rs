use super::NetworkLocalAiQueueError;

pub(super) fn normalized_summary_refs(
    summary_refs: &[String],
) -> Result<Vec<String>, NetworkLocalAiQueueError> {
    let mut refs = Vec::new();
    for summary_ref in summary_refs {
        let Some(normalized) = normalize_ref(summary_ref) else {
            return Err(NetworkLocalAiQueueError::EmptySummaryRef);
        };
        if !refs.contains(&normalized) {
            refs.push(normalized);
        }
    }
    Ok(refs)
}

pub(super) fn normalize_ref(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_owned())
    }
}
