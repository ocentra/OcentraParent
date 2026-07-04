use super::super::*;

pub(super) fn normalized_evidence_refs(
    values: &[String],
) -> Result<Vec<String>, NetworkAiDetectionEvaluationError> {
    if values.is_empty() {
        return Err(NetworkAiDetectionEvaluationError::EmptyEvidenceRefs);
    }
    let mut refs = Vec::new();
    for value in values {
        let Some(normalized) = super::normalize_ref(value) else {
            return Err(NetworkAiDetectionEvaluationError::EmptyEvidenceRef);
        };
        if !refs.contains(&normalized) {
            refs.push(normalized);
        }
    }
    Ok(refs)
}

pub(super) fn normalized_analyzer_alert_refs(
    values: &[String],
) -> Result<Vec<String>, NetworkAiDetectionEvaluationError> {
    let mut refs = Vec::new();
    for value in values {
        let Some(normalized) = super::normalize_ref(value) else {
            return Err(NetworkAiDetectionEvaluationError::EmptyAnalyzerAlertRef);
        };
        if !refs.contains(&normalized) {
            refs.push(normalized);
        }
    }
    Ok(refs)
}
