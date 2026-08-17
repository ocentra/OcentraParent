use ocentra_family_identity_core::account_identity_authority::VerifiedAccountIdentityAuthority;
use ocentra_schema::report_query_custody as contracts;

use super::{ReportQueryCustodyDerivationError, ReportQueryCustodyDerivationInput};

#[path = "report_query_custody_request_validate.rs"]
mod report_query_custody_request_validate;

pub(super) fn derive_report_query_custody_row(
    request: &contracts::ReportQueryCustodyRequest,
    _input: ReportQueryCustodyDerivationInput,
    authority: VerifiedAccountIdentityAuthority,
) -> Result<contracts::ReportQueryCustodyRow, ReportQueryCustodyDerivationError> {
    report_query_custody_request_validate::validate_report_query_custody_request(
        request, &authority,
    )?;

    // No production query-store resolver currently issues a trusted source
    // identity for this contract. Request citations and row/source identifiers
    // are transport data and cannot authorize access or become derived rows.
    Err(ReportQueryCustodyDerivationError::TrustedSourceResolutionUnavailable)
}
