use serde::{Deserialize, Serialize};

use super::apply_binding_types::{ParentStorageApplyIntentDigest, ParentStorageHouseholdRef};
use super::enums::{
    ParentStorageApplyState, ParentStorageCopyKey, ParentStorageDeleteActionKind,
    ParentStorageEncryptionStatus, ParentStorageKeyStatus, ParentStorageModeLabel,
    ParentStorageNoClaim, ParentStoragePreviewState, ParentStorageUiState,
};
use super::text_types::{
    ParentStorageActionId, ParentStorageApplyId, ParentStorageContractVersion,
    ParentStoragePreviewId, ParentStorageSettingsRowId, ParentStorageTimestamp,
};
use crate::parent_owned_sync_export::{
    ParentOwnedSyncDeleteVisibilityState, ParentOwnedSyncDisconnectVisibilityState,
    ParentOwnedSyncExportDataClass, ParentOwnedSyncProviderMode, ParentOwnedSyncProviderStatus,
    ParentOwnedSyncState,
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentStorageModeCard {
    pub row_id: ParentStorageSettingsRowId,
    pub current_mode_label: ParentStorageModeLabel,
    pub ui_state: ParentStorageUiState,
    pub provider_mode: ParentOwnedSyncProviderMode,
    pub provider_status: ParentOwnedSyncProviderStatus,
    pub sync_state: ParentOwnedSyncState,
    pub encryption_status: ParentStorageEncryptionStatus,
    pub key_status: ParentStorageKeyStatus,
    pub manual_required_visible: bool,
    pub disconnect_visible: bool,
    pub delete_visible: bool,
    pub restore_preview_available: bool,
    pub apply_back_available: bool,
    pub last_success_at: Option<ParentStorageTimestamp>,
    pub last_failure_at: Option<ParentStorageTimestamp>,
    pub summary: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentStorageRestorePreview {
    pub preview_id: ParentStoragePreviewId,
    pub household_ref: ParentStorageHouseholdRef,
    pub preview_state: ParentStoragePreviewState,
    pub created_at: ParentStorageTimestamp,
    pub product_version: String,
    pub schema_version: String,
    pub household_match: bool,
    pub device_match: bool,
    pub data_classes: Vec<ParentOwnedSyncExportDataClass>,
    pub conflicts: Vec<String>,
    pub rejected_sections: Vec<ParentOwnedSyncExportDataClass>,
    pub partial_restore: bool,
    pub confirmation_required: bool,
    pub local_truth_authoritative: bool,
    pub tombstones_preserved: bool,
    pub manual_required_note: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentStorageApplyDecision {
    pub apply_id: ParentStorageApplyId,
    pub apply_intent_digest: ParentStorageApplyIntentDigest,
    pub apply_state: ParentStorageApplyState,
    pub confirmation_required: bool,
    pub will_change: Vec<ParentOwnedSyncExportDataClass>,
    pub will_not_change: Vec<ParentOwnedSyncExportDataClass>,
    pub preserved_tombstones: Vec<ParentOwnedSyncExportDataClass>,
    pub manual_review_required: Vec<String>,
    pub rollback_available: bool,
    pub manual_required_note: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentStorageDeleteActionRow {
    pub action_id: ParentStorageActionId,
    pub action_kind: ParentStorageDeleteActionKind,
    pub state: ParentOwnedSyncDeleteVisibilityState,
    pub separate_from_disconnect: bool,
    pub proof_required: bool,
    pub notes: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentStorageDisconnectRow {
    pub action_id: ParentStorageActionId,
    pub state: ParentOwnedSyncDisconnectVisibilityState,
    pub existing_files_may_remain: bool,
    pub provider_delete_requested_separately: bool,
    pub notes: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentStorageClaimSafeCopyRow {
    pub copy_key: ParentStorageCopyKey,
    pub statement: String,
    pub forbidden_without_state: bool,
    pub notes: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentStorageSettingsApplyFlowContractProof {
    pub schema_version: String,
    pub contract_version: ParentStorageContractVersion,
    pub mode_card: ParentStorageModeCard,
    pub restore_preview: ParentStorageRestorePreview,
    pub apply_decision: ParentStorageApplyDecision,
    pub delete_actions: Vec<ParentStorageDeleteActionRow>,
    pub disconnect_action: ParentStorageDisconnectRow,
    pub claim_safe_copy: Vec<ParentStorageClaimSafeCopyRow>,
    pub no_claims: Vec<ParentStorageNoClaim>,
    pub updated_at: ParentStorageTimestamp,
}
