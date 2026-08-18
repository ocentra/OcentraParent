use chrono::Utc;
use ocentra_family_identity_core::account_identity_authority::VerifiedAccountIdentityAuthority;
use std::collections::BTreeSet;

use ocentra_schema::report_query_custody as contracts;

use super::{
    report_query_custody_source::ReportQueryCustodySourceResolution,
    ReportQueryCustodyDerivationError,
};

pub(super) fn build_report_query_custody_proof(
    request: &contracts::ReportQueryCustodyRequest,
    sources: Vec<ReportQueryCustodySourceResolution>,
    updated_at: contracts::ParentTimestamp,
    authority: &VerifiedAccountIdentityAuthority,
) -> Result<contracts::ReportQueryCustodyContractProof, ReportQueryCustodyDerivationError> {
    let resolved_at = Utc::now();
    super::report_query_custody_request_validate::validate_report_query_custody_request_at(
        request,
        authority,
        resolved_at,
    )?;

    let mut rows = Vec::with_capacity(sources.len());
    let mut seen_cursor_refs: BTreeSet<contracts::ReportQueryCustodyCursorRef> = BTreeSet::new();
    let mut seen_source_refs: BTreeSet<contracts::ReportQueryCustodySourceRef> = BTreeSet::new();
    let mut seen_sort_keys: BTreeSet<contracts::ReportQueryCustodySortKey> = BTreeSet::new();
    for source in sources {
        let row = super::report_query_custody_row::derive_report_query_custody_row_at(
            request,
            source,
            authority,
            resolved_at,
        )?;
        if !seen_cursor_refs.insert(row.cursor_ref.clone()) {
            return Err(ReportQueryCustodyDerivationError::DuplicateCursorRef);
        }
        if !seen_source_refs.insert(row.row_id.clone()) {
            return Err(ReportQueryCustodyDerivationError::DuplicateSourceRef);
        }
        if !seen_sort_keys.insert(row.stable_sort_key.clone()) {
            return Err(ReportQueryCustodyDerivationError::DuplicateStableSortKey);
        }
        if row.page_index != (rows.len() as u32).saturating_add(1) {
            return Err(ReportQueryCustodyDerivationError::NonSequentialPageIndex);
        }
        if let Some(next_cursor_ref) = row.next_cursor_ref.as_ref() {
            if seen_cursor_refs.contains(next_cursor_ref) {
                return Err(ReportQueryCustodyDerivationError::CursorContinuityMismatch);
            }
        }
        if let Some(previous) = rows.last() {
            if previous.next_cursor_ref.as_ref() != Some(&row.cursor_ref) {
                return Err(ReportQueryCustodyDerivationError::CursorContinuityMismatch);
            }
            if previous.source_cursor_ref != row.source_cursor_ref {
                return Err(ReportQueryCustodyDerivationError::SourceCursorContinuityMismatch);
            }
            if previous.stable_sort_key >= row.stable_sort_key {
                return Err(ReportQueryCustodyDerivationError::NonMonotonicStableSortKey);
            }
        } else if row.cursor_ref.to_string() != request.requested_cursor.to_string() {
            return Err(ReportQueryCustodyDerivationError::CursorContinuityMismatch);
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
