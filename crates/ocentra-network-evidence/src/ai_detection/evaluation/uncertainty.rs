use super::super::*;

pub(super) fn uncertainty_codes(
    case: &NetworkAiDetectionFixtureCase,
    drift_basis_points: u16,
    maximum_drift_basis_points: u16,
) -> Vec<NetworkAiDetectionUncertaintyCode> {
    let mut codes = Vec::new();
    if case.expected_label != case.predicted_label {
        codes.push(NetworkAiDetectionUncertaintyCode::LabelMismatch);
    }
    if super::is_positive_label(case.predicted_label) && case.expected_label != case.predicted_label
    {
        codes.push(NetworkAiDetectionUncertaintyCode::FalsePositiveFixture);
    }
    if super::is_positive_label(case.expected_label) && case.expected_label != case.predicted_label
    {
        codes.push(NetworkAiDetectionUncertaintyCode::FalseNegativeFixture);
    }
    if case.predicted_label == NetworkAiDetectionLabel::Unknown {
        codes.push(NetworkAiDetectionUncertaintyCode::UnknownPrediction);
    }
    if drift_basis_points > maximum_drift_basis_points {
        codes.push(NetworkAiDetectionUncertaintyCode::ConfidenceDriftExceeded);
    }
    if case.confidence_basis_points < 5_000 {
        codes.push(NetworkAiDetectionUncertaintyCode::LowConfidence);
    }
    codes
}
