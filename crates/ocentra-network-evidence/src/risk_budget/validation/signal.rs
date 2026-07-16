use super::super::*;

pub(super) fn validate_signal(
    signal: &NetworkRiskBudgetSignal,
) -> Result<(), NetworkRiskBudgetThresholdError> {
    if signal.signal_ref.trim().is_empty() {
        return Err(NetworkRiskBudgetThresholdError::EmptySignalRef);
    }
    validate_audit_report(&signal.audit_report)
}

pub(super) fn validate_audit_report(
    report: &NetworkAiAuditReport,
) -> Result<(), NetworkRiskBudgetThresholdError> {
    if report.audit_report_ref.trim().is_empty() {
        return Err(NetworkRiskBudgetThresholdError::EmptyAuditReportRef);
    }
    if report.cited_evidence_refs.is_empty() {
        return Err(NetworkRiskBudgetThresholdError::EmptyEvidenceRefs);
    }
    if report
        .cited_evidence_refs
        .iter()
        .any(|evidence_ref| evidence_ref.trim().is_empty())
    {
        return Err(NetworkRiskBudgetThresholdError::EmptyEvidenceRef);
    }
    if !report.parent_readable
        || !report.advisory_only
        || report.policy_authority
        || report.adapter_authority
        || report.enforcement_commands_published > 0
    {
        return Err(NetworkRiskBudgetThresholdError::AuditReportMustRemainAdvisory);
    }
    if report.raw_pcap_available
        || report.exact_url_available
        || report.decrypted_payload_available
        || report.page_content_available
        || report.private_message_available
        || report.search_query_available
        || report.remote_ai_used
    {
        return Err(NetworkRiskBudgetThresholdError::AuditReportUnsupportedClaim);
    }
    Ok(())
}
