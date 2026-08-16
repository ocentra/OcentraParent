use super::{ParentOwnedSyncExportDerivationError, ParentOwnedSyncProviderStatusInput};

#[path = "parent_owned_sync_export_provider_validate_destination.rs"]
mod parent_owned_sync_export_provider_validate_destination;
#[path = "parent_owned_sync_export_provider_validate_refs.rs"]
mod parent_owned_sync_export_provider_validate_refs;
#[path = "parent_owned_sync_export_provider_validate_visibility.rs"]
mod parent_owned_sync_export_provider_validate_visibility;

pub(super) fn validate_parent_owned_sync_provider_status(
    input: &ParentOwnedSyncProviderStatusInput,
) -> Result<(), ParentOwnedSyncExportDerivationError> {
    parent_owned_sync_export_provider_validate_destination::validate_destination_ownership(input)?;
    parent_owned_sync_export_provider_validate_refs::validate_provider_status_refs(input)?;
    parent_owned_sync_export_provider_validate_visibility::validate_provider_visibility(input)?;
    Ok(())
}
