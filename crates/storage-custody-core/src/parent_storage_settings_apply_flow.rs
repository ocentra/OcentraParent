use std::collections::BTreeSet;

use ocentra_schema::parent_owned_sync_export as sync_contracts;
use ocentra_schema::parent_storage_settings_apply_flow as contracts;

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParentStorageApplyDecisionInput {
    pub apply_id: contracts::ParentStorageApplyId,
    pub apply_state: contracts::ParentStorageApplyState,
    pub will_change: Vec<sync_contracts::ParentOwnedSyncExportDataClass>,
    pub will_not_change: Vec<sync_contracts::ParentOwnedSyncExportDataClass>,
    pub preserved_tombstones: Vec<sync_contracts::ParentOwnedSyncExportDataClass>,
    pub manual_review_required: Vec<String>,
    pub rollback_available: bool,
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
    ApplyMustStayConfirmationGated,
    ApplyCannotProceedWithoutPreview,
    DisconnectCannotDeleteProviderData,
    DeleteActionMustStaySeparateFromDisconnect,
    DuplicateDeleteActionKind(contracts::ParentStorageDeleteActionKind),
    DeleteActionCoverageIncomplete,
}

fn option_or_unreachable<T>(value: Option<T>, context: &str) -> T {
    match value {
        Some(value) => value,
        None => unreachable!("{context}"),
    }
}

pub fn derive_parent_storage_mode_card(
    input: ParentStorageModeCardInput,
) -> Result<contracts::ParentStorageModeCard, ParentStorageSettingsApplyFlowError> {
    let current_mode_label = match input.provider_status {
        sync_contracts::ParentOwnedSyncProviderStatus::Disabled
        | sync_contracts::ParentOwnedSyncProviderStatus::NotConfigured => {
            contracts::ParentStorageModeLabel::Disabled
        }
        sync_contracts::ParentOwnedSyncProviderStatus::Disconnected => {
            contracts::ParentStorageModeLabel::ProviderDisconnected
        }
        sync_contracts::ParentOwnedSyncProviderStatus::WrongAccount
        | sync_contracts::ParentOwnedSyncProviderStatus::FolderUnavailable
        | sync_contracts::ParentOwnedSyncProviderStatus::PartialUpload
        | sync_contracts::ParentOwnedSyncProviderStatus::Revoked => {
            contracts::ParentStorageModeLabel::ProviderError
        }
        sync_contracts::ParentOwnedSyncProviderStatus::ManualRequired => {
            contracts::ParentStorageModeLabel::ManualRequired
        }
        sync_contracts::ParentOwnedSyncProviderStatus::Ready => {
            if input.provider_mode == sync_contracts::ParentOwnedSyncProviderMode::LocalFolder {
                contracts::ParentStorageModeLabel::LocalPlusEncryptedBackup
            } else {
                contracts::ParentStorageModeLabel::LocalPlusEncryptedProviderSync
            }
        }
    };

    let ui_state = parent_storage_ui_state(input.provider_status, input.sync_state);

    let manual_required_visible = current_mode_label
        == contracts::ParentStorageModeLabel::ManualRequired
        || ui_state == contracts::ParentStorageUiState::ManualRequired
        || input.key_status == contracts::ParentStorageKeyStatus::ManualRequired;
    validate_parent_storage_mode_card(
        current_mode_label,
        manual_required_visible,
        input.provider_status,
    )?;

    Ok(contracts::ParentStorageModeCard {
        row_id: input.row_id,
        current_mode_label,
        ui_state,
        provider_mode: input.provider_mode,
        provider_status: input.provider_status,
        sync_state: input.sync_state,
        encryption_status: input.encryption_status,
        key_status: input.key_status,
        manual_required_visible,
        disconnect_visible: input.provider_status
            == sync_contracts::ParentOwnedSyncProviderStatus::Disconnected,
        delete_visible: input.provider_status
            != sync_contracts::ParentOwnedSyncProviderStatus::Disabled
            && input.provider_status
                != sync_contracts::ParentOwnedSyncProviderStatus::NotConfigured,
        restore_preview_available: true,
        apply_back_available: input.provider_status
            == sync_contracts::ParentOwnedSyncProviderStatus::Ready
            && input.sync_state != sync_contracts::ParentOwnedSyncState::ManualRequired,
        last_success_at: input.last_success_at,
        last_failure_at: input.last_failure_at,
        summary: summary_for_mode(current_mode_label, ui_state).to_string(),
    })
}

