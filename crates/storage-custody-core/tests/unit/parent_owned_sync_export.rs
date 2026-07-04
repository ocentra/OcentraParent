use crate::support::StorageCustodyTestValueExt;

use ocentra_schema::parent_owned_sync_export as contracts;
use ocentra_storage_custody_core::parent_owned_sync_export::{
    build_parent_owned_sync_export_proof, derive_parent_owned_sync_provider_status_row,
    derive_parent_owned_sync_state_row, derive_parent_owned_sync_tombstone_row,
    ParentOwnedSyncExportDerivationError, ParentOwnedSyncProviderStatusInput,
    ParentOwnedSyncStateInput, ParentOwnedSyncTombstoneInput,
};

#[test]
fn provider_modes_and_claim_safe_statuses_stay_explicit() {
    let proof = contracts::sample_parent_owned_sync_export_contract_proof();

    assert_eq!(proof.provider_statuses.len(), 11);
    assert!(proof
        .provider_statuses
        .iter()
        .all(|row| row.claim_safe && !row.oauth_runtime_claimed && !row.upload_runtime_claimed));
    assert!(
        proof
            .provider_statuses
            .iter()
            .any(|row| row.provider_status
                == contracts::ParentOwnedSyncProviderStatus::ManualRequired)
    );
    assert!(proof.provider_statuses.iter().any(|row| {
        row.disconnect_visibility_state
            == contracts::ParentOwnedSyncDisconnectVisibilityState::DisconnectVisible
    }));
}

#[test]
fn ready_and_revoked_provider_rows_require_their_specific_refs() {
    let timestamp = contracts::ParentTimestamp::parse("2026-06-28T18:40:00.000Z").assume_ok();
    let ready_missing_refs =
        derive_parent_owned_sync_provider_status_row(ParentOwnedSyncProviderStatusInput {
            provider_id: contracts::ParentOwnedSyncProviderId::parse("provider-ready").assume_ok(),
            provider_mode: contracts::ParentOwnedSyncProviderMode::GoogleDriveAppdata,
            provider_status: contracts::ParentOwnedSyncProviderStatus::Ready,
            destination_ownership:
                contracts::ParentOwnedSyncExportDestinationOwnership::ParentOwnedExternalStorage,
            account_ref: None,
            folder_ref: None,
            status_ref: contracts::ParentOwnedSyncStatusRef::parse("provider-status-ready")
                .assume_ok(),
            revocation_ref: None,
            disconnect_visibility_state:
                contracts::ParentOwnedSyncDisconnectVisibilityState::NotDisconnected,
            delete_visibility_state: contracts::ParentOwnedSyncDeleteVisibilityState::NotRequested,
            last_checked_at: timestamp.clone(),
        });
    assert_eq!(
        ready_missing_refs,
        Err(ParentOwnedSyncExportDerivationError::ReadyProviderMissingLocationRefs)
    );

    let revoked_missing_ref =
        derive_parent_owned_sync_provider_status_row(ParentOwnedSyncProviderStatusInput {
            provider_id: contracts::ParentOwnedSyncProviderId::parse("provider-revoked")
                .assume_ok(),
            provider_mode: contracts::ParentOwnedSyncProviderMode::OnedriveApproot,
            provider_status: contracts::ParentOwnedSyncProviderStatus::Revoked,
            destination_ownership:
                contracts::ParentOwnedSyncExportDestinationOwnership::ParentOwnedExternalStorage,
            account_ref: contracts::ParentOwnedSyncProviderRef::parse("account-revoked"),
            folder_ref: contracts::ParentOwnedSyncProviderRef::parse("folder-revoked"),
            status_ref: contracts::ParentOwnedSyncStatusRef::parse("provider-status-revoked")
                .assume_ok(),
            revocation_ref: None,
            disconnect_visibility_state:
                contracts::ParentOwnedSyncDisconnectVisibilityState::NotDisconnected,
            delete_visibility_state: contracts::ParentOwnedSyncDeleteVisibilityState::NotRequested,
            last_checked_at: timestamp,
        });
    assert_eq!(
        revoked_missing_ref,
        Err(ParentOwnedSyncExportDerivationError::RevokedProviderMissingRevocationRef)
    );
}

