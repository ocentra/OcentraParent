use ocentra_schema::report_query_custody as contracts;

use super::{ReportQueryCustodyDerivationError, ReportQueryCustodyDerivationInput};

#[path = "report_query_custody_row_state.rs"]
mod report_query_custody_row_state;
#[path = "report_query_custody_row_validate.rs"]
mod report_query_custody_row_validate;

pub(super) fn derive_report_query_custody_row(
    request: &contracts::ReportQueryCustodyRequest,
    input: ReportQueryCustodyDerivationInput,
) -> Result<contracts::ReportQueryCustodyRow, ReportQueryCustodyDerivationError> {
    report_query_custody_row_validate::validate_report_query_custody_input(request, &input)?;
    let (state, source_freshness, payload_redaction_state, tombstone_state) =
        report_query_custody_row_state::report_query_custody_state(&input)?;

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
        parent_authorized: request.parent_authorized,
        parent_owned_source_required: request.parent_owned_source_required,
        raw_child_evidence_included: false,
        report_cache_mutated: false,
        second_truth_store_claimed: false,
        claim_safe: true,
    })
}
