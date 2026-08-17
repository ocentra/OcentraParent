use ocentra_schema::parent_owned_sync_export as contracts;

use super::ParentOwnedSyncExportDerivationError;

pub(super) fn validate_manifest(
    manifest: &contracts::ParentOwnedSyncExportManifest,
) -> Result<(), ParentOwnedSyncExportDerivationError> {
    if manifest
        .items
        .iter()
        .any(|item| !is_honest_manifest_item(item))
    {
        return Err(ParentOwnedSyncExportDerivationError::ManifestItemNotClaimSafe);
    }
    Ok(())
}

fn is_honest_manifest_item(item: &contracts::ParentOwnedSyncExportManifestItem) -> bool {
    item.parent_action_required
        && !item.raw_child_evidence_uploaded_by_default
        && !item.ocentra_hosted_family_data_stored
        && item.claim_safe
        && item.destination_ownership
            != contracts::ParentOwnedSyncExportDestinationOwnership::OcentraHostedNonActivityMetadata
        && encryption_matches_format(item)
}

fn encryption_matches_format(item: &contracts::ParentOwnedSyncExportManifestItem) -> bool {
    match item.export_format {
        contracts::ParentOwnedSyncExportFormat::EncryptedMachineReadable
        | contracts::ParentOwnedSyncExportFormat::EncryptedSupportBundle => {
            item.encryption.encryption_state
                == contracts::ParentOwnedSyncExportEncryptionState::EncryptedAtRest
                && item.encryption.encrypted_before_upload
        }
        contracts::ParentOwnedSyncExportFormat::HumanReadableParentReport => {
            item.encryption.encryption_state
                == contracts::ParentOwnedSyncExportEncryptionState::HumanReadableParentAuthorized
                && !item.encryption.encrypted_before_upload
        }
    }
}
