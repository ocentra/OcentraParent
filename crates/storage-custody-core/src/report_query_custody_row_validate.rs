use ocentra_schema::report_query_custody as contracts;

use super::{ReportQueryCustodyDerivationError, ReportQueryCustodyDerivationInput};

#[path = "report_query_custody_request_validate.rs"]
mod report_query_custody_request_validate;

pub(super) fn validate_report_query_custody_input(
    request: &contracts::ReportQueryCustodyRequest,
    input: &ReportQueryCustodyDerivationInput,
) -> Result<(), ReportQueryCustodyDerivationError> {
    report_query_custody_request_validate::validate_report_query_custody_request(request)?;
    if input.raw_child_evidence_included {
        return Err(ReportQueryCustodyDerivationError::RawChildEvidenceRequested);
    }
    if input.page_index == 0 {
        return Err(ReportQueryCustodyDerivationError::NonPositivePageIndex);
    }
    if !request
        .allowed_source_data_classes
        .contains(&input.source_data_class)
        || !request
            .requested_data_classes
            .contains(&input.source_data_class)
    {
        return Err(ReportQueryCustodyDerivationError::DisallowedSourceDataClass);
    }
    Ok(())
}
