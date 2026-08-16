use ocentra_schema::report_query_custody as contracts;

use super::{
    report_query_custody_row_state_shared::{
        reject_next_cursor, require_deleted_source_metadata, require_next_cursor,
    },
    ReportQueryCustodyDerivationError, ReportQueryCustodyDerivationInput,
};

pub(super) fn deleted_report_query_custody_state(
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
    require_deleted_source_metadata(input)?;
    Ok((
        contracts::ReportQueryCustodyState::DeletedSource,
        contracts::ReportQueryCustodySourceFreshness::Deleted,
        contracts::ReportQueryCustodyPayloadRedaction::FullyRedacted,
        contracts::ReportQueryCustodyTombstoneState::Written,
    ))
}

pub(super) fn conflict_report_query_custody_state(
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

pub(super) fn cursor_expired_report_query_custody_state(
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

pub(super) fn rate_limited_report_query_custody_state(
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
