use super::ReportQueryCustodyDerivationError;

const REPORT_QUERY_CUSTODY_MAX_PAGE_SIZE: u32 = 100;

pub(super) fn validate_report_query_custody_page_size(
    page_size: u32,
) -> Result<(), ReportQueryCustodyDerivationError> {
    if page_size == 0 {
        return Err(ReportQueryCustodyDerivationError::NonPositivePageSize);
    }
    if page_size > REPORT_QUERY_CUSTODY_MAX_PAGE_SIZE {
        return Err(ReportQueryCustodyDerivationError::PageSizeExceedsLimit);
    }
    Ok(())
}
