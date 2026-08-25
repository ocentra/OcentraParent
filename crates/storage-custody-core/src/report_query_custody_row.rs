use chrono::{DateTime, Utc};
use ocentra_family_identity_core::account_identity_authority::VerifiedAccountIdentityAuthority;
use ocentra_schema::report_query_custody as contracts;

use super::report_query_custody_source::ReportQueryCustodySourceResolution;
use super::ReportQueryCustodyDerivationError;

pub(super) fn derive_report_query_custody_row(
    request: &contracts::ReportQueryCustodyRequest,
    source: ReportQueryCustodySourceResolution,
    authority: &VerifiedAccountIdentityAuthority,
) -> Result<contracts::ReportQueryCustodyRow, ReportQueryCustodyDerivationError> {
    derive_report_query_custody_row_at(request, source, authority, Utc::now())
}

pub(super) fn derive_report_query_custody_row_at(
    request: &contracts::ReportQueryCustodyRequest,
    source: ReportQueryCustodySourceResolution,
    authority: &VerifiedAccountIdentityAuthority,
    resolved_at: DateTime<Utc>,
) -> Result<contracts::ReportQueryCustodyRow, ReportQueryCustodyDerivationError> {
    super::report_query_custody_request_validate::validate_report_query_custody_request_at(
        request,
        authority,
        resolved_at,
    )?;
    if !source.matches_authority_at(authority, resolved_at) {
        return Err(ReportQueryCustodyDerivationError::TrustedSourceResolutionUnavailable);
    }
    let input = source.into_input();
    super::report_query_custody_row_validate::validate_report_query_custody_input_at(
        request,
        &input,
        authority,
        resolved_at,
    )?;
    let (state, source_freshness, payload_redaction_state, tombstone_state) =
        super::report_query_custody_row_state::report_query_custody_state(&input)?;
    Ok(contracts::ReportQueryCustodyRow {
        row_id: input.row_id,
        request_id: request.request_id.clone(),
        state,
        source_freshness,
        source_data_class: input.source_data_class,
        cursor_ref: input.cursor_ref,
        source_cursor_ref: input.source_cursor_ref,
        next_cursor_ref: input.next_cursor_ref,
        page_index: input.page_index,
        page_size: request.page_size,
        stable_sort_key: input.stable_sort_key,
        requested_data_classes: request.requested_data_classes.clone(),
        allowed_source_data_classes: request.allowed_source_data_classes.clone(),
        source_citation_refs: request.source_citation_refs.clone(),
        assistant_citation_refs: request.assistant_citation_refs.clone(),
        notification_payload_boundary: request.notification_payload_boundary,
        payload_redaction_state,
        tombstone_state,
        deleted_source_ref: input.deleted_source_ref,
        deleted_source_at: input.deleted_source_at,
        conflict_ref: input.conflict_ref,
        cursor_expired_at: input.cursor_expired_at,
        rate_limited_until_at: input.rate_limited_until_at,
        parent_authority: request.parent_authority.clone(),
        raw_child_evidence_included: input.raw_child_evidence_included,
        report_cache_mutated: false,
        second_truth_store_claimed: false,
        claim_safe: true,
    })
}
