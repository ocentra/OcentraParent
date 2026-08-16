use ocentra_schema::parent_owned_sync_export as contracts;

use super::{ParentOwnedSyncExportDerivationError, ParentOwnedSyncProviderStatusInput};

pub(super) fn validate_destination_ownership(
    input: &ParentOwnedSyncProviderStatusInput,
) -> Result<(), ParentOwnedSyncExportDerivationError> {
    if input.destination_ownership
        == contracts::ParentOwnedSyncExportDestinationOwnership::OcentraHostedNonActivityMetadata
    {
        return Err(ParentOwnedSyncExportDerivationError::OcentraHostedCustodyForbidden);
    }
    Ok(())
}
