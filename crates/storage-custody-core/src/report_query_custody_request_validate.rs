use ocentra_schema::report_query_custody as contracts;

use super::ReportQueryCustodyDerivationError;

pub(super) fn validate_report_query_custody_request(
    request: &contracts::ReportQueryCustodyRequest,
) -> Result<(), ReportQueryCustodyDerivationError> {
    let authority = &request.parent_authority;
    if authority.authority_generation == 0 {
        return Err(ReportQueryCustodyDerivationError::InvalidParentAuthority);
    }
    if authority.family_id != request.family.family_id
        || authority.parent_account_id != request.account.parent_account_id
        || authority.device_id != request.device.device_id
        || authority.child_profile_id != request.device.child_profile_id
    {
        return Err(ReportQueryCustodyDerivationError::ParentAuthorityIdentityMismatch);
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
    if request
        .source_citation_refs
        .iter()
        .chain(request.assistant_citation_refs.iter())
        .any(|citation| {
            citation.family_id != request.family.family_id
                || citation.child_profile_id != request.device.child_profile_id
        })
    {
        return Err(ReportQueryCustodyDerivationError::CitationIdentityMismatch);
    }
    if request
        .source_citation_refs
        .iter()
        .chain(request.assistant_citation_refs.iter())
        .any(|citation| {
            !request
                .requested_data_classes
                .contains(&citation.source_data_class)
                || !request
                    .allowed_source_data_classes
                    .contains(&citation.source_data_class)
        })
    {
        return Err(ReportQueryCustodyDerivationError::CitationSourceClassMismatch);
    }
    Ok(())
}