#[test]
fn sync_states_require_manifest_integrity_and_retry_evidence_honestly() {
    let synced_missing_signature = derive_parent_owned_sync_state_row(ParentOwnedSyncStateInput {
        sync_state: contracts::ParentOwnedSyncState::Synced,
        provider_status_ref: contracts::ParentOwnedSyncStatusRef::parse("provider-status-ready")
            .assume_ok(),
        cursor_ref: contracts::ParentOwnedSyncCursorRef::parse("cursor-synced"),
        batch_ref: contracts::ParentOwnedSyncBatchRef::parse("batch-synced"),
        manifest_integrity_state: contracts::ParentOwnedSyncManifestIntegrityState::Verified,
        manifest_checksum_ref: contracts::ParentOwnedSyncChecksumRef::parse("checksum-synced"),
        manifest_signature_ref: None,
        last_successful_sync_at: contracts::ParentTimestamp::parse("2026-06-28T18:40:00.000Z"),
        conflict_ref: None,
        retry_queue_ref: None,
        parent_action_required: false,
    });
    assert_eq!(
        synced_missing_signature,
        Err(
            ParentOwnedSyncExportDerivationError::SuccessfulSyncRequiresCursorBatchChecksumAndSignature
        )
    );

    let corrupt_synced = derive_parent_owned_sync_state_row(ParentOwnedSyncStateInput {
        sync_state: contracts::ParentOwnedSyncState::Synced,
        provider_status_ref: contracts::ParentOwnedSyncStatusRef::parse("provider-status-manual")
            .assume_ok(),
        cursor_ref: contracts::ParentOwnedSyncCursorRef::parse("cursor-corrupt"),
        batch_ref: contracts::ParentOwnedSyncBatchRef::parse("batch-corrupt"),
        manifest_integrity_state: contracts::ParentOwnedSyncManifestIntegrityState::Corrupt,
        manifest_checksum_ref: contracts::ParentOwnedSyncChecksumRef::parse("checksum-corrupt"),
        manifest_signature_ref: contracts::ParentOwnedSyncSignatureRef::parse("signature-corrupt"),
        last_successful_sync_at: contracts::ParentTimestamp::parse("2026-06-28T18:40:00.000Z"),
        conflict_ref: None,
        retry_queue_ref: None,
        parent_action_required: true,
    });
    assert_eq!(
        corrupt_synced,
        Err(ParentOwnedSyncExportDerivationError::CorruptManifestCannotClaimSynced)
    );

    let conflict_missing_ref = derive_parent_owned_sync_state_row(ParentOwnedSyncStateInput {
        sync_state: contracts::ParentOwnedSyncState::Conflict,
        provider_status_ref: contracts::ParentOwnedSyncStatusRef::parse("provider-status-conflict")
            .assume_ok(),
        cursor_ref: contracts::ParentOwnedSyncCursorRef::parse("cursor-conflict"),
        batch_ref: contracts::ParentOwnedSyncBatchRef::parse("batch-conflict"),
        manifest_integrity_state: contracts::ParentOwnedSyncManifestIntegrityState::Verified,
        manifest_checksum_ref: contracts::ParentOwnedSyncChecksumRef::parse("checksum-conflict"),
        manifest_signature_ref: contracts::ParentOwnedSyncSignatureRef::parse("signature-conflict"),
        last_successful_sync_at: None,
        conflict_ref: None,
        retry_queue_ref: contracts::ParentOwnedSyncPolicyRef::parse("retry-conflict"),
        parent_action_required: true,
    });
    assert_eq!(
        conflict_missing_ref,
        Err(ParentOwnedSyncExportDerivationError::ConflictStateRequiresConflictRef)
    );
}

