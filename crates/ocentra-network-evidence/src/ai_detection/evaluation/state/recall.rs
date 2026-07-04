use super::super::*;

pub(super) fn recall_state(
    recall_basis_points: Option<u16>,
    minimum_recall_basis_points: u16,
) -> NetworkAiDetectionRecallState {
    match recall_basis_points {
        None => NetworkAiDetectionRecallState::NoExpectedPositives,
        Some(recall) if recall >= minimum_recall_basis_points => {
            NetworkAiDetectionRecallState::MeetsThreshold
        }
        Some(_) => NetworkAiDetectionRecallState::BelowThreshold,
    }
}
