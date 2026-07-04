mod apply_state;
mod copy_key;
mod delete_action_kind;
mod encryption_status;
mod key_status;
mod mode_label;
mod no_claim;
mod preview_state;
mod ui_state;

pub(super) type ParentStorageApplyState = apply_state::ParentStorageApplyState;
pub(super) type ParentStorageCopyKey = copy_key::ParentStorageCopyKey;
pub(super) type ParentStorageDeleteActionKind = delete_action_kind::ParentStorageDeleteActionKind;
pub(super) type ParentStorageEncryptionStatus = encryption_status::ParentStorageEncryptionStatus;
pub(super) type ParentStorageKeyStatus = key_status::ParentStorageKeyStatus;
pub(super) type ParentStorageModeLabel = mode_label::ParentStorageModeLabel;
pub(super) type ParentStorageNoClaim = no_claim::ParentStorageNoClaim;
pub(super) type ParentStoragePreviewState = preview_state::ParentStoragePreviewState;
pub(super) type ParentStorageUiState = ui_state::ParentStorageUiState;
