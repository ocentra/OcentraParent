use chrono::{DateTime, Utc};
use ocentra_schema::report_query_custody as contracts;

use ocentra_family_identity_core::account_identity_authority::VerifiedAccountIdentityAuthority;

use super::{ReportQueryCustodyDerivationError, ReportQueryCustodyDerivationInput};

pub(super) fn validate_report_query_custody_input(
    request: &contracts::ReportQueryCustodyRequest,
    input: &ReportQueryCustodyDerivationInput,
    authority: &VerifiedAccountIdentityAuthority,
) -> Result<(), ReportQueryCustodyDerivationError> {
    validate_report_query_custody_input_at(request, input, authority, Utc::now())
}

pub(super) fn validate_report_query_custody_input_at(
    request: &contracts::ReportQueryCustodyRequest,
    input: &ReportQueryCustodyDerivationInput,
    authority: &VerifiedAccountIdentityAuthority,
    now: DateTime<Utc>,
) -> Result<(), ReportQueryCustodyDerivationError> {
    super::report_query_custody_request_validate::validate_report_query_custody_request_at(
        request, authority, now,
    )?;
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
