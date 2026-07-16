use super::*;

pub(super) fn audit_uncertainty_codes(
    detections: &[NetworkAiDetectionResult],
) -> Vec<NetworkAiAuditUncertaintyCode> {
    let mut codes = Vec::new();
    for detection in detections {
        for code in &detection.uncertainty_codes {
            push_unique_code(&mut codes, map_uncertainty_code(code));
        }
    }
    codes
}

fn map_uncertainty_code(code: &NetworkAiDetectionUncertaintyCode) -> NetworkAiAuditUncertaintyCode {
    match code {
        NetworkAiDetectionUncertaintyCode::LabelMismatch => {
            NetworkAiAuditUncertaintyCode::DetectionMismatch
        }
        NetworkAiDetectionUncertaintyCode::FalsePositiveFixture => {
            NetworkAiAuditUncertaintyCode::FalsePositiveFixture
        }
        NetworkAiDetectionUncertaintyCode::FalseNegativeFixture => {
            NetworkAiAuditUncertaintyCode::FalseNegativeFixture
        }
        NetworkAiDetectionUncertaintyCode::UnknownPrediction => {
            NetworkAiAuditUncertaintyCode::UnknownPrediction
        }
        NetworkAiDetectionUncertaintyCode::ConfidenceDriftExceeded => {
            NetworkAiAuditUncertaintyCode::ConfidenceDriftExceeded
        }
        NetworkAiDetectionUncertaintyCode::LowConfidence => {
            NetworkAiAuditUncertaintyCode::LowConfidence
        }
    }
}

fn push_unique_code(
    codes: &mut Vec<NetworkAiAuditUncertaintyCode>,
    audit_code: NetworkAiAuditUncertaintyCode,
) {
    if !codes.contains(&audit_code) {
        codes.push(audit_code);
    }
}
