mod drift;
mod evaluation_state;
mod precision;
mod recall;

use super::*;

pub(super) fn precision_state(
    precision_basis_points: Option<u16>,
    minimum_precision_basis_points: u16,
) -> NetworkAiDetectionPrecisionState {
    precision::precision_state(precision_basis_points, minimum_precision_basis_points)
}

pub(super) fn recall_state(
    recall_basis_points: Option<u16>,
    minimum_recall_basis_points: u16,
) -> NetworkAiDetectionRecallState {
    recall::recall_state(recall_basis_points, minimum_recall_basis_points)
}

pub(super) fn drift_state(
    average_confidence_drift_basis_points: u16,
    maximum_average_drift_basis_points: u16,
) -> NetworkAiDetectionDriftState {
    drift::drift_state(
        average_confidence_drift_basis_points,
        maximum_average_drift_basis_points,
    )
}

pub(super) fn evaluation_state(
    precision_state: NetworkAiDetectionPrecisionState,
    recall_state: NetworkAiDetectionRecallState,
    drift_state: NetworkAiDetectionDriftState,
) -> NetworkAiDetectionEvaluationState {
    evaluation_state::evaluation_state(precision_state, recall_state, drift_state)
}

pub(super) fn is_positive_label(label: NetworkAiDetectionLabel) -> bool {
    !matches!(
        label,
        NetworkAiDetectionLabel::BenignExpected | NetworkAiDetectionLabel::Unknown
    )
}

pub(super) fn normalize_ref(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_owned())
    }
}
