use super::super::*;

pub(super) fn precision_state(
    precision_basis_points: Option<u16>,
    minimum_precision_basis_points: u16,
) -> NetworkAiDetectionPrecisionState {
    match precision_basis_points {
        None => NetworkAiDetectionPrecisionState::NoPositivePredictions,
        Some(precision) if precision >= minimum_precision_basis_points => {
            NetworkAiDetectionPrecisionState::MeetsThreshold
        }
        Some(_) => NetworkAiDetectionPrecisionState::BelowThreshold,
    }
}
