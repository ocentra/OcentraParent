mod average;
mod case_reject;
mod counts;
mod global_reject;
mod kinds;
mod normalize;
mod ratio;
mod refs;
mod state;
mod uncertainty;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct DetectionCounts {
    pub(super) true_positive: usize,
    pub(super) false_positive: usize,
    pub(super) false_negative: usize,
    pub(super) true_negative: usize,
    pub(super) predicted_positive: usize,
    pub(super) expected_positive: usize,
}

pub(super) fn reject_global_claims(
    input: &NetworkAiDetectionEvaluationInput,
) -> Result<(), NetworkAiDetectionEvaluationError> {
    global_reject::reject_global_claims(input)
}

pub(super) fn normalize_results(
    input: &NetworkAiDetectionEvaluationInput,
) -> Result<Vec<NetworkAiDetectionResult>, NetworkAiDetectionEvaluationError> {
    normalize::normalize_results(input)
}

pub(super) fn count_detection_results(results: &[NetworkAiDetectionResult]) -> DetectionCounts {
    counts::count_detection_results(results)
}

pub(super) fn ratio_basis_points(numerator: usize, denominator: usize) -> Option<u16> {
    ratio::ratio_basis_points(numerator, denominator)
}

pub(super) fn average_drift_basis_points(results: &[NetworkAiDetectionResult]) -> u16 {
    average::average_drift_basis_points(results)
}

pub(super) fn precision_state(
    precision_basis_points: Option<u16>,
    minimum_precision_basis_points: u16,
) -> NetworkAiDetectionPrecisionState {
    state::precision_state(precision_basis_points, minimum_precision_basis_points)
}

pub(super) fn recall_state(
    recall_basis_points: Option<u16>,
    minimum_recall_basis_points: u16,
) -> NetworkAiDetectionRecallState {
    state::recall_state(recall_basis_points, minimum_recall_basis_points)
}

pub(super) fn drift_state(
    average_confidence_drift_basis_points: u16,
    maximum_average_drift_basis_points: u16,
) -> NetworkAiDetectionDriftState {
    state::drift_state(
        average_confidence_drift_basis_points,
        maximum_average_drift_basis_points,
    )
}

pub(super) fn evaluation_state(
    precision_state: NetworkAiDetectionPrecisionState,
    recall_state: NetworkAiDetectionRecallState,
    drift_state: NetworkAiDetectionDriftState,
) -> NetworkAiDetectionEvaluationState {
    state::evaluation_state(precision_state, recall_state, drift_state)
}

pub(super) fn normalize_ref(value: &str) -> Option<String> {
    state::normalize_ref(value)
}

pub(super) fn is_positive_label(label: NetworkAiDetectionLabel) -> bool {
    state::is_positive_label(label)
}
