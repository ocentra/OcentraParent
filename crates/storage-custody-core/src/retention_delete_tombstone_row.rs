use ocentra_schema::retention_delete_tombstone as contracts;

use super::{RetentionDeleteDerivationError, RetentionDeleteDerivationInput};

#[path = "retention_delete_tombstone_row_logic.rs"]
mod retention_delete_tombstone_row_logic;

pub(super) fn derive_retention_delete_tombstone_row(
    request: &contracts::RetentionDeleteRequest,
    input: RetentionDeleteDerivationInput,
) -> Result<contracts::RetentionDeleteRow, RetentionDeleteDerivationError> {
    retention_delete_tombstone_row_logic::derive_retention_delete_tombstone_row(request, input)
}
