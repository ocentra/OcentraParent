use ocentra_family_identity_core::account_identity_authority::VerifiedAccountIdentityAuthority;
use std::collections::BTreeSet;

use ocentra_schema::report_query_custody as contracts;

use super::{
    ReportQueryCustodyDerivationError,
    report_query_custody_source::ReportQueryCustodySourceResolution,
};

pub(super) fn build_report_query_custody_proof(
    request: &contracts::ReportQueryCustodyRequest,
    sources: Vec<ReportQueryCustodySourceResolution>,
    updated_at: contracts::ParentTimestamp,
    authority: VerifiedAccountIdentityAuthority,
) -> Result<contracts::ReportQueryCustodyContractProof, ReportQueryCustodyDerivationError> {
    super::report_query_custody_request_validate::validate_report_query_custody_request(
        request, &authority,
    )?;

    let mut rows = Vec::with_capacity(sources.len());
    let mut seen_cursor_refs = BTreeSet::new();
    for source in sources {
        let row = super::report_query_custody_row::derive_report_query_custody_row(
            request,
            source,
            authority.clone(),
        )?;
        if !seen_cursor_refs.insert(row.cursor_ref.to_string()) {
            return Err(ReportQueryCustodyDerivationError::DuplicateCursorRef);
        }
        if row.page_index != (rows.len() as u32).saturating_add(1) {
            return Err(ReportQueryCustodyDerivationError::NonSequentialPageIndex);
        }
        rows.push(row);
    }

    Ok(contracts::ReportQueryCustodyContractProof {
        schema_version: contracts::REPORT_QUERY_CUSTODY_SCHEMA_VERSION.to_string(),
        contract_version: contracts::ParentContractSchemaVersion::parse("v0.6")
            .ok_or(ReportQueryCustodyDerivationError::InvalidContractVersion)?,
        request: request.clone(),
        rows,
        non_claims: contracts::required_report_query_custody_non_claims(),
        report_runtime_claimed: false,
        portal_ui_claimed: false,
        provider_routing_claimed: false,
        ocentra_hosted_family_data_custody_claimed: false,
        second_truth_store_claimed: false,
        raw_child_evidence_claimed: false,
        updated_at,
    })
}
