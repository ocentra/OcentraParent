use super::*;

pub(super) fn cited_evidence_refs(
    detections: &[NetworkAiDetectionResult],
) -> Result<Vec<String>, NetworkAiAuditReportError> {
    let mut refs = Vec::new();
    for detection in detections {
        let detection_refs = normalized_refs(
            &detection.evidence_refs,
            NetworkAiAuditReportError::EmptyEvidenceRefs,
            NetworkAiAuditReportError::EmptyEvidenceRef,
        )?;
        extend_unique(&mut refs, detection_refs);
    }
    Ok(refs)
}

pub(super) fn cited_analyzer_alert_refs(
    detections: &[NetworkAiDetectionResult],
) -> Result<Vec<String>, NetworkAiAuditReportError> {
    let mut refs = Vec::new();
    for detection in detections {
        let detection_refs = normalized_refs(
            &detection.analyzer_alert_refs,
            NetworkAiAuditReportError::EmptyAnalyzerAlertRef,
            NetworkAiAuditReportError::EmptyAnalyzerAlertRef,
        )?;
        extend_unique(&mut refs, detection_refs);
    }
    Ok(refs)
}

pub(super) fn normalized_refs(
    values: &[String],
    empty_values_error: NetworkAiAuditReportError,
    empty_ref_error: NetworkAiAuditReportError,
) -> Result<Vec<String>, NetworkAiAuditReportError> {
    if values.is_empty() && empty_values_error != NetworkAiAuditReportError::EmptyAnalyzerAlertRef {
        return Err(empty_values_error);
    }
    let mut refs = Vec::new();
    for value in values {
        let Some(normalized) = normalize_ref(value) else {
            return Err(empty_ref_error);
        };
        if !refs.contains(&normalized) {
            refs.push(normalized);
        }
    }
    Ok(refs)
}

fn extend_unique(target: &mut Vec<String>, values: Vec<String>) {
    for value in values {
        if !target.contains(&value) {
            target.push(value);
        }
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
