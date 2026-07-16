use ocentra_schema::retention_delete_tombstone as contracts;

use super::{RetentionDeleteDerivationError, RetentionDeleteDerivationInput};

#[path = "retention_delete_tombstone_proof_logic.rs"]
mod retention_delete_tombstone_proof_logic;

pub(super) fn build_retention_delete_tombstone_proof(
    request: &contracts::RetentionDeleteRequest,
    inputs: Vec<RetentionDeleteDerivationInput>,
    updated_at: contracts::RetentionDeleteTimestamp,
) -> Result<contracts::RetentionDeleteTombstoneContractProof, RetentionDeleteDerivationError> {
    retention_delete_tombstone_proof_logic::build_retention_delete_tombstone_proof(
        request, inputs, updated_at,
    )
}
