use std::collections::BTreeSet;

use ocentra_schema::report_query_custody as contracts;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportQueryCustodySignal {
    Fresh,
    Stale,
    PartiallyRedacted,
    Deleted,
    Conflict,
    CursorExpired,
    RateLimited,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReportQueryCustodyDerivationInput {
    pub row_id: contracts::ReportQueryCustodySourceRef,
    pub source_data_class: contracts::ReportQueryCustodySourceDataClass,
    pub signal: ReportQueryCustodySignal,
    pub cursor_ref: contracts::ReportQueryCustodyCursorRef,
    pub source_cursor_ref: contracts::ReportQueryCustodyCursorRef,
    pub next_cursor_ref: Option<contracts::ReportQueryCustodyCursorRef>,
    pub page_index: u32,
    pub stable_sort_key: contracts::ReportQueryCustodySortKey,
    pub deleted_source_ref: Option<contracts::ReportQueryCustodyDeletedSourceRef>,
    pub deleted_source_at: Option<contracts::ParentTimestamp>,
    pub conflict_ref: Option<contracts::ReportQueryCustodyConflictRef>,
    pub cursor_expired_at: Option<contracts::ParentTimestamp>,
    pub rate_limited_until_at: Option<contracts::ParentTimestamp>,
    pub raw_child_evidence_included: bool,
    pub tombstone_confirmed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReportQueryCustodyDerivationError {
    DisallowedSourceDataClass,
    RawChildEvidenceRequested,
    NonPositivePageIndex,
    MissingNextCursor,
    UnexpectedNextCursor,
    MissingDeletedSourceMetadata,
    TombstoneRequiredForDeletedSource,
    MissingConflictRef,
    MissingCursorExpiredAt,
    MissingRateLimitedUntilAt,
    DuplicateCursorRef,
    NonSequentialPageIndex,
}

fn option_or_unreachable<T>(value: Option<T>, context: &str) -> T {
    match value {
        Some(value) => value,
        None => unreachable!("{context}"),
    }
}

pub fn derive_report_query_custody_row(
    request: &contracts::ReportQueryCustodyRequest,
    input: ReportQueryCustodyDerivationInput,
) -> Result<contracts::ReportQueryCustodyRow, ReportQueryCustodyDerivationError> {
    validate_report_query_custody_input(request, &input)?;
    let (state, source_freshness, payload_redaction_state, tombstone_state) =
        report_query_custody_state(&input)?;

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

fn validate_report_query_custody_input(
    request: &contracts::ReportQueryCustodyRequest,
    input: &ReportQueryCustodyDerivationInput,
) -> Result<(), ReportQueryCustodyDerivationError> {
    if input.raw_child_evidence_included {
        return Err(ReportQueryCustodyDerivationError::RawChildEvidenceRequested);
    }
    if input.page_index == 0 {
        return Err(ReportQueryCustodyDerivationError::NonPositivePageIndex);
    }
    if !request
        .allowed_source_data_classes
        .contains(&input.source_data_class)
    {
        return Err(ReportQueryCustodyDerivationError::DisallowedSourceDataClass);
    }
    Ok(())
}

fn report_query_custody_state(
    input: &ReportQueryCustodyDerivationInput,
) -> Result<
    (
        contracts::ReportQueryCustodyState,
        contracts::ReportQueryCustodySourceFreshness,
        contracts::ReportQueryCustodyPayloadRedaction,
        contracts::ReportQueryCustodyTombstoneState,
    ),
    ReportQueryCustodyDerivationError,
> {
    match input.signal {
        ReportQueryCustodySignal::Fresh => fresh_report_query_custody_state(input),
        ReportQueryCustodySignal::Stale => stale_report_query_custody_state(input),
        ReportQueryCustodySignal::PartiallyRedacted => {
            partially_redacted_report_query_custody_state(input)
        }
        ReportQueryCustodySignal::Deleted => deleted_report_query_custody_state(input),
        ReportQueryCustodySignal::Conflict => conflict_report_query_custody_state(input),
        ReportQueryCustodySignal::CursorExpired => cursor_expired_report_query_custody_state(input),
        ReportQueryCustodySignal::RateLimited => rate_limited_report_query_custody_state(input),
    }
}

fn fresh_report_query_custody_state(
    input: &ReportQueryCustodyDerivationInput,
) -> Result<
    (
        contracts::ReportQueryCustodyState,
        contracts::ReportQueryCustodySourceFreshness,
        contracts::ReportQueryCustodyPayloadRedaction,
        contracts::ReportQueryCustodyTombstoneState,
    ),
    ReportQueryCustodyDerivationError,
> {
    require_next_cursor(input)?;
    Ok((
        contracts::ReportQueryCustodyState::DerivedFresh,
        contracts::ReportQueryCustodySourceFreshness::Fresh,
        contracts::ReportQueryCustodyPayloadRedaction::FullyRedacted,
        contracts::ReportQueryCustodyTombstoneState::NotRequired,
    ))
}

fn stale_report_query_custody_state(
    input: &ReportQueryCustodyDerivationInput,
) -> Result<
    (
        contracts::ReportQueryCustodyState,
        contracts::ReportQueryCustodySourceFreshness,
        contracts::ReportQueryCustodyPayloadRedaction,
        contracts::ReportQueryCustodyTombstoneState,
    ),
    ReportQueryCustodyDerivationError,
> {
    require_next_cursor(input)?;
    Ok((
        contracts::ReportQueryCustodyState::DerivedStale,
        contracts::ReportQueryCustodySourceFreshness::Stale,
        contracts::ReportQueryCustodyPayloadRedaction::FullyRedacted,
        contracts::ReportQueryCustodyTombstoneState::NotRequired,
    ))
}

fn partially_redacted_report_query_custody_state(
    input: &ReportQueryCustodyDerivationInput,
) -> Result<
    (
        contracts::ReportQueryCustodyState,
        contracts::ReportQueryCustodySourceFreshness,
        contracts::ReportQueryCustodyPayloadRedaction,
        contracts::ReportQueryCustodyTombstoneState,
    ),
    ReportQueryCustodyDerivationError,
> {
    require_next_cursor(input)?;
    Ok((
        contracts::ReportQueryCustodyState::PartiallyRedacted,
        contracts::ReportQueryCustodySourceFreshness::Stale,
        contracts::ReportQueryCustodyPayloadRedaction::PartiallyRedacted,
        contracts::ReportQueryCustodyTombstoneState::NotRequired,
    ))
}

fn deleted_report_query_custody_state(
    input: &ReportQueryCustodyDerivationInput,
) -> Result<
    (
        contracts::ReportQueryCustodyState,
        contracts::ReportQueryCustodySourceFreshness,
        contracts::ReportQueryCustodyPayloadRedaction,
        contracts::ReportQueryCustodyTombstoneState,
    ),
    ReportQueryCustodyDerivationError,
> {
    reject_next_cursor(input)?;
    if input.deleted_source_ref.is_none() || input.deleted_source_at.is_none() {
        return Err(ReportQueryCustodyDerivationError::MissingDeletedSourceMetadata);
    }
    if !input.tombstone_confirmed {
        return Err(ReportQueryCustodyDerivationError::TombstoneRequiredForDeletedSource);
    }
    Ok((
        contracts::ReportQueryCustodyState::DeletedSource,
        contracts::ReportQueryCustodySourceFreshness::Deleted,
        contracts::ReportQueryCustodyPayloadRedaction::FullyRedacted,
        contracts::ReportQueryCustodyTombstoneState::Written,
    ))
}

fn conflict_report_query_custody_state(
    input: &ReportQueryCustodyDerivationInput,
) -> Result<
    (
        contracts::ReportQueryCustodyState,
        contracts::ReportQueryCustodySourceFreshness,
        contracts::ReportQueryCustodyPayloadRedaction,
        contracts::ReportQueryCustodyTombstoneState,
    ),
    ReportQueryCustodyDerivationError,
> {
    require_next_cursor(input)?;
    if input.conflict_ref.is_none() {
        return Err(ReportQueryCustodyDerivationError::MissingConflictRef);
    }
    Ok((
        contracts::ReportQueryCustodyState::SyncConflict,
        contracts::ReportQueryCustodySourceFreshness::Conflicted,
        contracts::ReportQueryCustodyPayloadRedaction::FullyRedacted,
        contracts::ReportQueryCustodyTombstoneState::NotRequired,
    ))
}

fn cursor_expired_report_query_custody_state(
    input: &ReportQueryCustodyDerivationInput,
) -> Result<
    (
        contracts::ReportQueryCustodyState,
        contracts::ReportQueryCustodySourceFreshness,
        contracts::ReportQueryCustodyPayloadRedaction,
        contracts::ReportQueryCustodyTombstoneState,
    ),
    ReportQueryCustodyDerivationError,
> {
    reject_next_cursor(input)?;
    if input.cursor_expired_at.is_none() {
        return Err(ReportQueryCustodyDerivationError::MissingCursorExpiredAt);
    }
    Ok((
        contracts::ReportQueryCustodyState::CursorExpired,
        contracts::ReportQueryCustodySourceFreshness::Expired,
        contracts::ReportQueryCustodyPayloadRedaction::FullyRedacted,
        contracts::ReportQueryCustodyTombstoneState::NotRequired,
    ))
}

fn rate_limited_report_query_custody_state(
    input: &ReportQueryCustodyDerivationInput,
) -> Result<
    (
        contracts::ReportQueryCustodyState,
        contracts::ReportQueryCustodySourceFreshness,
        contracts::ReportQueryCustodyPayloadRedaction,
        contracts::ReportQueryCustodyTombstoneState,
    ),
    ReportQueryCustodyDerivationError,
> {
    reject_next_cursor(input)?;
    if input.rate_limited_until_at.is_none() {
        return Err(ReportQueryCustodyDerivationError::MissingRateLimitedUntilAt);
    }
    Ok((
        contracts::ReportQueryCustodyState::RateLimited,
        contracts::ReportQueryCustodySourceFreshness::RateLimited,
        contracts::ReportQueryCustodyPayloadRedaction::FullyRedacted,
        contracts::ReportQueryCustodyTombstoneState::NotRequired,
    ))
}

pub fn build_report_query_custody_proof(
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
        if !seen_cursor_refs.insert(row.cursor_ref.as_str().to_owned()) {
            return Err(ReportQueryCustodyDerivationError::DuplicateCursorRef);
        }
    }

    Ok(contracts::ReportQueryCustodyContractProof {
        schema_version: contracts::REPORT_QUERY_CUSTODY_SCHEMA_VERSION.to_string(),
        contract_version: option_or_unreachable(
            contracts::ParentContractSchemaVersion::parse("v0.6"),
            "contract version",
        ),
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

fn require_next_cursor(
    input: &ReportQueryCustodyDerivationInput,
) -> Result<(), ReportQueryCustodyDerivationError> {
    if input.next_cursor_ref.is_none() {
        return Err(ReportQueryCustodyDerivationError::MissingNextCursor);
    }
    Ok(())
}

fn reject_next_cursor(
    input: &ReportQueryCustodyDerivationInput,
) -> Result<(), ReportQueryCustodyDerivationError> {
    if input.next_cursor_ref.is_some() {
        return Err(ReportQueryCustodyDerivationError::UnexpectedNextCursor);
    }
    Ok(())
}