fn parent_storage_ui_state(
    provider_status: sync_contracts::ParentOwnedSyncProviderStatus,
    sync_state: sync_contracts::ParentOwnedSyncState,
) -> contracts::ParentStorageUiState {
    match provider_status {
        sync_contracts::ParentOwnedSyncProviderStatus::NotConfigured => {
            contracts::ParentStorageUiState::ProviderNotConfigured
        }
        sync_contracts::ParentOwnedSyncProviderStatus::ManualRequired => {
            contracts::ParentStorageUiState::ManualRequired
        }
        sync_contracts::ParentOwnedSyncProviderStatus::Revoked => {
            contracts::ParentStorageUiState::ProviderRevoked
        }
        sync_contracts::ParentOwnedSyncProviderStatus::WrongAccount => {
            contracts::ParentStorageUiState::ProviderAuthExpired
        }
        sync_contracts::ParentOwnedSyncProviderStatus::FolderUnavailable => {
            contracts::ParentStorageUiState::ProviderPermissionMissing
        }
        sync_contracts::ParentOwnedSyncProviderStatus::PartialUpload => {
            contracts::ParentStorageUiState::ProviderQuotaExceeded
        }
        sync_contracts::ParentOwnedSyncProviderStatus::Disconnected => {
            contracts::ParentStorageUiState::RemoteDisabled
        }
        sync_contracts::ParentOwnedSyncProviderStatus::Disabled => {
            contracts::ParentStorageUiState::SyncDisabled
        }
        sync_contracts::ParentOwnedSyncProviderStatus::Ready => match sync_state {
            sync_contracts::ParentOwnedSyncState::OfflineRetryPending => {
                contracts::ParentStorageUiState::OfflineQueued
            }
            sync_contracts::ParentOwnedSyncState::ManualRequired => {
                contracts::ParentStorageUiState::ManualRequired
            }
            _ => contracts::ParentStorageUiState::Ready,
        },
    }
}

fn validate_parent_storage_mode_card(
    current_mode_label: contracts::ParentStorageModeLabel,
    manual_required_visible: bool,
    provider_status: sync_contracts::ParentOwnedSyncProviderStatus,
) -> Result<(), ParentStorageSettingsApplyFlowError> {
    if current_mode_label == contracts::ParentStorageModeLabel::ManualRequired
        && !manual_required_visible
    {
        return Err(ParentStorageSettingsApplyFlowError::ManualRequiredMustStayVisible);
    }

    if current_mode_label == contracts::ParentStorageModeLabel::Disabled
        && provider_status != sync_contracts::ParentOwnedSyncProviderStatus::Disabled
        && provider_status != sync_contracts::ParentOwnedSyncProviderStatus::NotConfigured
    {
        return Err(ParentStorageSettingsApplyFlowError::DisabledModeMustStayDisabled);
    }

    Ok(())
}

