use super::constants::*;
use super::enums::{
    ParentStorageApplyState, ParentStorageCopyKey, ParentStorageDeleteActionKind,
    ParentStorageEncryptionStatus, ParentStorageKeyStatus, ParentStorageModeLabel,
    ParentStorageNoClaim, ParentStoragePreviewState, ParentStorageUiState,
};
use super::identifiers::{
    action_id, apply_id, contract_version, household_ref, owned_text, preview_id, row_id, timestamp,
};
use super::proof_types::{
    ParentStorageApplyDecision, ParentStorageClaimSafeCopyRow, ParentStorageDeleteActionRow,
    ParentStorageDisconnectRow, ParentStorageModeCard, ParentStorageRestorePreview,
    ParentStorageSettingsApplyFlowContractProof,
};
use crate::parent_owned_sync_export::{
    ParentOwnedSyncDeleteVisibilityState, ParentOwnedSyncDisconnectVisibilityState,
    ParentOwnedSyncExportDataClass, ParentOwnedSyncProviderMode, ParentOwnedSyncProviderStatus,
    ParentOwnedSyncState,
};

pub(super) fn required_parent_storage_mode_labels() -> Vec<ParentStorageModeLabel> {
    vec![
        ParentStorageModeLabel::LocalOnly,
        ParentStorageModeLabel::LocalPlusEncryptedBackup,
        ParentStorageModeLabel::LocalPlusEncryptedProviderSync,
        ParentStorageModeLabel::ProviderDisconnected,
        ParentStorageModeLabel::ProviderError,
        ParentStorageModeLabel::ManualRequired,
        ParentStorageModeLabel::Disabled,
    ]
}

pub(super) fn required_parent_storage_delete_action_kinds() -> Vec<ParentStorageDeleteActionKind> {
    vec![
        ParentStorageDeleteActionKind::LocalChildEvidence,
        ParentStorageDeleteActionKind::ParentPortalCache,
        ParentStorageDeleteActionKind::GeneratedReport,
        ParentStorageDeleteActionKind::ProviderBackupCopy,
        ParentStorageDeleteActionKind::SupportBundle,
        ParentStorageDeleteActionKind::OcentraMetadata,
    ]
}

pub(super) fn required_parent_storage_copy_keys() -> Vec<ParentStorageCopyKey> {
    vec![
        ParentStorageCopyKey::CustodyBoundary,
        ParentStorageCopyKey::MetadataLeakage,
        ParentStorageCopyKey::SensitiveEncryptedBeforeUpload,
        ParentStorageCopyKey::LostKeyMayBeUnrecoverable,
        ParentStorageCopyKey::DisconnectDoesNotDelete,
        ParentStorageCopyKey::TombstonesMayBeRequired,
        ParentStorageCopyKey::BackupQueued,
        ParentStorageCopyKey::ProviderUploadPending,
        ParentStorageCopyKey::ProviderUploadFailed,
        ParentStorageCopyKey::ProviderUploadConfirmed,
        ParentStorageCopyKey::ImportPreviewPassed,
        ParentStorageCopyKey::ApplyRequiresConfirmation,
        ParentStorageCopyKey::DeletedLocallyProviderDeletePending,
        ParentStorageCopyKey::ProviderDisconnectedExistingFilesMayRemain,
        ParentStorageCopyKey::ManualProofRequired,
    ]
}

pub(super) fn required_parent_storage_no_claims() -> Vec<ParentStorageNoClaim> {
    vec![
        ParentStorageNoClaim::PortalImplementationReady,
        ParentStorageNoClaim::ProviderRuntimeReady,
        ParentStorageNoClaim::AutoApply,
        ParentStorageNoClaim::DisconnectDeletesProviderData,
        ParentStorageNoClaim::DeleteDisconnectCollapse,
        ParentStorageNoClaim::TsBusinessOwner,
        ParentStorageNoClaim::LanOwnership,
    ]
}

