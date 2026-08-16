use ocentra_schema::parent_owned_sync_export as contracts;

use super::{ParentOwnedSyncExportDerivationError, ParentOwnedSyncProviderStatusInput};

#[path = "parent_owned_sync_export_provider_validate.rs"]
mod parent_owned_sync_export_provider_validate;

pub(super) fn derive_parent_owned_sync_provider_status_row(
    input: ParentOwnedSyncProviderStatusInput,
) -> Result<contracts::ParentOwnedSyncProviderStatusRow, ParentOwnedSyncExportDerivationError> {
    parent_owned_sync_export_provider_validate::validate_parent_owned_sync_provider_status(&input)?;

    Ok(contracts::ParentOwnedSyncProviderStatusRow {
        provider_id: input.provider_id,
        provider_mode: input.provider_mode,
        provider_status: input.provider_status,
        destination_ownership: input.destination_ownership,
        account_ref: input.account_ref,
        folder_ref: input.folder_ref,
        status_ref: input.status_ref,
        revocation_ref: input.revocation_ref,
        disconnect_visibility_state: input.disconnect_visibility_state,
        delete_visibility_state: input.delete_visibility_state,
        last_checked_at: input.last_checked_at,
        oauth_runtime_claimed: false,
        upload_runtime_claimed: false,
        delete_runtime_claimed: false,
        ocentra_hosted_family_data_stored: false,
        claim_safe: true,
    })
}
