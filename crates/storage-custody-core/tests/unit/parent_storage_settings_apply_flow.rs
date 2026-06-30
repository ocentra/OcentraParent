use crate::support::StorageCustodyTestValueExt;

use ocentra_schema::parent_owned_sync_export as sync_contracts;
use ocentra_schema::parent_storage_settings_apply_flow as contracts;
use ocentra_storage_custody_core::parent_storage_settings_apply_flow::{
    build_parent_storage_settings_apply_flow_proof, derive_parent_storage_apply_decision,
    derive_parent_storage_disconnect_row, derive_parent_storage_mode_card,
    derive_parent_storage_restore_preview, ParentStorageApplyDecisionInput,
    ParentStorageDeleteActionInput, ParentStorageDisconnectInput, ParentStorageModeCardInput,
    ParentStorageRestorePreviewInput, ParentStorageSettingsApplyFlowError,
};

#[test]
fn parent_storage_settings_mode_card_keeps_explicit_mode_labels_and_manual_required_visible() {
    let card = derive_parent_storage_mode_card(ParentStorageModeCardInput {
        row_id: row_id("settings-row-manual"),
        provider_mode: sync_contracts::ParentOwnedSyncProviderMode::GoogleDrivePickerFile,
        provider_status: sync_contracts::ParentOwnedSyncProviderStatus::ManualRequired,
        sync_state: sync_contracts::ParentOwnedSyncState::ManualRequired,
        encryption_status: contracts::ParentStorageEncryptionStatus::EncryptedBeforeUpload,
        key_status: contracts::ParentStorageKeyStatus::ManualRequired,
        last_success_at: Some(timestamp("2026-06-28T19:10:00.000Z")),
        last_failure_at: Some(timestamp("2026-06-28T19:12:00.000Z")),
    })
    .value_or_unreachable("mode card");

    assert_eq!(
        card.current_mode_label,
        contracts::ParentStorageModeLabel::ManualRequired
    );
    assert_eq!(
        card.ui_state,
        contracts::ParentStorageUiState::ManualRequired
    );
    assert!(card.manual_required_visible);
}

#[test]
fn parent_storage_settings_restore_preview_stays_preview_before_apply() {
    let preview = derive_parent_storage_restore_preview(preview_input(
        contracts::ParentStoragePreviewState::PartialRestore,
        true,
        true,
        vec![sync_contracts::ParentOwnedSyncExportDataClass::NotificationHistory],
    ))
    .value_or_unreachable("preview");
    assert!(preview.confirmation_required);
    assert!(preview.tombstones_preserved);

    let apply = derive_parent_storage_apply_decision(
        &preview,
        ParentStorageApplyDecisionInput {
            apply_id: apply_id("apply-confirmation"),
            apply_state: contracts::ParentStorageApplyState::ApplyRequiresConfirmation,
            will_change: vec![sync_contracts::ParentOwnedSyncExportDataClass::GeneratedSummary],
            will_not_change: vec![
                sync_contracts::ParentOwnedSyncExportDataClass::NotificationHistory,
            ],
            preserved_tombstones: vec![
                sync_contracts::ParentOwnedSyncExportDataClass::NotificationHistory,
            ],
            manual_review_required: vec!["notification-history tombstone conflict".to_string()],
            rollback_available: false,
            manual_required_note: None,
        },
    )
    .value_or_unreachable("apply decision");
    assert_eq!(
        apply.apply_state,
        contracts::ParentStorageApplyState::ApplyRequiresConfirmation
    );
    assert!(apply.confirmation_required);
}

