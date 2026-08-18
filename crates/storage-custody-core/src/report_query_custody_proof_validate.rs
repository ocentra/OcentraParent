use ocentra_schema::report_query_custody as contracts;

use super::ReportQueryCustodyDerivationError;

pub(super) fn validate_required_states(
    rows: &[contracts::ReportQueryCustodyRow],
) -> Result<(), ReportQueryCustodyDerivationError> {
    for required_state in contracts::required_report_query_custody_states() {
        if !rows.iter().any(|row| row.state == required_state) {
            return Err(ReportQueryCustodyDerivationError::MissingRequiredState(
                required_state,
            ));
        }
    }
    Ok(())
}
