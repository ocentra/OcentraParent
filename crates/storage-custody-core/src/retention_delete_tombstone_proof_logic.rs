use ocentra_schema::retention_delete_tombstone as contracts;

use super::{RetentionDeleteDerivationError, RetentionDeleteDerivationInput};

#[path = "retention_delete_tombstone_proof_finalize.rs"]
mod retention_delete_tombstone_proof_finalize;
#[path = "retention_delete_tombstone_proof_rows.rs"]
mod retention_delete_tombstone_proof_rows;

pub(super) fn build_retention_delete_tombstone_proof(
    request: &contracts::RetentionDeleteRequest,
    inputs: Vec<RetentionDeleteDerivationInput>,
    updated_at: contracts::RetentionDeleteTimestamp,
) -> Result<contracts::RetentionDeleteTombstoneContractProof, RetentionDeleteDerivationError> {
    let rows =
        self::retention_delete_tombstone_proof_rows::collect_retention_delete_tombstone_rows(
            request, inputs,
        )?;
    self::retention_delete_tombstone_proof_finalize::build_retention_delete_tombstone_proof_finalize(
        request, rows, updated_at,
    )
}