#[test]
fn parent_storage_settings_disconnect_and_delete_stay_separate() {
    let proof = build_parent_storage_settings_apply_flow_proof(
        ParentStorageModeCardInput {
            row_id: row_id("settings-row-disconnect"),
            provider_mode:
                sync_contracts::ParentOwnedSyncProviderMode::IcloudDriveParentSelectedLocation,
            provider_status: sync_contracts::ParentOwnedSyncProviderStatus::Disconnected,
            sync_state: sync_contracts::ParentOwnedSyncState::NotStarted,
            encryption_status: contracts::ParentStorageEncryptionStatus::EncryptedBeforeUpload,
            key_status: contracts::ParentStorageKeyStatus::KeyAvailable,
            last_success_at: None,
            last_failure_at: Some(timestamp("2026-06-28T19:15:00.000Z")),
        },
        preview_input(
            contracts::ParentStoragePreviewState::ImportPreviewPassed,
            true,
            true,
            Vec::new(),
        ),
        ParentStorageApplyDecisionInput {
            apply_id: apply_id("apply-pending"),
            apply_state: contracts::ParentStorageApplyState::ApplyRequiresConfirmation,
            will_change: vec![sync_contracts::ParentOwnedSyncExportDataClass::GeneratedSummary],
            will_not_change: vec![],
            preserved_tombstones: vec![],
            manual_review_required: vec![],
            rollback_available: false,
            manual_required_note: None,
        },
        delete_inputs(),
        ParentStorageDisconnectInput {
            action_id: action_id("disconnect-provider"),
            state: sync_contracts::ParentOwnedSyncDisconnectVisibilityState::DisconnectVisible,
            notes: "Disconnect stops future sync only.".to_string(),
        },
        timestamp("2026-06-28T19:18:00.000Z"),
    )
    .value_or_unreachable("proof");

    assert!(proof
        .delete_actions
        .iter()
        .all(|row| row.separate_from_disconnect && row.proof_required));
    assert!(proof.disconnect_action.existing_files_may_remain);
    assert!(proof.disconnect_action.provider_delete_requested_separately);
}

#[test]
fn parent_storage_settings_wrong_household_and_partial_restore_states_stay_explicit() {
    let wrong_household = derive_parent_storage_restore_preview(preview_input(
        contracts::ParentStoragePreviewState::WrongHousehold,
        false,
        true,
        Vec::new(),
    ))
    .value_or_unreachable("wrong household preview");
    assert_eq!(
        wrong_household.preview_state,
        contracts::ParentStoragePreviewState::WrongHousehold
    );

    let partial_missing_rejection = derive_parent_storage_restore_preview(preview_input(
        contracts::ParentStoragePreviewState::PartialRestore,
        true,
        true,
        Vec::new(),
    ));
    assert_eq!(
        partial_missing_rejection,
        Err(ParentStorageSettingsApplyFlowError::PartialRestoreMustNameRejectedSections)
    );
}

#[test]
fn parent_storage_settings_manual_required_and_disconnect_rows_require_explicit_visibility() {
    let disconnect = derive_parent_storage_disconnect_row(ParentStorageDisconnectInput {
        action_id: action_id("disconnect-manual"),
        state: sync_contracts::ParentOwnedSyncDisconnectVisibilityState::ManualRequired,
        notes: "Manual re-auth is required.".to_string(),
    })
    .value_or_unreachable("disconnect row");
    assert!(disconnect.provider_delete_requested_separately);

    let blocked_apply = derive_parent_storage_apply_decision(
        &derive_parent_storage_restore_preview(preview_input(
            contracts::ParentStoragePreviewState::ManualRequired,
            true,
            true,
            Vec::new(),
        ))
        .value_or_unreachable("manual preview"),
        ParentStorageApplyDecisionInput {
            apply_id: apply_id("apply-blocked"),
            apply_state: contracts::ParentStorageApplyState::BlockedManualRequired,
            will_change: vec![],
            will_not_change: vec![
                sync_contracts::ParentOwnedSyncExportDataClass::NotificationHistory,
            ],
            preserved_tombstones: vec![
                sync_contracts::ParentOwnedSyncExportDataClass::NotificationHistory,
            ],
            manual_review_required: vec!["provider re-auth required".to_string()],
            rollback_available: false,
            manual_required_note: Some("Provider re-auth must complete first.".to_string()),
        },
    )
    .value_or_unreachable("blocked apply");
    assert_eq!(
        blocked_apply.apply_state,
        contracts::ParentStorageApplyState::BlockedManualRequired
    );
}