pub fn derive_parent_storage_restore_preview(
    input: ParentStorageRestorePreviewInput,
) -> Result<contracts::ParentStorageRestorePreview, ParentStorageSettingsApplyFlowError> {
    if !matches!(
        input.preview_state,
        contracts::ParentStoragePreviewState::WrongHousehold
            | contracts::ParentStoragePreviewState::WrongKey
            | contracts::ParentStoragePreviewState::SchemaUnsupported
            | contracts::ParentStoragePreviewState::BundleCorrupt
            | contracts::ParentStoragePreviewState::TombstoneConflict
            | contracts::ParentStoragePreviewState::PartialRestore
            | contracts::ParentStoragePreviewState::ImportPreviewPassed
            | contracts::ParentStoragePreviewState::ManualRequired
    ) {
        return Err(ParentStorageSettingsApplyFlowError::RestorePreviewMustRequireConfirmation);
    }

    if input.preview_state == contracts::ParentStoragePreviewState::PartialRestore
        && input.rejected_sections.is_empty()
    {
        return Err(ParentStorageSettingsApplyFlowError::PartialRestoreMustNameRejectedSections);
    }
    if input.preview_state == contracts::ParentStoragePreviewState::WrongHousehold
        && input.household_match
    {
        return Err(ParentStorageSettingsApplyFlowError::WrongHouseholdPreviewMustNotMatch);
    }
    if input.preview_state == contracts::ParentStoragePreviewState::WrongKey && input.device_match {
        // wrong key can still be device-matched; don't reject
    }
    if input.preview_state == contracts::ParentStoragePreviewState::ManualRequired
        && input.manual_required_note.is_none()
    {
        return Err(ParentStorageSettingsApplyFlowError::ManualRequiredMustStayVisible);
    }
    if input.preview_state == contracts::ParentStoragePreviewState::WrongHousehold
        && input.device_match
        && input.household_match
    {
        return Err(ParentStorageSettingsApplyFlowError::WrongHouseholdPreviewMustNotMatch);
    }
    if input.preview_state == contracts::ParentStoragePreviewState::TombstoneConflict
        && input.rejected_sections.is_empty()
    {
        return Err(ParentStorageSettingsApplyFlowError::PartialRestoreMustNameRejectedSections);
    }

    Ok(contracts::ParentStorageRestorePreview {
        preview_id: input.preview_id,
        preview_state: input.preview_state,
        created_at: input.created_at,
        product_version: input.product_version,
        schema_version: input.schema_version,
        household_match: input.household_match,
        device_match: input.device_match,
        data_classes: input.data_classes,
        conflicts: input.conflicts,
        rejected_sections: input.rejected_sections,
        partial_restore: input.partial_restore,
        confirmation_required: true,
        local_truth_authoritative: true,
        tombstones_preserved: true,
        manual_required_note: input.manual_required_note,
    })
}

pub fn derive_parent_storage_apply_decision(
    preview: &contracts::ParentStorageRestorePreview,
    input: ParentStorageApplyDecisionInput,
) -> Result<contracts::ParentStorageApplyDecision, ParentStorageSettingsApplyFlowError> {
    match input.apply_state {
        contracts::ParentStorageApplyState::ApplyRequiresConfirmation
        | contracts::ParentStorageApplyState::ApplyPending
        | contracts::ParentStorageApplyState::Applied
        | contracts::ParentStorageApplyState::Partial
        | contracts::ParentStorageApplyState::RollbackManualRequired
        | contracts::ParentStorageApplyState::BlockedManualRequired
        | contracts::ParentStorageApplyState::NotStarted => {}
    }

    if preview.confirmation_required
        && matches!(
            input.apply_state,
            contracts::ParentStorageApplyState::Applied
                | contracts::ParentStorageApplyState::Partial
        )
    {
        return Err(ParentStorageSettingsApplyFlowError::ApplyMustStayConfirmationGated);
    }
    if input.apply_state == contracts::ParentStorageApplyState::ApplyRequiresConfirmation
        && !preview.confirmation_required
    {
        return Err(ParentStorageSettingsApplyFlowError::ApplyCannotProceedWithoutPreview);
    }
    if matches!(
        input.apply_state,
        contracts::ParentStorageApplyState::BlockedManualRequired
            | contracts::ParentStorageApplyState::RollbackManualRequired
    ) && input.manual_required_note.is_none()
    {
        return Err(ParentStorageSettingsApplyFlowError::ManualRequiredMustStayVisible);
    }

    Ok(contracts::ParentStorageApplyDecision {
        apply_id: input.apply_id,
        apply_state: input.apply_state,
        confirmation_required: true,
        will_change: input.will_change,
        will_not_change: input.will_not_change,
        preserved_tombstones: input.preserved_tombstones,
        manual_review_required: input.manual_review_required,
        rollback_available: input.rollback_available,
        manual_required_note: input.manual_required_note,
    })
}

pub fn derive_parent_storage_delete_action_row(
    input: ParentStorageDeleteActionInput,
) -> Result<contracts::ParentStorageDeleteActionRow, ParentStorageSettingsApplyFlowError> {
    Ok(contracts::ParentStorageDeleteActionRow {
        action_id: input.action_id,
        action_kind: input.action_kind,
        state: input.state,
        separate_from_disconnect: true,
        proof_required: true,
        notes: input.notes,
    })
}

