use super::*;

pub(super) fn build_network_ai_audit_report(
    input: &NetworkAiAuditReportInput,
) -> Result<NetworkAiAuditReport, NetworkAiAuditReportError> {
    claims::reject_global_claims(input)?;
    if input.detection_results.is_empty() {
        return Err(NetworkAiAuditReportError::EmptyDetectionResults);
    }

    let audit_report_ref = refs::normalize_ref(&input.audit_report_ref)
        .ok_or(NetworkAiAuditReportError::EmptyAuditReportRef)?;
    let narrative_template_ref = refs::normalize_ref(&input.narrative_template_ref)
        .ok_or(NetworkAiAuditReportError::EmptyNarrativeTemplateRef)?;
    let model_version_ref = refs::normalize_ref(&input.model_version_ref)
        .ok_or(NetworkAiAuditReportError::EmptyModelVersionRef)?;
    let policy_context_ref = refs::normalize_ref(&input.policy_context_ref)
        .ok_or(NetworkAiAuditReportError::EmptyPolicyContextRef)?;
    let parent_rule_refs = refs::normalized_refs(
        &input.parent_rule_refs,
        NetworkAiAuditReportError::EmptyParentRuleRefs,
        NetworkAiAuditReportError::EmptyParentRuleRef,
    )?;
    let detection_refs = claims::normalized_detection_refs(&input.detection_results)?;
    let evidence_refs = refs::cited_evidence_refs(&input.detection_results)?;
    let analyzer_alert_refs = refs::cited_analyzer_alert_refs(&input.detection_results)?;
    let uncertainty_codes = recommendations::audit_uncertainty_codes(&input.detection_results);
    let narrative_state =
        recommendations::narrative_state(&input.detection_results, &uncertainty_codes);

    Ok(NetworkAiAuditReport {
        audit_report_ref: audit_report_ref.clone(),
        narrative_template_ref,
        model_version_ref,
        policy_context_ref,
        narrative_state,
        narrative_headline: recommendations::narrative_headline(narrative_state),
        cited_detection_refs: detection_refs.clone(),
        cited_evidence_refs: evidence_refs.clone(),
        cited_analyzer_alert_refs: analyzer_alert_refs,
        cited_parent_rule_refs: parent_rule_refs.clone(),
        recommendations: recommendations::recommendations(
            &audit_report_ref,
            &detection_refs,
            &evidence_refs,
            &parent_rule_refs,
            recommendations::recommendation_kinds(&input.detection_results, &uncertainty_codes),
        ),
        uncertainty_codes,
        parent_readable: true,
        advisory_only: true,
        raw_pcap_available: false,
        exact_url_available: false,
        decrypted_payload_available: false,
        page_content_available: false,
        private_message_available: false,
        search_query_available: false,
        remote_ai_used: false,
        policy_authority: false,
        adapter_authority: false,
        enforcement_commands_published: 0,
    })
}
