use ocentra_schema::report_query_custody as contracts;

use super::super::ReportQueryCustodyDerivationError;

pub(super) fn validate(
    request: &contracts::ReportQueryCustodyRequest,
) -> Result<(), ReportQueryCustodyDerivationError> {
    if request.raw_child_evidence_requested {
        return Err(ReportQueryCustodyDerivationError::RawChildEvidenceRequested);
    }
    super::report_query_custody_page_size_validate::validate_report_query_custody_page_size(
        request.page_size,
    )?;
    if request.requested_data_classes.is_empty() || request.allowed_source_data_classes.is_empty() {
        return Err(ReportQueryCustodyDerivationError::EmptyRequestScope);
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
    Ok(())
}