pub fn derive_parent_storage_disconnect_row(
    input: ParentStorageDisconnectInput,
) -> Result<contracts::ParentStorageDisconnectRow, ParentStorageSettingsApplyFlowError> {
    if input.state == sync_contracts::ParentOwnedSyncDisconnectVisibilityState::ManualRequired
        && input.notes.trim().is_empty()
    {
        return Err(ParentStorageSettingsApplyFlowError::ManualRequiredMustStayVisible);
    }

    Ok(contracts::ParentStorageDisconnectRow {
        action_id: input.action_id,
        state: input.state,
        existing_files_may_remain: true,
        provider_delete_requested_separately: true,
        notes: input.notes,
    })
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
    let mode_card = derive_parent_storage_mode_card(mode_card_input)?;
    let preview = derive_parent_storage_restore_preview(preview_input)?;
    let apply_decision = derive_parent_storage_apply_decision(&preview, apply_input)?;
    let disconnect_action = derive_parent_storage_disconnect_row(disconnect_input)?;

    let mut seen_delete_kinds = BTreeSet::new();
    let mut delete_actions = Vec::new();
    for input in delete_action_inputs {
        if !seen_delete_kinds.insert(input.action_kind.as_str().to_owned()) {
            return Err(
                ParentStorageSettingsApplyFlowError::DuplicateDeleteActionKind(input.action_kind),
            );
        }
        delete_actions.push(derive_parent_storage_delete_action_row(input)?);
    }

    if seen_delete_kinds.len() != contracts::required_parent_storage_delete_action_kinds().len() {
        return Err(ParentStorageSettingsApplyFlowError::DeleteActionCoverageIncomplete);
    }

    if disconnect_action.provider_delete_requested_separately
        && delete_actions
            .iter()
            .any(|row| !row.separate_from_disconnect)
    {
        return Err(
            ParentStorageSettingsApplyFlowError::DeleteActionMustStaySeparateFromDisconnect,
        );
    }

    Ok(contracts::ParentStorageSettingsApplyFlowContractProof {
        schema_version: contracts::PARENT_STORAGE_SETTINGS_APPLY_FLOW_SCHEMA_VERSION.to_string(),
        contract_version: option_or_unreachable(
            contracts::ParentStorageContractVersion::parse("v0.6"),
            "contract version",
        ),
        mode_card,
        restore_preview: preview,
        apply_decision,
        delete_actions,
        disconnect_action,
        claim_safe_copy: contracts::sample_parent_storage_settings_apply_flow_contract_proof()
            .claim_safe_copy,
        no_claims: contracts::required_parent_storage_no_claims(),
        updated_at,
    })
}

fn summary_for_mode(
    mode: contracts::ParentStorageModeLabel,
    ui_state: contracts::ParentStorageUiState,
) -> &'static str {
    match (mode, ui_state) {
        (contracts::ParentStorageModeLabel::LocalOnly, _) => {
            "Local-only storage remains the current custody path."
        }
        (contracts::ParentStorageModeLabel::LocalPlusEncryptedBackup, _) => {
            "Parent-owned local backup remains explicit and encrypted."
        }
        (contracts::ParentStorageModeLabel::LocalPlusEncryptedProviderSync, _) => {
            "Provider sync stays encrypted and parent-owned."
        }
        (contracts::ParentStorageModeLabel::ProviderDisconnected, _) => {
            "Provider is disconnected; existing files may remain until separate delete proof succeeds."
        }
        (contracts::ParentStorageModeLabel::ProviderError, _) => {
            "Provider failure is explicit; no success-looking state is shown."
        }
        (contracts::ParentStorageModeLabel::ManualRequired, _) => {
            "Manual proof is required before a safe apply or delete step proceeds."
        }
        (contracts::ParentStorageModeLabel::Disabled, contracts::ParentStorageUiState::SyncDisabled) => {
            "Provider sync is intentionally disabled."
        }
        (contracts::ParentStorageModeLabel::Disabled, _) => "Provider storage is not configured.",
    }
}
