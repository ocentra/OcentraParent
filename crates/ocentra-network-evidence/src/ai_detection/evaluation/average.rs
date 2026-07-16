use super::super::*;

pub(super) fn average_drift_basis_points(results: &[NetworkAiDetectionResult]) -> u16 {
    let total: u32 = results
        .iter()
        .map(|result| result.confidence_drift_basis_points as u32)
        .sum();
    ((total + results.len() as u32 / 2) / results.len() as u32) as u16
}
