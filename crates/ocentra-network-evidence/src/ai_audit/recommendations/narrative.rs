use super::*;

pub(super) fn narrative_state(
    detections: &[NetworkAiDetectionResult],
    uncertainty_codes: &[NetworkAiAuditUncertaintyCode],
) -> NetworkAiAuditNarrativeState {
    if !uncertainty_codes.is_empty() {
        return NetworkAiAuditNarrativeState::UncertainReviewRequired;
    }
    if has_high_risk_true_positive(detections) {
        return NetworkAiAuditNarrativeState::Ready;
    }
    NetworkAiAuditNarrativeState::MonitorOnly
}

pub(super) fn narrative_headline(state: NetworkAiAuditNarrativeState) -> String {
    match state {
        NetworkAiAuditNarrativeState::Ready => {
            "Network AI audit recommends parent review for cited high-risk network detections."
        }
        NetworkAiAuditNarrativeState::UncertainReviewRequired => {
            "Network AI audit found uncertainty and recommends evidence confirmation before policy action."
        }
        NetworkAiAuditNarrativeState::MonitorOnly => {
            "Network AI audit recommends monitor-only handling for cited network detections."
        }
    }
    .to_owned()
}

fn has_high_risk_true_positive(detections: &[NetworkAiDetectionResult]) -> bool {
    detections.iter().any(|detection| {
        detection.true_positive
            && matches!(
                detection.risk_level,
                NetworkAiDetectionRiskLevel::High | NetworkAiDetectionRiskLevel::Critical
            )
    })
}