pub(super) fn parent_storage_settings_apply_flow_known_gaps() -> [&'static str; 4] {
    [
        PARENT_STORAGE_KNOWN_GAP_FINAL_PORTAL_RENDERING,
        PARENT_STORAGE_KNOWN_GAP_DESKTOP_HOST_WIRING,
        PARENT_STORAGE_KNOWN_GAP_PROVIDER_SDK_RUNTIME,
        PARENT_STORAGE_KNOWN_GAP_AUTOMATIC_PROVIDER_DELETE_OR_APPLY,
    ]
}

pub(super) fn sample_parent_storage_settings_apply_flow_contract_proof(
) -> ParentStorageSettingsApplyFlowContractProof {
    ParentStorageSettingsApplyFlowContractProof {
        schema_version: owned_text(PARENT_STORAGE_SETTINGS_APPLY_FLOW_SCHEMA_VERSION),
        contract_version: contract_version(PARENT_STORAGE_CONTRACT_VERSION_VALUE),
        mode_card: sample_mode_card(),
        restore_preview: sample_restore_preview(),
        apply_decision: sample_apply_decision(),
        delete_actions: sample_delete_actions(),
        disconnect_action: sample_disconnect_action(),
        claim_safe_copy: sample_claim_safe_copy(),
        no_claims: required_parent_storage_no_claims(),
        updated_at: timestamp(PARENT_STORAGE_UPDATED_AT_VALUE),
    }
}

fn sample_mode_card() -> ParentStorageModeCard {
    ParentStorageModeCard {
        row_id: row_id(PARENT_STORAGE_ROW_ID_VALUE),
        current_mode_label: ParentStorageModeLabel::ManualRequired,
        ui_state: ParentStorageUiState::ManualRequired,
        provider_mode: ParentOwnedSyncProviderMode::GoogleDrivePickerFile,
        provider_status: ParentOwnedSyncProviderStatus::ManualRequired,
        sync_state: ParentOwnedSyncState::ManualRequired,
        encryption_status: ParentStorageEncryptionStatus::EncryptedBeforeUpload,
        key_status: ParentStorageKeyStatus::ManualRequired,
        manual_required_visible: true,
        disconnect_visible: false,
        delete_visible: true,
        restore_preview_available: true,
        apply_back_available: false,
        last_success_at: Some(timestamp(PARENT_STORAGE_LAST_SUCCESS_AT_VALUE)),
        last_failure_at: Some(timestamp(PARENT_STORAGE_LAST_FAILURE_AT_VALUE)),
        summary: owned_text(PARENT_STORAGE_SUMMARY_MANUAL_PROOF_REQUIRED),
    }
}

fn sample_restore_preview() -> ParentStorageRestorePreview {
    ParentStorageRestorePreview {
        preview_id: preview_id(PARENT_STORAGE_RESTORE_PREVIEW_ID_VALUE),
        household_ref: household_ref(PARENT_STORAGE_HOUSEHOLD_REF_VALUE),
        preview_state: ParentStoragePreviewState::PartialRestore,
        created_at: timestamp(PARENT_STORAGE_RESTORE_PREVIEW_CREATED_AT_VALUE),
        product_version: owned_text(PARENT_STORAGE_PRODUCT_VERSION_VALUE),
        schema_version: owned_text(PARENT_STORAGE_EXPORT_SCHEMA_VERSION_VALUE),
        household_match: true,
        device_match: true,
        data_classes: vec![
            ParentOwnedSyncExportDataClass::EncryptedJournalSegment,
            ParentOwnedSyncExportDataClass::GeneratedSummary,
            ParentOwnedSyncExportDataClass::NotificationHistory,
        ],
        conflicts: vec![owned_text(
            PARENT_STORAGE_CONFLICT_NOTIFICATION_HISTORY_TOMBSTONE_PRESERVED,
        )],
        rejected_sections: vec![ParentOwnedSyncExportDataClass::NotificationHistory],
        partial_restore: true,
        confirmation_required: true,
        local_truth_authoritative: true,
        tombstones_preserved: true,
        manual_required_note: Some(owned_text(PARENT_STORAGE_MANUAL_REVIEW_REQUIRED_NOTE)),
    }
}

