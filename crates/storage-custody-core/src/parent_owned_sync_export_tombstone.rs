use ocentra_schema::parent_owned_sync_export as contracts;

use super::{ParentOwnedSyncExportDerivationError, ParentOwnedSyncTombstoneInput};

pub(super) fn derive_parent_owned_sync_tombstone_row(
    input: ParentOwnedSyncTombstoneInput,
) -> Result<contracts::ParentOwnedSyncTombstoneRow, ParentOwnedSyncExportDerivationError> {
    match input.propagation_state {
        contracts::ParentOwnedSyncTombstonePropagationState::NotRequested => {
            if input.delete_request_ref.is_some()
                || input.last_propagated_at.is_some()
                || input.blocked_reason_ref.is_some()
            {
                return Err(ParentOwnedSyncExportDerivationError::TombstoneDeleteRequestMissing);
            }
        }
        contracts::ParentOwnedSyncTombstonePropagationState::Pending => {
            if input.delete_request_ref.is_none() {
                return Err(ParentOwnedSyncExportDerivationError::TombstoneDeleteRequestMissing);
            }
        }
        contracts::ParentOwnedSyncTombstonePropagationState::Propagated => {
            if input.delete_request_ref.is_none() {
                return Err(ParentOwnedSyncExportDerivationError::TombstoneDeleteRequestMissing);
            }
            if input.last_propagated_at.is_none() {
                return Err(
                    ParentOwnedSyncExportDerivationError::TombstonePropagatedRequiresTimestamp,
                );
            }
        }
        contracts::ParentOwnedSyncTombstonePropagationState::Blocked
        | contracts::ParentOwnedSyncTombstonePropagationState::ManualRequired => {
            if input.delete_request_ref.is_none() {
                return Err(ParentOwnedSyncExportDerivationError::TombstoneDeleteRequestMissing);
            }
            if input.blocked_reason_ref.is_none() {
                return Err(ParentOwnedSyncExportDerivationError::TombstoneBlockedRequiresReason);
            }
        }
    }

    Ok(contracts::ParentOwnedSyncTombstoneRow {
        tombstone_ref: input.tombstone_ref,
        data_class: input.data_class,
        propagation_state: input.propagation_state,
        delete_request_ref: input.delete_request_ref,
        provider_status_ref: input.provider_status_ref,
        last_propagated_at: input.last_propagated_at,
        blocked_reason_ref: input.blocked_reason_ref,
        claim_safe: true,
    })
}
