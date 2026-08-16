use std::collections::HashSet;

use ocentra_schema::retention_delete_tombstone as contracts;

use super::{RetentionDeleteDerivationError, RetentionDeleteDerivationInput};

pub(super) fn collect_retention_delete_tombstone_rows(
    request: &contracts::RetentionDeleteRequest,
    inputs: Vec<RetentionDeleteDerivationInput>,
) -> Result<Vec<contracts::RetentionDeleteRow>, RetentionDeleteDerivationError> {
    let mut rows = Vec::with_capacity(inputs.len());
    let mut seen_states = HashSet::with_capacity(inputs.len());

    for input in inputs {
        let state = self::retention_delete_tombstone_proof_signal::signal_state(input.signal);
        if !seen_states.insert(state.as_str()) {
            return Err(RetentionDeleteDerivationError::DuplicateState(state));
        }
        rows.push(
            crate::retention_delete_tombstone::retention_delete_tombstone_row::derive_retention_delete_tombstone_row(
                request,
                input,
            )?,
        );
    }

    rows.sort_by_key(|row| self::retention_delete_tombstone_proof_rank::state_rank(row.state));
    Ok(rows)
}

#[path = "retention_delete_tombstone_proof_rank.rs"]
mod retention_delete_tombstone_proof_rank;
#[path = "retention_delete_tombstone_proof_signal.rs"]
mod retention_delete_tombstone_proof_signal;
