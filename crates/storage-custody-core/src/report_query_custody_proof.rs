use ocentra_family_identity_core::account_identity_authority::VerifiedAccountIdentityAuthority;
use ocentra_schema::report_query_custody as contracts;

use super::{ReportQueryCustodyDerivationError, ReportQueryCustodyDerivationInput};

#[path = "report_query_custody_request_validate.rs"]
mod report_query_custody_request_validate;

pub(super) fn build_report_query_custody_proof(
    request: &contracts::ReportQueryCustodyRequest,
    _inputs: Vec<ReportQueryCustodyDerivationInput>,
    _updated_at: contracts::ParentTimestamp,
    authority: VerifiedAccountIdentityAuthority,
) -> Result<contracts::ReportQueryCustodyContractProof, ReportQueryCustodyDerivationError> {
    report_query_custody_request_validate::validate_report_query_custody_request(
        request, &authority,
    )?;

    // Proof construction cannot promote caller-provided source references.
    // It stays unavailable until the query-store owner exposes a resolved,
    // current source identity boundary.
    Err(ReportQueryCustodyDerivationError::TrustedSourceResolutionUnavailable)
}
