use ocentra_schema::report_query_custody as contracts;

use super::{
    report_query_custody_row_state_shared::require_next_cursor, ReportQueryCustodyDerivationError,
    ReportQueryCustodyDerivationInput,
};

pub(super) fn fresh_report_query_custody_state(
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

pub(super) fn stale_report_query_custody_state(
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

pub(super) fn partially_redacted_report_query_custody_state(
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
