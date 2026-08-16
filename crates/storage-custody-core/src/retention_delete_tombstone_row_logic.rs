use ocentra_schema::retention_delete_tombstone as contracts;

use super::{RetentionDeleteDerivationError, RetentionDeleteDerivationInput};

#[path = "retention_delete_tombstone_row_request.rs"]
mod retention_delete_tombstone_row_request;
#[path = "retention_delete_tombstone_row_state.rs"]
mod retention_delete_tombstone_row_state;

pub(super) fn derive_retention_delete_tombstone_row(
    request: &contracts::RetentionDeleteRequest,
    input: RetentionDeleteDerivationInput,
) -> Result<contracts::RetentionDeleteRow, RetentionDeleteDerivationError> {
    retention_delete_tombstone_row_request::derive_retention_delete_tombstone_row(request, input)
}
