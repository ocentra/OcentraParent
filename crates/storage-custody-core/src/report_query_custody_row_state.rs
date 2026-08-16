use ocentra_schema::report_query_custody as contracts;

use super::{ReportQueryCustodyDerivationError, ReportQueryCustodyDerivationInput};
use crate::report_query_custody::ReportQueryCustodySignal;

#[path = "report_query_custody_row_state_required.rs"]
mod report_query_custody_row_state_required;
#[path = "report_query_custody_row_state_shared.rs"]
mod report_query_custody_row_state_shared;
#[path = "report_query_custody_row_state_terminal.rs"]
mod report_query_custody_row_state_terminal;

pub(super) fn report_query_custody_state(
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
        ReportQueryCustodySignal::Fresh => {
            report_query_custody_row_state_required::fresh_report_query_custody_state(input)
        }
        ReportQueryCustodySignal::Stale => {
            report_query_custody_row_state_required::stale_report_query_custody_state(input)
        }
        ReportQueryCustodySignal::PartiallyRedacted => {
            report_query_custody_row_state_required::partially_redacted_report_query_custody_state(
                input,
            )
        }
        ReportQueryCustodySignal::Deleted => {
            report_query_custody_row_state_terminal::deleted_report_query_custody_state(input)
        }
        ReportQueryCustodySignal::Conflict => {
            report_query_custody_row_state_terminal::conflict_report_query_custody_state(input)
        }
        ReportQueryCustodySignal::CursorExpired => {
            report_query_custody_row_state_terminal::cursor_expired_report_query_custody_state(
                input,
            )
        }
        ReportQueryCustodySignal::RateLimited => {
            report_query_custody_row_state_terminal::rate_limited_report_query_custody_state(input)
        }
    }
}
