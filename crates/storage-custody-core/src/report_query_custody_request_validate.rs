use ocentra_schema::report_query_custody as contracts;

use super::ReportQueryCustodyDerivationError;

pub(super) fn validate_report_query_custody_request(
    request: &contracts::ReportQueryCustodyRequest,
) -> Result<(), ReportQueryCustodyDerivationError> {
    if !request.parent_authorized {
        return Err(ReportQueryCustodyDerivationError::ParentAuthorizationRequired);
    }
    if !request.parent_owned_source_required {
        return Err(ReportQueryCustodyDerivationError::ParentOwnedSourceRequired);
    }
    if request.raw_child_evidence_requested {
        return Err(ReportQueryCustodyDerivationError::RawChildEvidenceRequested);
    }
    if request.page_size == 0 {
        return Err(ReportQueryCustodyDerivationError::NonPositivePageSize);
    }
    if request.requested_data_classes.is_empty() || request.allowed_source_data_classes.is_empty() {
        return Err(ReportQueryCustodyDerivationError::EmptyRequestScope);
    }
    if request.source_citation_refs.is_empty() || request.assistant_citation_refs.is_empty() {
        return Err(ReportQueryCustodyDerivationError::MissingCitationRefs);
    }
    if request.notification_payload_boundary
        != contracts::ReportQueryCustodyBoundary::ParentOwnedCitationsOnly
    {
        return Err(ReportQueryCustodyDerivationError::InvalidNotificationBoundary);
    }
    if request
        .requested_data_classes
        .iter()
        .any(|data_class| !request.allowed_source_data_classes.contains(data_class))
    {
        return Err(ReportQueryCustodyDerivationError::DisallowedSourceDataClass);
    }
    if request
        .source_citation_refs
        .iter()
        .chain(request.assistant_citation_refs.iter())
        .any(|citation| citation.kind != contracts::ParentEvidenceReferenceKind::QueryStoreSummary)
    {
        return Err(ReportQueryCustodyDerivationError::InvalidCitationKind);
    }
    Ok(())
}