fn sample_apply_decision() -> ParentStorageApplyDecision {
    ParentStorageApplyDecision {
        apply_id: apply_id(PARENT_STORAGE_APPLY_DECISION_ID_VALUE),
        apply_state: ParentStorageApplyState::ApplyRequiresConfirmation,
        confirmation_required: true,
        will_change: vec![
            ParentOwnedSyncExportDataClass::EncryptedJournalSegment,
            ParentOwnedSyncExportDataClass::GeneratedSummary,
        ],
        will_not_change: vec![ParentOwnedSyncExportDataClass::NotificationHistory],
        preserved_tombstones: vec![ParentOwnedSyncExportDataClass::NotificationHistory],
        manual_review_required: vec![owned_text(
            PARENT_STORAGE_CONFLICT_NOTIFICATION_HISTORY_TOMBSTONE_CONFLICT,
        )],
        rollback_available: false,
        manual_required_note: None,
    }
}

fn sample_delete_actions() -> Vec<ParentStorageDeleteActionRow> {
    vec![
        delete_action(
            PARENT_STORAGE_DELETE_LOCAL_EVIDENCE_ACTION_ID,
            ParentStorageDeleteActionKind::LocalChildEvidence,
            ParentOwnedSyncDeleteVisibilityState::DeleteVisible,
            PARENT_STORAGE_DELETE_LOCAL_EVIDENCE_NOTE,
        ),
        delete_action(
            PARENT_STORAGE_DELETE_PARENT_CACHE_ACTION_ID,
            ParentStorageDeleteActionKind::ParentPortalCache,
            ParentOwnedSyncDeleteVisibilityState::DeleteVisible,
            PARENT_STORAGE_DELETE_PARENT_CACHE_NOTE,
        ),
        delete_action(
            PARENT_STORAGE_DELETE_GENERATED_REPORT_ACTION_ID,
            ParentStorageDeleteActionKind::GeneratedReport,
            ParentOwnedSyncDeleteVisibilityState::DeleteVisible,
            PARENT_STORAGE_DELETE_GENERATED_REPORT_NOTE,
        ),
        delete_action(
            PARENT_STORAGE_DELETE_PROVIDER_COPY_ACTION_ID,
            ParentStorageDeleteActionKind::ProviderBackupCopy,
            ParentOwnedSyncDeleteVisibilityState::ManualRequired,
            PARENT_STORAGE_DELETE_PROVIDER_COPY_NOTE,
        ),
        delete_action(
            PARENT_STORAGE_DELETE_SUPPORT_BUNDLE_ACTION_ID,
            ParentStorageDeleteActionKind::SupportBundle,
            ParentOwnedSyncDeleteVisibilityState::DeleteVisible,
            PARENT_STORAGE_DELETE_SUPPORT_BUNDLE_NOTE,
        ),
        delete_action(
            PARENT_STORAGE_DELETE_OCENTRA_METADATA_ACTION_ID,
            ParentStorageDeleteActionKind::OcentraMetadata,
            ParentOwnedSyncDeleteVisibilityState::DeleteVisible,
            PARENT_STORAGE_DELETE_OCENTRA_METADATA_NOTE,
        ),
    ]
}

fn sample_disconnect_action() -> ParentStorageDisconnectRow {
    ParentStorageDisconnectRow {
        action_id: action_id(PARENT_STORAGE_DISCONNECT_ACTION_ID),
        state: ParentOwnedSyncDisconnectVisibilityState::DisconnectVisible,
        existing_files_may_remain: true,
        provider_delete_requested_separately: true,
        notes: owned_text(PARENT_STORAGE_DISCONNECT_NOTE),
    }
}