#[test]
fn tombstone_propagation_stays_separate_from_sync_success() {
    let propagated = derive_parent_owned_sync_tombstone_row(ParentOwnedSyncTombstoneInput {
        tombstone_ref: contracts::ParentOwnedSyncTombstoneRef::parse("tombstone-propagated")
            .assume_ok(),
        data_class: contracts::ParentOwnedSyncExportDataClass::GeneratedSummary,
        propagation_state: contracts::ParentOwnedSyncTombstonePropagationState::Propagated,
        delete_request_ref: contracts::ParentOwnedSyncDeleteRequestRef::parse("delete-summary"),
        provider_status_ref: contracts::ParentOwnedSyncStatusRef::parse("provider-status-ready")
            .assume_ok(),
        last_propagated_at: contracts::ParentTimestamp::parse("2026-06-28T18:40:00.000Z"),
        blocked_reason_ref: None,
    })
    .assume_ok();
    assert_eq!(
        propagated.propagation_state,
        contracts::ParentOwnedSyncTombstonePropagationState::Propagated
    );

    let blocked_missing_reason =
        derive_parent_owned_sync_tombstone_row(ParentOwnedSyncTombstoneInput {
            tombstone_ref: contracts::ParentOwnedSyncTombstoneRef::parse("tombstone-blocked")
                .assume_ok(),
            data_class: contracts::ParentOwnedSyncExportDataClass::NotificationHistory,
            propagation_state: contracts::ParentOwnedSyncTombstonePropagationState::Blocked,
            delete_request_ref: contracts::ParentOwnedSyncDeleteRequestRef::parse(
                "delete-notification",
            ),
            provider_status_ref: contracts::ParentOwnedSyncStatusRef::parse(
                "provider-status-folder",
            )
            .assume_ok(),
            last_propagated_at: None,
            blocked_reason_ref: None,
        });
    assert_eq!(
        blocked_missing_reason,
        Err(ParentOwnedSyncExportDerivationError::TombstoneBlockedRequiresReason)
    );
}

#[test]
fn proof_builder_keeps_non_claims_and_no_default_ocentra_custody_truth() {
    let sample = contracts::sample_parent_owned_sync_export_contract_proof();
    let built = build_parent_owned_sync_export_proof(
        &sample.manifest,
        sample
            .provider_statuses
            .iter()
            .cloned()
            .map(|row| ParentOwnedSyncProviderStatusInput {
                provider_id: row.provider_id,
                provider_mode: row.provider_mode,
                provider_status: row.provider_status,
                destination_ownership: row.destination_ownership,
                account_ref: row.account_ref,
                folder_ref: row.folder_ref,
                status_ref: row.status_ref,
                revocation_ref: row.revocation_ref,
                disconnect_visibility_state: row.disconnect_visibility_state,
                delete_visibility_state: row.delete_visibility_state,
                last_checked_at: row.last_checked_at,
            })
            .collect(),
        sample
            .sync_states
            .iter()
            .cloned()
            .map(|row| ParentOwnedSyncStateInput {
                sync_state: row.sync_state,
                provider_status_ref: row.provider_status_ref,
                cursor_ref: row.cursor_ref,
                batch_ref: row.batch_ref,
                manifest_integrity_state: row.manifest_integrity_state,
                manifest_checksum_ref: row.manifest_checksum_ref,
                manifest_signature_ref: row.manifest_signature_ref,
                last_successful_sync_at: row.last_successful_sync_at,
                conflict_ref: row.conflict_ref,
                retry_queue_ref: row.retry_queue_ref,
                parent_action_required: row.parent_action_required,
            })
            .collect(),
        sample
            .tombstones
            .iter()
            .cloned()
            .map(|row| ParentOwnedSyncTombstoneInput {
                tombstone_ref: row.tombstone_ref,
                data_class: row.data_class,
                propagation_state: row.propagation_state,
                delete_request_ref: row.delete_request_ref,
                provider_status_ref: row.provider_status_ref,
                last_propagated_at: row.last_propagated_at,
                blocked_reason_ref: row.blocked_reason_ref,
            })
            .collect(),
        contracts::ParentTimestamp::parse("2026-06-28T18:50:00.000Z").assume_ok(),
    )
    .assume_ok();

    assert_eq!(
        built.non_claims,
        contracts::required_parent_owned_sync_export_non_claims()
    );
    assert!(!built.transfer_runtime_claimed);
    assert!(!built.connector_o_auth_claimed);
    assert!(!built.upload_runtime_claimed);
    assert!(!built.delete_runtime_claimed);
    assert!(!built.ocentra_hosted_child_evidence_stored);
    assert_eq!(built.provider_statuses.len(), 11);
    assert_eq!(built.sync_states.len(), 8);
    assert_eq!(built.tombstones.len(), 5);
}
