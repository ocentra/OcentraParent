use super::super::*;

pub(super) fn evaluation_state(
    precision_state: NetworkAiDetectionPrecisionState,
    recall_state: NetworkAiDetectionRecallState,
    drift_state: NetworkAiDetectionDriftState,
) -> NetworkAiDetectionEvaluationState {
    let quality_passed = precision_state == NetworkAiDetectionPrecisionState::MeetsThreshold
        && recall_state == NetworkAiDetectionRecallState::MeetsThreshold;
    match (quality_passed, drift_state) {
        (true, NetworkAiDetectionDriftState::WithinTolerance) => {
            NetworkAiDetectionEvaluationState::MeetsFixtureGate
        }
        (true, NetworkAiDetectionDriftState::ExceededTolerance) => {
            NetworkAiDetectionEvaluationState::DriftExceeded
        }
        (false, NetworkAiDetectionDriftState::WithinTolerance) => {
            NetworkAiDetectionEvaluationState::BelowQualityThreshold
        }
        (false, NetworkAiDetectionDriftState::ExceededTolerance) => {
            NetworkAiDetectionEvaluationState::BelowQualityAndDriftExceeded
        }
    }
}
