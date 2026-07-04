use ocentra_schema::parent_owned_sync_export as contracts;

use super::{ParentOwnedSyncExportDerivationError, ParentOwnedSyncStateInput};

pub(super) fn derive_parent_owned_sync_state_row(
    input: ParentOwnedSyncStateInput,
) -> Result<contracts::ParentOwnedSyncStateRow, ParentOwnedSyncExportDerivationError> {
    match input.sync_state {
        contracts::ParentOwnedSyncState::Synced | contracts::ParentOwnedSyncState::Stale => {
            if input.cursor_ref.is_none()
                || input.batch_ref.is_none()
                || input.manifest_checksum_ref.is_none()
                || input.manifest_signature_ref.is_none()
                || input.last_successful_sync_at.is_none()
            {
                return Err(
                    ParentOwnedSyncExportDerivationError::SuccessfulSyncRequiresCursorBatchChecksumAndSignature,
                );
            }
            if input.manifest_integrity_state
                == contracts::ParentOwnedSyncManifestIntegrityState::Corrupt
            {
                return Err(ParentOwnedSyncExportDerivationError::CorruptManifestCannotClaimSynced);
            }
        }
        contracts::ParentOwnedSyncState::Conflict => {
            if input.conflict_ref.is_none() {
                return Err(ParentOwnedSyncExportDerivationError::ConflictStateRequiresConflictRef);
            }
            if input.retry_queue_ref.is_none() {
                return Err(ParentOwnedSyncExportDerivationError::RetryStateRequiresQueueRef);
            }
        }
        contracts::ParentOwnedSyncState::OfflineRetryPending
        | contracts::ParentOwnedSyncState::PartialOutage
        | contracts::ParentOwnedSyncState::ManualRequired
        | contracts::ParentOwnedSyncState::Missing => {
            if input.retry_queue_ref.is_none() {
                return Err(ParentOwnedSyncExportDerivationError::RetryStateRequiresQueueRef);
            }
        }
        contracts::ParentOwnedSyncState::NotStarted => {
            if input.cursor_ref.is_some()
                || input.batch_ref.is_some()
                || input.manifest_checksum_ref.is_some()
                || input.manifest_signature_ref.is_some()
                || input.last_successful_sync_at.is_some()
                || input.conflict_ref.is_some()
                || input.retry_queue_ref.is_some()
            {
                return Err(
                    ParentOwnedSyncExportDerivationError::NotStartedStateCannotHaveOperationalRefs,
                );
            }
        }
    }

    Ok(contracts::ParentOwnedSyncStateRow {
        sync_state: input.sync_state,
        provider_status_ref: input.provider_status_ref,
        cursor_ref: input.cursor_ref,
        batch_ref: input.batch_ref,
        manifest_integrity_state: input.manifest_integrity_state,
        manifest_checksum_ref: input.manifest_checksum_ref,
        manifest_signature_ref: input.manifest_signature_ref,
        last_successful_sync_at: input.last_successful_sync_at,
        conflict_ref: input.conflict_ref,
        retry_queue_ref: input.retry_queue_ref,
        parent_action_required: input.parent_action_required,
        claim_safe: true,
    })
}
