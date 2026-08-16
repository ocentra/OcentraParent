use super::super::*;

pub(super) fn drift_state(
    average_confidence_drift_basis_points: u16,
    maximum_average_drift_basis_points: u16,
) -> NetworkAiDetectionDriftState {
    if average_confidence_drift_basis_points > maximum_average_drift_basis_points {
        NetworkAiDetectionDriftState::ExceededTolerance
    } else {
        NetworkAiDetectionDriftState::WithinTolerance
    }
}
