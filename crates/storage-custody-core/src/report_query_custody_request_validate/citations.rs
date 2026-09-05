use ocentra_schema::report_query_custody as contracts;

use super::super::ReportQueryCustodyDerivationError;

pub(super) fn validate(
    request: &contracts::ReportQueryCustodyRequest,
) -> Result<(), ReportQueryCustodyDerivationError> {
    if request.source_citation_refs.is_empty() || request.assistant_citation_refs.is_empty() {
        return Err(ReportQueryCustodyDerivationError::MissingCitationRefs);
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
