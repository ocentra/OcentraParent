use std::collections::BTreeSet;

use ocentra_schema::parent_owned_sync_export as contracts;

use super::{
    derive_parent_owned_sync_provider_status_row, derive_parent_owned_sync_state_row,
    derive_parent_owned_sync_tombstone_row, ParentOwnedSyncExportDerivationError,
    ParentOwnedSyncProviderStatusInput, ParentOwnedSyncStateInput, ParentOwnedSyncTombstoneInput,
};

#[path = "parent_owned_sync_export_manifest.rs"]
mod parent_owned_sync_export_manifest;

pub(super) fn build_parent_owned_sync_export_proof(
    manifest: &contracts::ParentOwnedSyncExportManifest,
    provider_inputs: Vec<ParentOwnedSyncProviderStatusInput>,
    sync_inputs: Vec<ParentOwnedSyncStateInput>,
    tombstone_inputs: Vec<ParentOwnedSyncTombstoneInput>,
    updated_at: contracts::ParentTimestamp,
) -> Result<contracts::ParentOwnedSyncExportContractProof, ParentOwnedSyncExportDerivationError> {
    parent_owned_sync_export_manifest::validate_manifest(manifest)?;

    let provider_statuses = provider_inputs
        .into_iter()
        .map(derive_parent_owned_sync_provider_status_row)
        .collect::<Result<Vec<_>, _>>()?;
    let sync_states = sync_inputs
        .into_iter()
        .map(derive_parent_owned_sync_state_row)
        .collect::<Result<Vec<_>, _>>()?;
    let tombstones = tombstone_inputs
        .into_iter()
        .map(derive_parent_owned_sync_tombstone_row)
        .collect::<Result<Vec<_>, _>>()?;

    let mut seen_status_refs = BTreeSet::new();
    for row in &provider_statuses {
        if !seen_status_refs.insert(row.status_ref.as_str().to_owned()) {
            return Err(ParentOwnedSyncExportDerivationError::DuplicateProviderStatusRef);
        }
    }

    let mut seen_tombstone_refs = BTreeSet::new();
    for row in &tombstones {
        if !seen_tombstone_refs.insert(row.tombstone_ref.as_str().to_owned()) {
            return Err(ParentOwnedSyncExportDerivationError::DuplicateTombstoneRef);
        }
    }

    Ok(contracts::ParentOwnedSyncExportContractProof {
        schema_version: contracts::PARENT_OWNED_SYNC_EXPORT_SCHEMA_VERSION.to_string(),
        contract_version: contracts::ParentContractSchemaVersion::parse("v0.6")
            .ok_or(ParentOwnedSyncExportDerivationError::InvalidContractVersion)?,
        manifest: manifest.clone(),
        provider_statuses,
        sync_states,
        tombstones,
        non_claims: contracts::required_parent_owned_sync_export_non_claims(),
        transfer_runtime_claimed: false,
        connector_o_auth_claimed: false,
        upload_runtime_claimed: false,
        delete_runtime_claimed: false,
        ocentra_hosted_child_evidence_stored: false,
        updated_at,
    })
}
