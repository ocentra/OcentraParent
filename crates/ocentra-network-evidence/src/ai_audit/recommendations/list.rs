use super::*;

pub(super) fn recommendations(
    audit_report_ref: &str,
    detection_refs: &[String],
    evidence_refs: &[String],
    parent_rule_refs: &[String],
    kinds: Vec<NetworkAiAuditRecommendationKind>,
) -> Vec<NetworkAiAuditRecommendation> {
    kinds
        .into_iter()
        .map(|kind| NetworkAiAuditRecommendation {
            recommendation_ref: format!("{audit_report_ref}:{kind:?}"),
            kind,
            cited_detection_refs: detection_refs.to_vec(),
            cited_evidence_refs: evidence_refs.to_vec(),
            cited_parent_rule_refs: parent_rule_refs.to_vec(),
            advisory_only: true,
            policy_authority: false,
            adapter_authority: false,
            enforcement_command_published: false,
        })
        .collect()
}
