use ocentra_schema::parent_owned_sync_export as contracts;

#[path = "parent_owned_sync_export_proof.rs"]
mod parent_owned_sync_export_proof;
#[path = "parent_owned_sync_export_provider.rs"]
mod parent_owned_sync_export_provider;
#[path = "parent_owned_sync_export_state.rs"]
mod parent_owned_sync_export_state;
#[path = "parent_owned_sync_export_tombstone.rs"]
mod parent_owned_sync_export_tombstone;

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
    InvalidContractVersion,
}

pub fn derive_parent_owned_sync_provider_status_row(
    input: ParentOwnedSyncProviderStatusInput,
) -> Result<contracts::ParentOwnedSyncProviderStatusRow, ParentOwnedSyncExportDerivationError> {
    parent_owned_sync_export_provider::derive_parent_owned_sync_provider_status_row(input)
}

pub fn derive_parent_owned_sync_state_row(
    input: ParentOwnedSyncStateInput,
) -> Result<contracts::ParentOwnedSyncStateRow, ParentOwnedSyncExportDerivationError> {
    parent_owned_sync_export_state::derive_parent_owned_sync_state_row(input)
}

pub fn derive_parent_owned_sync_tombstone_row(
    input: ParentOwnedSyncTombstoneInput,
) -> Result<contracts::ParentOwnedSyncTombstoneRow, ParentOwnedSyncExportDerivationError> {
    parent_owned_sync_export_tombstone::derive_parent_owned_sync_tombstone_row(input)
}

pub fn build_parent_owned_sync_export_proof(
    manifest: &contracts::ParentOwnedSyncExportManifest,
    provider_inputs: Vec<ParentOwnedSyncProviderStatusInput>,
    sync_inputs: Vec<ParentOwnedSyncStateInput>,
    tombstone_inputs: Vec<ParentOwnedSyncTombstoneInput>,
    updated_at: contracts::ParentTimestamp,
) -> Result<contracts::ParentOwnedSyncExportContractProof, ParentOwnedSyncExportDerivationError> {
    parent_owned_sync_export_proof::build_parent_owned_sync_export_proof(
        manifest,
        provider_inputs,
        sync_inputs,
        tombstone_inputs,
        updated_at,
    )
}
