use std::collections::BTreeSet;

use ocentra_schema::parent_owned_sync_export as contracts;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParentOwnedSyncProviderStatusInput {
    pub provider_id: contracts::ParentOwnedSyncProviderId,
    pub provider_mode: contracts::ParentOwnedSyncProviderMode,
    pub provider_status: contracts::ParentOwnedSyncProviderStatus,
    pub destination_ownership: contracts::ParentOwnedSyncExportDestinationOwnership,
    pub account_ref: Option<contracts::ParentOwnedSyncProviderRef>,
    pub folder_ref: Option<contracts::ParentOwnedSyncProviderRef>,
    pub status_ref: contracts::ParentOwnedSyncStatusRef,
    pub revocation_ref: Option<contracts::ParentOwnedSyncProviderRef>,
    pub disconnect_visibility_state: contracts::ParentOwnedSyncDisconnectVisibilityState,
    pub delete_visibility_state: contracts::ParentOwnedSyncDeleteVisibilityState,
    pub last_checked_at: contracts::ParentTimestamp,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParentOwnedSyncStateInput {
    pub sync_state: contracts::ParentOwnedSyncState,
    pub provider_status_ref: contracts::ParentOwnedSyncStatusRef,
    pub cursor_ref: Option<contracts::ParentOwnedSyncCursorRef>,
    pub batch_ref: Option<contracts::ParentOwnedSyncBatchRef>,
    pub manifest_integrity_state: contracts::ParentOwnedSyncManifestIntegrityState,
    pub manifest_checksum_ref: Option<contracts::ParentOwnedSyncChecksumRef>,
    pub manifest_signature_ref: Option<contracts::ParentOwnedSyncSignatureRef>,
    pub last_successful_sync_at: Option<contracts::ParentTimestamp>,
    pub conflict_ref: Option<contracts::ParentOwnedSyncConflictRef>,
    pub retry_queue_ref: Option<contracts::ParentOwnedSyncPolicyRef>,
    pub parent_action_required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParentOwnedSyncTombstoneInput {
    pub tombstone_ref: contracts::ParentOwnedSyncTombstoneRef,
    pub data_class: contracts::ParentOwnedSyncExportDataClass,
    pub propagation_state: contracts::ParentOwnedSyncTombstonePropagationState,
    pub delete_request_ref: Option<contracts::ParentOwnedSyncDeleteRequestRef>,
    pub provider_status_ref: contracts::ParentOwnedSyncStatusRef,
    pub last_propagated_at: Option<contracts::ParentTimestamp>,
    pub blocked_reason_ref: Option<contracts::ParentOwnedSyncPolicyRef>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParentOwnedSyncExportDerivationError {
    ReadyProviderMissingLocationRefs,
    RevokedProviderMissingRevocationRef,
    DisabledProviderMustNotKeepRefs,
    NotConfiguredProviderMustNotKeepRefs,
    DisconnectedProviderMustBeVisible,
    ManualProviderStateMustStayVisible,
    OcentraHostedCustodyForbidden,
    SuccessfulSyncRequiresCursorBatchChecksumAndSignature,
    CorruptManifestCannotClaimSynced,
    ConflictStateRequiresConflictRef,
    RetryStateRequiresQueueRef,
    NotStartedStateCannotHaveOperationalRefs,
    TombstoneDeleteRequestMissing,
    TombstoneBlockedRequiresReason,
    TombstonePropagatedRequiresTimestamp,
    DuplicateProviderStatusRef,
    DuplicateTombstoneRef,
}

fn option_or_unreachable<T>(value: Option<T>, context: &str) -> T {
    match value {
        Some(value) => value,
        None => unreachable!("{context}"),
    }
}

pub fn derive_parent_owned_sync_provider_status_row(
    input: ParentOwnedSyncProviderStatusInput,
) -> Result<contracts::ParentOwnedSyncProviderStatusRow, ParentOwnedSyncExportDerivationError> {
    validate_parent_owned_sync_provider_status(&input)?;

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

fn validate_parent_owned_sync_provider_status(
    input: &ParentOwnedSyncProviderStatusInput,
) -> Result<(), ParentOwnedSyncExportDerivationError> {
    if input.destination_ownership
        == contracts::ParentOwnedSyncExportDestinationOwnership::OcentraHostedNonActivityMetadata
    {
        return Err(ParentOwnedSyncExportDerivationError::OcentraHostedCustodyForbidden);
    }

    match input.provider_status {
        contracts::ParentOwnedSyncProviderStatus::Ready => {
            if input.account_ref.is_none() || input.folder_ref.is_none() {
                return Err(ParentOwnedSyncExportDerivationError::ReadyProviderMissingLocationRefs);
            }
        }
        contracts::ParentOwnedSyncProviderStatus::Revoked => {
            if input.revocation_ref.is_none() {
                return Err(
                    ParentOwnedSyncExportDerivationError::RevokedProviderMissingRevocationRef,
                );
            }
        }
        contracts::ParentOwnedSyncProviderStatus::Disabled => {
            if input.account_ref.is_some()
                || input.folder_ref.is_some()
                || input.revocation_ref.is_some()
            {
                return Err(ParentOwnedSyncExportDerivationError::DisabledProviderMustNotKeepRefs);
            }
        }
        contracts::ParentOwnedSyncProviderStatus::NotConfigured => {
            if input.account_ref.is_some()
                || input.folder_ref.is_some()
                || input.revocation_ref.is_some()
            {
                return Err(
                    ParentOwnedSyncExportDerivationError::NotConfiguredProviderMustNotKeepRefs,
                );
            }
        }
        contracts::ParentOwnedSyncProviderStatus::Disconnected => {
            if input.disconnect_visibility_state
                != contracts::ParentOwnedSyncDisconnectVisibilityState::DisconnectVisible
            {
                return Err(
                    ParentOwnedSyncExportDerivationError::DisconnectedProviderMustBeVisible,
                );
            }
        }
        contracts::ParentOwnedSyncProviderStatus::ManualRequired => {
            if input.disconnect_visibility_state
                != contracts::ParentOwnedSyncDisconnectVisibilityState::ManualRequired
                && input.delete_visibility_state
                    != contracts::ParentOwnedSyncDeleteVisibilityState::ManualRequired
            {
                return Err(
                    ParentOwnedSyncExportDerivationError::ManualProviderStateMustStayVisible,
                );
            }
        }
        contracts::ParentOwnedSyncProviderStatus::WrongAccount
        | contracts::ParentOwnedSyncProviderStatus::FolderUnavailable
        | contracts::ParentOwnedSyncProviderStatus::PartialUpload => {}
    }

    Ok(())
}

pub fn derive_parent_owned_sync_state_row(
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

pub fn derive_parent_owned_sync_tombstone_row(
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

pub fn build_parent_owned_sync_export_proof(
    manifest: &contracts::ParentOwnedSyncExportManifest,
    provider_inputs: Vec<ParentOwnedSyncProviderStatusInput>,
    sync_inputs: Vec<ParentOwnedSyncStateInput>,
    tombstone_inputs: Vec<ParentOwnedSyncTombstoneInput>,
    updated_at: contracts::ParentTimestamp,
) -> Result<contracts::ParentOwnedSyncExportContractProof, ParentOwnedSyncExportDerivationError> {
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
        contract_version: option_or_unreachable(
            contracts::ParentContractSchemaVersion::parse("v0.6"),
            "contract version",
        ),
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
