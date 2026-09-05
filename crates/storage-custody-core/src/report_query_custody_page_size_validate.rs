use super::ReportQueryCustodyDerivationError;
use ocentra_schema::report_query_custody as contracts;

pub(super) fn validate_report_query_custody_page_size(
    page_size: u32,
) -> Result<(), ReportQueryCustodyDerivationError> {
    if page_size == 0 {
        return Err(ReportQueryCustodyDerivationError::NonPositivePageSize);
    }
    if page_size > contracts::REPORT_QUERY_CUSTODY_MAX_PAGE_SIZE {
        return Err(ReportQueryCustodyDerivationError::PageSizeExceedsLimit);
    }
    Ok(())
}
