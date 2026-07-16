use super::super::*;

pub(super) fn normalize_results(
    input: &NetworkAiDetectionEvaluationInput,
) -> Result<Vec<NetworkAiDetectionResult>, NetworkAiDetectionEvaluationError> {
    let mut detection_refs = Vec::new();
    let mut results = Vec::new();
    for case in &input.cases {
        super::case_reject::reject_case_claims(case)?;
        let detection_ref = super::normalize_ref(&case.detection_ref)
            .ok_or(NetworkAiDetectionEvaluationError::EmptyDetectionRef)?;
        if detection_refs.contains(&detection_ref) {
            return Err(NetworkAiDetectionEvaluationError::DuplicateDetectionRef);
        }
        detection_refs.push(detection_ref.clone());
        results.push(normalize_case_result(
            case,
            detection_ref,
            input.maximum_average_drift_basis_points,
        )?);
    }
    Ok(results)
}

fn normalize_case_result(
    case: &NetworkAiDetectionFixtureCase,
    detection_ref: String,
    maximum_case_drift_basis_points: u16,
) -> Result<NetworkAiDetectionResult, NetworkAiDetectionEvaluationError> {
    let fixture_ref = super::normalize_ref(&case.fixture_ref)
        .ok_or(NetworkAiDetectionEvaluationError::EmptyFixtureRef)?;
    let summary_ref = super::normalize_ref(&case.summary_ref)
        .ok_or(NetworkAiDetectionEvaluationError::EmptySummaryRef)?;
    let evidence_refs = super::refs::normalized_evidence_refs(&case.evidence_refs)?;
    let analyzer_alert_refs =
        super::refs::normalized_analyzer_alert_refs(&case.analyzer_alert_refs)?;
    let input_kinds = super::kinds::normalized_input_kinds(&case.input_kinds)?;
    let label_match = case.expected_label == case.predicted_label;
    let expected_positive = super::is_positive_label(case.expected_label);
    let predicted_positive = super::is_positive_label(case.predicted_label);
    let confidence_drift_basis_points = case
        .confidence_basis_points
        .abs_diff(case.baseline_confidence_basis_points);

    Ok(NetworkAiDetectionResult {
        detection_ref,
        fixture_ref,
        summary_ref,
        evidence_refs,
        analyzer_alert_refs,
        expected_label: case.expected_label,
        predicted_label: case.predicted_label,
        confidence_basis_points: case.confidence_basis_points,
        baseline_confidence_basis_points: case.baseline_confidence_basis_points,
        confidence_drift_basis_points,
        risk_level: case.risk_level,
        input_kinds,
        label_match,
        expected_positive,
        predicted_positive,
        true_positive: expected_positive && label_match,
        false_positive: predicted_positive && !label_match,
        false_negative: expected_positive && !label_match,
        true_negative: !expected_positive && !predicted_positive && label_match,
        uncertainty_codes: super::uncertainty::uncertainty_codes(
            case,
            confidence_drift_basis_points,
            maximum_case_drift_basis_points,
        ),
        raw_pcap_available: false,
        exact_url_available: false,
        decrypted_payload_available: false,
        page_content_available: false,
        policy_authority: false,
        adapter_authority: false,
        enforcement_command_published: false,
    })
}
