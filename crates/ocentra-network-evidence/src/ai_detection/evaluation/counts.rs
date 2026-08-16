use super::super::*;
use super::DetectionCounts;

pub(super) fn count_detection_results(results: &[NetworkAiDetectionResult]) -> DetectionCounts {
    DetectionCounts {
        true_positive: results.iter().filter(|result| result.true_positive).count(),
        false_positive: results
            .iter()
            .filter(|result| result.false_positive)
            .count(),
        false_negative: results
            .iter()
            .filter(|result| result.false_negative)
            .count(),
        true_negative: results.iter().filter(|result| result.true_negative).count(),
        predicted_positive: results
            .iter()
            .filter(|result| result.predicted_positive)
            .count(),
        expected_positive: results
            .iter()
            .filter(|result| result.expected_positive)
            .count(),
    }
}
