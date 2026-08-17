use ocentra_schema::parent_owned_sync_export as sync_contracts;
use ocentra_schema::parent_storage_settings_apply_flow as contracts;

#[path = "parent_storage_settings_apply_flow_actions.rs"]
mod parent_storage_settings_apply_flow_actions;
#[path = "parent_storage_settings_apply_flow_apply.rs"]
mod parent_storage_settings_apply_flow_apply;
#[path = "parent_storage_settings_apply_flow_card.rs"]
mod parent_storage_settings_apply_flow_card;
#[path = "parent_storage_settings_apply_flow_intent_digest.rs"]
mod parent_storage_settings_apply_flow_intent_digest;
#[path = "parent_storage_settings_apply_flow_preview.rs"]
mod parent_storage_settings_apply_flow_preview;
#[path = "parent_storage_settings_apply_flow_proof.rs"]
mod parent_storage_settings_apply_flow_proof;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParentStorageModeCardInput {
    pub row_id: contracts::ParentStorageSettingsRowId,
    pub provider_mode: sync_contracts::ParentOwnedSyncProviderMode,
    pub provider_status: sync_contracts::ParentOwnedSyncProviderStatus,
    pub sync_state: sync_contracts::ParentOwnedSyncState,
    pub encryption_status: contracts::ParentStorageEncryptionStatus,
    pub key_status: contracts::ParentStorageKeyStatus,
    pub last_success_at: Option<contracts::ParentStorageTimestamp>,
    pub last_failure_at: Option<contracts::ParentStorageTimestamp>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParentStorageRestorePreviewInput {
    pub preview_id: contracts::ParentStoragePreviewId,
    pub household_ref: contracts::ParentStorageHouseholdRef,
    pub preview_state: contracts::ParentStoragePreviewState,
    pub created_at: contracts::ParentStorageTimestamp,
    pub product_version: String,
    pub schema_version: String,
    pub household_match: bool,
    pub device_match: bool,
    pub data_classes: Vec<sync_contracts::ParentOwnedSyncExportDataClass>,
    pub conflicts: Vec<String>,
    pub rejected_sections: Vec<sync_contracts::ParentOwnedSyncExportDataClass>,
    pub partial_restore: bool,
    pub manual_required_note: Option<String>,
}

/// Contract-only apply intent. It contains no authority, confirmation state,
/// executor result, or provider-side effect claim.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParentStorageApplyDecisionInput {
    pub apply_id: contracts::ParentStorageApplyId,
    pub will_change: Vec<sync_contracts::ParentOwnedSyncExportDataClass>,
    pub will_not_change: Vec<sync_contracts::ParentOwnedSyncExportDataClass>,
    pub preserved_tombstones: Vec<sync_contracts::ParentOwnedSyncExportDataClass>,
    pub manual_review_required: Vec<String>,
    pub manual_required_note: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParentStorageDeleteActionInput {
    pub action_id: contracts::ParentStorageActionId,
    pub action_kind: contracts::ParentStorageDeleteActionKind,
    pub state: sync_contracts::ParentOwnedSyncDeleteVisibilityState,
    pub notes: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParentStorageDisconnectInput {
    pub action_id: contracts::ParentStorageActionId,
    pub state: sync_contracts::ParentOwnedSyncDisconnectVisibilityState,
    pub notes: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParentStorageSettingsApplyFlowError {
    ManualRequiredMustStayVisible,
    DisabledModeMustStayDisabled,
    RestorePreviewMustRequireConfirmation,
    PartialRestoreMustNameRejectedSections,
    WrongHouseholdPreviewMustNotMatch,
    WrongDevicePreviewMustNotMatch,
    ApplyCannotProceedWithoutPreview,
    ApplyIntentDigestUnavailable,
    DisconnectCannotDeleteProviderData,
    DeleteActionMustStaySeparateFromDisconnect,
    DuplicateDeleteActionKind(contracts::ParentStorageDeleteActionKind),
    DeleteActionCoverageIncomplete,
    InvalidContractVersion,
}

pub fn derive_parent_storage_mode_card(
    input: ParentStorageModeCardInput,
) -> Result<contracts::ParentStorageModeCard, ParentStorageSettingsApplyFlowError> {
    parent_storage_settings_apply_flow_card::derive_parent_storage_mode_card(input)
}

pub fn derive_parent_storage_restore_preview(
    input: ParentStorageRestorePreviewInput,
) -> Result<contracts::ParentStorageRestorePreview, ParentStorageSettingsApplyFlowError> {
    parent_storage_settings_apply_flow_preview::derive_parent_storage_restore_preview(input)
}

/// Derive confirmation readiness only. Terminal Applied/Partial states require
/// a future digest-bound household authority and post-side-effect executor
/// receipt; neither can be supplied through this public contract boundary.
pub fn derive_parent_storage_apply_decision(
    preview: &contracts::ParentStorageRestorePreview,
    input: ParentStorageApplyDecisionInput,
) -> Result<contracts::ParentStorageApplyDecision, ParentStorageSettingsApplyFlowError> {
    parent_storage_settings_apply_flow_apply::derive_parent_storage_apply_decision(preview, input)
}

pub fn derive_parent_storage_delete_action_row(
    input: ParentStorageDeleteActionInput,
) -> Result<contracts::ParentStorageDeleteActionRow, ParentStorageSettingsApplyFlowError> {
    parent_storage_settings_apply_flow_actions::derive_parent_storage_delete_action_row(input)
}

pub fn derive_parent_storage_disconnect_row(
    input: ParentStorageDisconnectInput,
) -> Result<contracts::ParentStorageDisconnectRow, ParentStorageSettingsApplyFlowError> {
    parent_storage_settings_apply_flow_actions::derive_parent_storage_disconnect_row(input)
}

pub fn build_parent_storage_settings_apply_flow_proof(
    mode_card_input: ParentStorageModeCardInput,
    preview_input: ParentStorageRestorePreviewInput,
    apply_input: ParentStorageApplyDecisionInput,
    delete_action_inputs: Vec<ParentStorageDeleteActionInput>,
    disconnect_input: ParentStorageDisconnectInput,
    updated_at: contracts::ParentStorageTimestamp,
) -> Result<
    contracts::ParentStorageSettingsApplyFlowContractProof,
    ParentStorageSettingsApplyFlowError,
> {
    parent_storage_settings_apply_flow_proof::build_parent_storage_settings_apply_flow_proof(
        mode_card_input,
        preview_input,
        apply_input,
        delete_action_inputs,
        disconnect_input,
        updated_at,
    )
}