fn sample_claim_safe_copy() -> Vec<ParentStorageClaimSafeCopyRow> {
    vec![
        copy_row(
            ParentStorageCopyKey::CustodyBoundary,
            PARENT_STORAGE_COPY_CUSTODY_BOUNDARY_STATEMENT,
        ),
        copy_row(
            ParentStorageCopyKey::MetadataLeakage,
            PARENT_STORAGE_COPY_METADATA_LEAKAGE_STATEMENT,
        ),
        copy_row(
            ParentStorageCopyKey::SensitiveEncryptedBeforeUpload,
            PARENT_STORAGE_COPY_SENSITIVE_ENCRYPTED_BEFORE_UPLOAD_STATEMENT,
        ),
        copy_row(
            ParentStorageCopyKey::LostKeyMayBeUnrecoverable,
            PARENT_STORAGE_COPY_LOST_KEY_MAY_BE_UNRECOVERABLE_STATEMENT,
        ),
        copy_row(
            ParentStorageCopyKey::DisconnectDoesNotDelete,
            PARENT_STORAGE_COPY_DISCONNECT_DOES_NOT_DELETE_STATEMENT,
        ),
        copy_row(
            ParentStorageCopyKey::TombstonesMayBeRequired,
            PARENT_STORAGE_COPY_TOMBSTONES_MAY_BE_REQUIRED_STATEMENT,
        ),
        copy_row(
            ParentStorageCopyKey::BackupQueued,
            PARENT_STORAGE_COPY_BACKUP_QUEUED_STATEMENT,
        ),
        copy_row(
            ParentStorageCopyKey::ProviderUploadPending,
            PARENT_STORAGE_COPY_PROVIDER_UPLOAD_PENDING_STATEMENT,
        ),
        copy_row(
            ParentStorageCopyKey::ProviderUploadFailed,
            PARENT_STORAGE_COPY_PROVIDER_UPLOAD_FAILED_STATEMENT,
        ),
        copy_row(
            ParentStorageCopyKey::ProviderUploadConfirmed,
            PARENT_STORAGE_COPY_PROVIDER_UPLOAD_CONFIRMED_STATEMENT,
        ),
        copy_row(
            ParentStorageCopyKey::ImportPreviewPassed,
            PARENT_STORAGE_COPY_IMPORT_PREVIEW_PASSED_STATEMENT,
        ),
        copy_row(
            ParentStorageCopyKey::ApplyRequiresConfirmation,
            PARENT_STORAGE_COPY_APPLY_REQUIRES_CONFIRMATION_STATEMENT,
        ),
        copy_row(
            ParentStorageCopyKey::DeletedLocallyProviderDeletePending,
            PARENT_STORAGE_COPY_DELETED_LOCALLY_PROVIDER_DELETE_PENDING_STATEMENT,
        ),
        copy_row(
            ParentStorageCopyKey::ProviderDisconnectedExistingFilesMayRemain,
            PARENT_STORAGE_COPY_PROVIDER_DISCONNECTED_EXISTING_FILES_MAY_REMAIN_STATEMENT,
        ),
        copy_row(
            ParentStorageCopyKey::ManualProofRequired,
            PARENT_STORAGE_COPY_MANUAL_PROOF_REQUIRED_STATEMENT,
        ),
    ]
}

fn delete_action(
    value: &str,
    action_kind: ParentStorageDeleteActionKind,
    state: ParentOwnedSyncDeleteVisibilityState,
    notes: &str,
) -> ParentStorageDeleteActionRow {
    ParentStorageDeleteActionRow {
        action_id: action_id(value),
        action_kind,
        state,
        separate_from_disconnect: true,
        proof_required: true,
        notes: owned_text(notes),
    }
}

fn copy_row(copy_key: ParentStorageCopyKey, statement: &str) -> ParentStorageClaimSafeCopyRow {
    ParentStorageClaimSafeCopyRow {
        copy_key,
        statement: owned_text(statement),
        forbidden_without_state: true,
        notes: owned_text(PARENT_STORAGE_CLAIM_SAFE_COPY_NOTE),
    }
}
