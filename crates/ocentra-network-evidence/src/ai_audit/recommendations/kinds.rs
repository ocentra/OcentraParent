use super::*;

pub(super) fn recommendation_kinds(
    detections: &[NetworkAiDetectionResult],
    uncertainty_codes: &[NetworkAiAuditUncertaintyCode],
) -> Vec<NetworkAiAuditRecommendationKind> {
    let mut kinds = Vec::new();
    if has_high_risk_true_positive(detections) {
        kinds.push(NetworkAiAuditRecommendationKind::ReviewWithParent);
        kinds.push(NetworkAiAuditRecommendationKind::ReviewPolicyRule);
    }
    if !uncertainty_codes.is_empty() {
        kinds.push(NetworkAiAuditRecommendationKind::ConfirmWithManagedBrowser);
        kinds.push(NetworkAiAuditRecommendationKind::ConfirmWithScreenSummary);
    }
    if kinds.is_empty() {
        kinds.push(NetworkAiAuditRecommendationKind::MonitorOnly);
    }
    kinds
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