fn preview_input(
    preview_state: contracts::ParentStoragePreviewState,
    household_match: bool,
    device_match: bool,
    rejected_sections: Vec<sync_contracts::ParentOwnedSyncExportDataClass>,
) -> ParentStorageRestorePreviewInput {
    ParentStorageRestorePreviewInput {
        preview_id: preview_id("preview"),
        preview_state,
        created_at: timestamp("2026-06-28T19:14:00.000Z"),
        product_version: "2026.06.28".to_string(),
        schema_version: "export-import-backup-recovery-proof".to_string(),
        household_match,
        device_match,
        data_classes: vec![
            sync_contracts::ParentOwnedSyncExportDataClass::EncryptedJournalSegment,
            sync_contracts::ParentOwnedSyncExportDataClass::GeneratedSummary,
            sync_contracts::ParentOwnedSyncExportDataClass::NotificationHistory,
        ],
        conflicts: if rejected_sections.is_empty() {
            Vec::new()
        } else {
            vec!["notification-history tombstone conflict".to_string()]
        },
        rejected_sections,
        partial_restore: preview_state == contracts::ParentStoragePreviewState::PartialRestore,
        manual_required_note: if preview_state
            == contracts::ParentStoragePreviewState::ManualRequired
        {
            Some("Manual review is required.".to_string())
        } else {
            None
        },
    }
}

fn delete_inputs() -> Vec<ParentStorageDeleteActionInput> {
    vec![
        delete_input(
            "delete-local",
            contracts::ParentStorageDeleteActionKind::DeleteLocalChildEvidence,
            sync_contracts::ParentOwnedSyncDeleteVisibilityState::DeleteVisible,
        ),
        delete_input(
            "delete-cache",
            contracts::ParentStorageDeleteActionKind::DeleteParentPortalCache,
            sync_contracts::ParentOwnedSyncDeleteVisibilityState::DeleteVisible,
        ),
        delete_input(
            "delete-report",
            contracts::ParentStorageDeleteActionKind::DeleteGeneratedReport,
            sync_contracts::ParentOwnedSyncDeleteVisibilityState::DeleteVisible,
        ),
        delete_input(
            "delete-provider",
            contracts::ParentStorageDeleteActionKind::DeleteProviderBackupCopy,
            sync_contracts::ParentOwnedSyncDeleteVisibilityState::ManualRequired,
        ),
        delete_input(
            "delete-support",
            contracts::ParentStorageDeleteActionKind::DeleteSupportBundle,
            sync_contracts::ParentOwnedSyncDeleteVisibilityState::DeleteVisible,
        ),
        delete_input(
            "delete-metadata",
            contracts::ParentStorageDeleteActionKind::DeleteOcentraMetadata,
            sync_contracts::ParentOwnedSyncDeleteVisibilityState::DeleteVisible,
        ),
    ]
}

fn delete_input(
    id: &str,
    action_kind: contracts::ParentStorageDeleteActionKind,
    state: sync_contracts::ParentOwnedSyncDeleteVisibilityState,
) -> ParentStorageDeleteActionInput {
    ParentStorageDeleteActionInput {
        action_id: action_id(id),
        action_kind,
        state,
        notes: format!("{} remains separate from disconnect.", action_kind.as_str()),
    }
}

fn row_id(value: &str) -> contracts::ParentStorageSettingsRowId {
    contracts::ParentStorageSettingsRowId::parse(value).value_or_unreachable("row id")
}

fn preview_id(value: &str) -> contracts::ParentStoragePreviewId {
    contracts::ParentStoragePreviewId::parse(value).value_or_unreachable("preview id")
}

fn apply_id(value: &str) -> contracts::ParentStorageApplyId {
    contracts::ParentStorageApplyId::parse(value).value_or_unreachable("apply id")
}

fn action_id(value: &str) -> contracts::ParentStorageActionId {
    contracts::ParentStorageActionId::parse(value).value_or_unreachable("action id")
}

fn timestamp(value: &str) -> contracts::ParentStorageTimestamp {
    contracts::ParentStorageTimestamp::parse(value).value_or_unreachable("timestamp")
}
