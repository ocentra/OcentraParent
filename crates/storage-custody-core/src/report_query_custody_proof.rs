use std::collections::BTreeSet;

use ocentra_schema::report_query_custody as contracts;

use super::{
    derive_report_query_custody_row, ReportQueryCustodyDerivationError,
    ReportQueryCustodyDerivationInput,
};

pub(super) fn build_report_query_custody_proof(
    request: &contracts::ReportQueryCustodyRequest,
    inputs: Vec<ReportQueryCustodyDerivationInput>,
    updated_at: contracts::ParentTimestamp,
) -> Result<contracts::ReportQueryCustodyContractProof, ReportQueryCustodyDerivationError> {
    let mut rows = inputs
        .into_iter()
        .map(|input| derive_report_query_custody_row(request, input))
        .collect::<Result<Vec<_>, _>>()?;

    rows.sort_by_key(|row| row.page_index);

    if rows
        .iter()
        .enumerate()
        .any(|(index, row)| row.page_index != index as u32 + 1)
    {
        return Err(ReportQueryCustodyDerivationError::NonSequentialPageIndex);
    }

    let mut seen_cursor_refs = BTreeSet::new();
    for row in &rows {
        if !seen_cursor_refs.insert(row.cursor_ref.to_string()) {
            return Err(ReportQueryCustodyDerivationError::DuplicateCursorRef);
        }
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
