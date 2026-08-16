#[path = "recommendations/kinds.rs"]
mod kinds;
#[path = "recommendations/list.rs"]
mod list;
#[path = "recommendations/narrative.rs"]
mod narrative;
#[path = "recommendations/uncertainty.rs"]
mod uncertainty;

use super::*;

pub(super) fn audit_uncertainty_codes(
    detections: &[NetworkAiDetectionResult],
) -> Vec<NetworkAiAuditUncertaintyCode> {
    uncertainty::audit_uncertainty_codes(detections)
}

pub(super) fn recommendation_kinds(
    detections: &[NetworkAiDetectionResult],
    uncertainty_codes: &[NetworkAiAuditUncertaintyCode],
) -> Vec<NetworkAiAuditRecommendationKind> {
    kinds::recommendation_kinds(detections, uncertainty_codes)
}

pub(super) fn recommendations(
    audit_report_ref: &str,
    detection_refs: &[String],
    evidence_refs: &[String],
    parent_rule_refs: &[String],
    kinds: Vec<NetworkAiAuditRecommendationKind>,
) -> Vec<NetworkAiAuditRecommendation> {
    list::recommendations(
        audit_report_ref,
        detection_refs,
        evidence_refs,
        parent_rule_refs,
        kinds,
    )
}

pub(super) fn narrative_state(
    detections: &[NetworkAiDetectionResult],
    uncertainty_codes: &[NetworkAiAuditUncertaintyCode],
) -> NetworkAiAuditNarrativeState {
    narrative::narrative_state(detections, uncertainty_codes)
}

pub(super) fn narrative_headline(state: NetworkAiAuditNarrativeState) -> String {
    narrative::narrative_headline(state)
}
